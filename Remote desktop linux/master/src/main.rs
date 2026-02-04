use anyhow::Result;
use clap::Parser;
use kmf_driver::driver::{DeviceReader, DriverWriter};
use kmf_middleware::command::parse_command;
use kmf_protocol::config::ServerMessage;
use kmf_protocol::{Packet, TransportFactory, TransportType};
use std::io;
use std::io::{BufRead, Write};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::broadcast;
use tokio::time::sleep;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    mouse: Option<String>,

    #[arg(short, long)]
    keyboard: Option<String>,

    /// Transport type (tcp, quic)
    #[arg(short, long, default_value = "tcp")]
    transport: String,

    /// Bind address for server
    #[arg(short, long, default_value = "0.0.0.0:8081")]
    bind: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    println!("devices:");
    for (path, name, t) in DeviceReader::list_devices() {
        println!("{t:?} | {name} | {}", path.display());
    }

    // Parse transport type
    let transport = TransportType::from_str(&args.transport).unwrap_or_else(|e| {
        eprintln!("Warning: {}. Using TCP.", e);
        TransportType::Tcp
    });

    println!(
        "kmf-master starting with {:?} transport on {}",
        transport, args.bind
    );
    run_master(&args.bind, transport).await?;

    let mouse = args.mouse.map(|m| PathBuf::from_str(&m).unwrap());
    let keyboard = args.keyboard.map(|k| PathBuf::from_str(&k).unwrap());

    let mut devices = Vec::new();

    if let Some(keyboard) = keyboard.map(|k| DeviceReader::open_path(k, true, true).unwrap()) {
        devices.push(keyboard);
    }

    if let Some(mouse) = mouse.map(|m| DeviceReader::open_path(m, false, true).unwrap()) {
        devices.push(mouse);
    }

    let mut reader = DeviceReader::new(devices).unwrap();
    let keys = reader.available_keys().unwrap_or_default();
    let axes = reader.available_axes().unwrap_or_default();

    let mut writer = DriverWriter::new(keys, axes)?;

    loop {
        let events = reader.read_events()?;

        for e in events {
            println!("event: {e:?}");
            writer.simulate_event(e)?;
        }
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
}

/// Starts the server and listens for client connections.
/// # Arguments
///
/// * `bind_addr` - The address to bind to (e.g., "0.0.0.0:8080")
/// * `transport` - The transport type to use (TCP, QUIC, etc.)
///
/// # Returns
///
/// Returns `Ok(())` on success, or an error if binding fails.
pub async fn run_master(bind_addr: &str, transport: TransportType) -> anyhow::Result<()> {
    let mut listener = TransportFactory::bind_server(transport, bind_addr).await?;
    println!(
        "Server listening on {} using {:?} transport",
        bind_addr, transport
    );

    // Use broadcast channel to send server messages to all connected clients
    // Channel capacity of 100 means up to 100 messages can be queued
    let (tx, _rx) = broadcast::channel::<ServerMessage>(100);

    // Shared flag to signal server shutdown
    let shutdown = Arc::new(AtomicBool::new(false));
    let shutdown_clone = shutdown.clone();

    spawn_input_handler(tx.clone(), shutdown_clone);

    loop {
        // Check if shutdown was requested
        if shutdown.load(Ordering::Relaxed) {
            println!("[INFO] Server shutting down...");
            // Send quit to all connected clients
            let _ = tx.send(ServerMessage::Quit);
            // Give clients time to receive the quit message
            sleep(Duration::from_millis(100)).await;
            break;
        }

        // Use timeout to periodically check shutdown flag
        let accept_result =
            tokio::time::timeout(Duration::from_millis(500), listener.accept()).await;

        match accept_result {
            Ok(Ok((socket, addr))) => {
                println!("[INFO] New client connected: {}", addr);
                let rx = tx.subscribe();
                spawn_client_handler(socket, rx);
            }
            Ok(Err(e)) => {
                eprintln!("[ERROR] Failed to accept connection: {}", e);
            }
            Err(_) => {
                // Timeout - just continue loop to check shutdown flag
                continue;
            }
        }
    }

    println!("[INFO] Server stopped.");
    Ok(())
}

