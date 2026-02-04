use clap::Parser;
use kmf_driver::driver::{DriverEvent, DriverWriter};
use kmf_middleware::command::parse_command;
use kmf_protocol::config::ServerMessage;
use kmf_protocol::{ErrorCode, Packet, ServerConfig, TransportFactory, TransportType};
use std::io;
use std::io::{BufRead, Write};
use std::str::FromStr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::sync::broadcast;

#[cfg(target_os = "linux")]
use evdev::{KeyCode, RelativeAxisCode};
#[cfg(not(target_os = "linux"))]
use kmf_driver::driver::{KeyCode, RelativeAxisCode};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Server address to connect to
    #[arg(short, long)]
    server: String,

    /// Transport type (tcp, mod)
    #[arg(short, long, default_value = "mod")]
    transport: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    // Parse transport type
    let transport = TransportType::from_str(&args.transport).unwrap_or(TransportType::Tcp);

    run_client(&args.server, transport).await?;
    Ok(())
}

/// Runs the client and connects to a server.
pub async fn run_client(server_addr: &str, transport: TransportType) -> anyhow::Result<()> {
    println!("Attempting to connect to server at {}...", server_addr);

    let mut stream = match TransportFactory::connect_client(transport, server_addr).await {
        Ok(stream) => {
            println!("Successfully connected to server at {}", server_addr);
            stream
        }
        Err(e) => {
            eprintln!("\n--- Connection Failed ---");
            eprintln!("Error: {}", e);
            eprintln!("\nTroubleshooting steps:");
            eprintln!("1. Is the server running?");
            eprintln!("2. Is the IP address '{}' correct?", server_addr);
            eprintln!("3. Is a firewall blocking the connection? Try temporarily disabling your firewall.");
            eprintln!("   (Check Windows Defender Firewall or any third-party antivirus software)");
            return Ok(());
        }
    };

    // Send ServerHello handshake with client configuration
    // This allows the server to know the client's screen dimensions and hostname
    let config = ServerConfig {
        version: 1,
        screen_width: 1920,
        screen_height: 1080,
        hostname: hostname::get()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string(),
    };

    if let Err(e) = kmf_protocol::send(Packet::ServerHello(config), &mut stream).await {
        eprintln!("Failed to send ServerHello: {}", e);
        return Ok(());
    }

    println!("[INFO] ServerHello sent, waiting for messages...");

    // Initialize DriverWriter
    // We need to tell uinput which keys and axes this virtual device supports.
    let axes = vec![
        RelativeAxisCode::REL_X,
        RelativeAxisCode::REL_Y,
        RelativeAxisCode::REL_WHEEL,
    ];

    let keys = (0..0x2ff).map(KeyCode::new).collect::<Vec<KeyCode>>();

    let mut writer = DriverWriter::new(keys, axes)
        .map_err(|e| anyhow::anyhow!("Failed to init DriverWriter: {}", e))?;

    // Force cursor to top-left on slave (best-effort)
    let _ = writer.simulate_event(DriverEvent::MouseMove(kmf_driver::event::MouseMove {
        x: -10000,
        y: -10000,
        wheel: 0,
    }));
    println!("[INFO] Forced slave cursor to top-left (delta -10000,-10000)");

    // Main client receive loop
    loop {
        match kmf_protocol::receive(&mut stream).await {
            Ok(packet) => {
                println!("[DEBUG] Received protocol: {:?}", packet);

                let should_quit = handle_packet(&mut stream, packet, &mut writer)
                    .await
                    .unwrap();
                if should_quit {
                    break;
                }
            }
            Err(e) => {
                eprintln!("[ERROR] Failed to receive protocol: {}. Disconnecting.", e);
                break;
            }
        }
    }

    println!("Disconnected from server.");
    Ok(())
}

/// Handles an incoming packet on the slave side.
async fn handle_packet(
    stream: &mut Box<dyn kmf_protocol::AsyncStream>,
    packet: Packet,
    writer: &mut DriverWriter,
) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
    match packet {
        Packet::Action(action_val) => {
            // println!("[ACTION] Received: {:?}", action_val);
            if let Ok(event) = serde_json::from_value::<DriverEvent>(action_val) {
                // println!("[ACTION] Simulating: {:?}", event);
                if let Err(e) = writer.simulate_event(event) {
                    eprintln!("[ERROR] Simulation failed: {}", e);
                }
            } else {
                eprintln!("[ERROR] Failed to deserialize DriverEvent from Action");
            }

            kmf_protocol::send(Packet::Ok, stream).await?;
            Ok(false)
        }
        Packet::DropSend { filename } => {
            println!("[DROP] Receiving file: {}", filename);
            if let Err(e) = kmf_middleware::file_transfer::receive_file(stream, &filename).await {
                eprintln!("[ERROR] Failed to save file: {}", e);
                let _ = kmf_protocol::send(
                    Packet::Err {
                        code: ErrorCode::Internal,
                        message: ":(".to_string(),
                    },
                    stream,
                )
                .await;
            }
            Ok(false)
        }
        Packet::DropRequest { filename } => {
            println!("[DROP] Server requesting file: {}", filename);
            match kmf_middleware::file_transfer::read_file(&filename).await {
                Ok(data) => {
                    kmf_protocol::send(Packet::Data(data), stream).await?;
                    println!("[FILE] Sent: {}", filename);
                }
                Err(e) => {
                    eprintln!("[ERROR] Failed to read file: {}", e);
                    let _ = kmf_protocol::send(
                        Packet::Err {
                            code: ErrorCode::Internal,
                            message: ":(".to_string(),
                        },
                        stream,
                    )
                    .await;
                }
            }
            Ok(false)
        }
        Packet::EdgeL => {
            println!("[EDGE] Cursor left edge detected");
            let _ = kmf_protocol::send(Packet::Ok, stream).await;
            Ok(false)
        }
        Packet::EdgeR => {
            println!("[EDGE] Cursor right edge detected");
            let _ = kmf_protocol::send(Packet::Ok, stream).await;
            Ok(false)
        }
        Packet::ClientQuit => {
            println!("[INFO] Server requested disconnect");
            Ok(true)
        }
        Packet::Ok => {
            println!("[OK] Acknowledgment received");
            Ok(false)
        }
        Packet::Err { code, message } => {
            eprintln!("[ERROR] Error code {} Server error: {}", code, message);
            Ok(false)
        }
        _ => {
            println!("[WARN] Unhandled packet type");
            Ok(false)
        }
    }
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