/// Spawns a dedicated OS thread for handling command-line input.
///
/// This function **must** run in a separate OS thread (not a tokio task) because:
/// - `stdin().lock()` is blocking I/O
/// - Blocking the tokio runtime would prevent network operations
/// - OS threads allow true parallel execution with the async runtime
///
/// The thread continuously reads commands and broadcasts them to all connected clients.
///
/// # Arguments
///
/// * `tx` - Broadcast sender for distributing commands to client handlers
/// * `shutdown` - Atomic flag to signal server shutdown when quit is entered
pub fn spawn_input_handler(tx: broadcast::Sender<ServerMessage>, shutdown: Arc<AtomicBool>) {
    // IMPORTANT: Use std::thread::spawn, NOT tokio::spawn
    std::thread::spawn(move || {
        let stdin = io::stdin();
        let mut reader = stdin.lock();
        let mut line = String::new();

        loop {
            print!("Enter command (move <x> <y> | click <left|right|middle> <down|up> | key <key> <down|up> | file <path> | quit): ");
            io::stdout().flush().expect("Failed to flush stdout");

            line.clear();
            if reader.read_line(&mut line).is_err() {
                eprintln!("Failed to read input");
                continue;
            }

            let input = line.trim();
            if let Some(message) = parse_command(input) {
                // Check if it's a quit command
                if matches!(message, ServerMessage::Quit) {
                    println!("[INFO] Quit command received. Shutting down server...");
                    shutdown.store(true, Ordering::Relaxed);
                    // Still broadcast quit to connected clients
                    let _ = tx.send(message);
                    break; // Exit the input handler thread
                }

                match tx.send(message) {
                    Ok(n) => println!("[INFO] Command broadcast to {} client(s)", n),
                    Err(_) => println!("[WARN] No clients connected"),
                }
            } else if !input.is_empty() {
                eprintln!("Unknown command");
            }
        }
    });
}

/// Spawns an async task to handle a connected client.
/// # Arguments
///
/// * `socket` - The stream for this client connection
/// * `rx` - Broadcast receiver subscribed to server messages
///
/// # Note
///
/// This function spawns a tokio task that runs until:
/// - The client disconnects
/// - An error occurs during send/receive
/// - A Quit message is broadcast
pub fn spawn_client_handler(
    mut socket: Box<dyn kmf_protocol::AsyncStream>,
    mut rx: broadcast::Receiver<ServerMessage>,
) {
    tokio::spawn(async move {
        // Wait for client's ServerHello - this is the handshake protocol
        match kmf_protocol::receive(&mut socket).await {
            Ok(Packet::ServerHello(config)) => {
                println!("[INFO] Client config: {:?}", config);
            }
            Ok(other) => {
                eprintln!("[ERROR] Expected ServerHello, got {:?}", other);
                return;
            }
            Err(e) => {
                eprintln!("[ERROR] Failed to receive ServerHello: {}", e);
                return;
            }
        }

        // Main message handling loop
        loop {
            match rx.recv().await {
                Ok(ServerMessage::Action(action)) => {
                    println!("[DEBUG] Broadcasting action to client");
                    if let Err(e) = kmf_protocol::send(Packet::Action(action), &mut socket).await {
                        eprintln!("[ERROR] Failed to send action to client: {}", e);
                        break;
                    }

                    // Wait for acknowledgment - ensures client received and processed action
                    match kmf_protocol::receive(&mut socket).await {
                        Ok(Packet::Ok) => {
                            println!("[DEBUG] Client acknowledged action")
                        }
                        Ok(Packet::Err { code, message }) => {
                            eprintln!("[ERROR] Client error (code {}): {}", code, message);
                        }
                        Err(e) => {
                            eprintln!("[ERROR] Failed to receive ack: {}", e);
                            break;
                        }
                        _ => {}
                    }
                }
                Ok(ServerMessage::File { path }) => {
                    println!("[DEBUG] Broadcasting file to client");
                    match kmf_middleware::file_transfer::send_file(&mut socket, &path).await {
                        Ok(_) => println!("[INFO] File sent"),
                        Err(e) => {
                            eprintln!("[ERROR] Failed to send file: {}", e);
                            break;
                        }
                    }
                }
                Ok(ServerMessage::Quit) => {
                    println!("[INFO] Sending quit to client");
                    let _ = kmf_protocol::send(Packet::ClientQuit, &mut socket).await;
                    break;
                }
                Err(e) => {
                    eprintln!("[ERROR] Broadcast receive error: {}", e);
                    break;
                }
            }
        }

        println!("[INFO] Client disconnected");
    });
}

/// Sends a file to a client using the two-protocol protocol.
#[allow(dead_code)]
async fn send_file(
    socket: &mut Box<dyn kmf_protocol::AsyncStream>,
    path: &str,
) -> anyhow::Result<()> {
    let file_path = Path::new(path);
    let filename = file_path
        .file_name()
        .ok_or_else(|| anyhow::anyhow!("Invalid filename"))?
        .to_string_lossy()
        .to_string();

    let data = read_file(path).await?;

    kmf_protocol::send(
        Packet::DropSend {
            filename: filename.clone(),
        },
        socket,
    )
    .await?;

    kmf_protocol::send(Packet::Data(data.clone()), socket).await?;

    println!("[DEBUG] File sent: {} ({} bytes)", filename, data.len());

    match kmf_protocol::receive(socket).await {
        Ok(Packet::Ok) => {
            println!("[DEBUG] File transfer acknowledged");
            Ok(())
        }
        Ok(Packet::Err {
            code: _,
            message: _,
        }) => Err(anyhow::anyhow!("Client error")),
        Err(e) => Err(anyhow::anyhow!("Failed to receive ack: {}", e)),
        _ => Err(anyhow::anyhow!("Unexpected response")),
    }
}

/// Reads an entire file into memory asynchronously.
async fn read_file(path: &str) -> anyhow::Result<Vec<u8>> {
    Ok(kmf_middleware::file_transfer::read_file(path).await?)
}
