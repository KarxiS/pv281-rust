#[cfg(target_os = "linux")]
use evdev::{KeyCode, RelativeAxisCode};
use kmf_driver::driver::DriverWriter;
#[cfg(not(target_os = "linux"))]
use kmf_driver::driver::{KeyCode, RelativeAxisCode};
use kmf_protocol::config::ServerConfig;
use kmf_protocol::{ErrorCode, Packet, TransportFactory, TransportType};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use tokio::task::JoinHandle;

pub struct SlaveService {
    running: Arc<AtomicBool>,
    handle: Mutex<Option<JoinHandle<()>>>,
    status: Arc<Mutex<SlaveStatus>>,
}

#[derive(Clone, Debug)]
pub struct SlaveStatusSnapshot {
    pub running: bool,
    pub connecting: bool,
    pub connected: bool,
    pub last_error: Option<String>,
}

#[derive(Debug)]
struct SlaveStatus {
    running: bool,
    connecting: bool,
    connected: bool,
    last_error: Option<String>,
}

impl Default for SlaveService {
    fn default() -> Self {
        Self::new()
    }
}

impl SlaveService {
    pub fn new() -> Self {
        Self {
            running: Arc::new(AtomicBool::new(false)),
            handle: Mutex::new(None),
            status: Arc::new(Mutex::new(SlaveStatus {
                running: false,
                connecting: false,
                connected: false,
                last_error: None,
            })),
        }
    }

    pub fn get_status(&self) -> SlaveStatusSnapshot {
        let status = self.status.lock().unwrap();
        SlaveStatusSnapshot {
            running: status.running,
            connecting: status.connecting,
            connected: status.connected,
            last_error: status.last_error.clone(),
        }
    }

    pub fn start(&self, server_ip: String) -> Result<(), String> {
        if self.running.load(Ordering::SeqCst) {
            return Err("Slave is already running".to_string());
        }

        self.running.store(true, Ordering::SeqCst);
        {
            let mut status = self.status.lock().unwrap();
            status.running = true;
            status.connecting = true;
            status.connected = false;
            status.last_error = None;
        }
        let running_flag = self.running.clone();
        let status_flag = self.status.clone();

        let h = tokio::spawn(async move {
            let transport = TransportType::Tcp;

            if let Err(e) =
                run_client_internal(server_ip, transport, running_flag, status_flag.clone()).await
            {
                eprintln!("Slave connection error: {}", e);
                let mut status = status_flag.lock().unwrap();
                status.last_error = Some(e.to_string());
                status.connected = false;
                status.connecting = false;
            }

            let mut status = status_flag.lock().unwrap();
            status.running = false;
            status.connected = false;
            status.connecting = false;
        });

        *self.handle.lock().unwrap() = Some(h);
        Ok(())
    }

    #[allow(dead_code)]
    pub fn stop(&self) {
        if self.running.load(Ordering::SeqCst) {
            self.running.store(false, Ordering::SeqCst);
            if let Some(handle) = self.handle.lock().unwrap().take() {
                handle.abort();
            }
            let mut status = self.status.lock().unwrap();
            status.running = false;
            status.connected = false;
            status.connecting = false;
        }
    }

    #[allow(dead_code)]
    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }
}

async fn run_client_internal(
    server_addr: String,
    transport: TransportType,
    running: Arc<AtomicBool>,
    status: Arc<Mutex<SlaveStatus>>,
) -> anyhow::Result<()> {
    let mut stream = match TransportFactory::connect_client(transport, &server_addr).await {
        Ok(stream) => stream,
        Err(e) => {
            let mut status = status.lock().unwrap();
            status.connecting = false;
            status.connected = false;
            return Err(anyhow::anyhow!("Connection Failed: {}", e));
        }
    };

    {
        let mut status = status.lock().unwrap();
        status.connecting = false;
        status.connected = true;
    }

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
        return Err(anyhow::anyhow!("Handshake failed: {}", e));
    }

    let axes = vec![
        RelativeAxisCode::REL_X,
        RelativeAxisCode::REL_Y,
        RelativeAxisCode::REL_WHEEL,
    ];

    let keys = (0..0x2ff).map(KeyCode::new).collect::<Vec<KeyCode>>();

    let mut writer = DriverWriter::new(keys, axes)
        .map_err(|e| anyhow::anyhow!("Failed to init DriverWriter: {}", e))?;

    // Reset cursor position
    let _ = writer.simulate_event(kmf_driver::driver::DriverEvent::MouseMove(
        kmf_driver::event::MouseMove {
            x: -10000,
            y: -10000,
            wheel: 0,
        },
    ));

    while running.load(Ordering::SeqCst) {
        if let Ok(Ok(packet)) = tokio::time::timeout(
            std::time::Duration::from_millis(200),
            kmf_protocol::receive(&mut stream),
        )
        .await
        {
            let should_quit = handle_packet(&mut stream, packet, &mut writer).await?;
            if should_quit {
                break;
            }
        }
    }

    {
        let mut status = status.lock().unwrap();
        status.connected = false;
        status.connecting = false;
    }
    Ok(())
}

async fn handle_packet(
    stream: &mut Box<dyn kmf_protocol::AsyncStream>,
    packet: Packet,
    writer: &mut DriverWriter,
) -> Result<bool, anyhow::Error> {
    match packet {
        Packet::Action(action) => {
            if let Ok(event) = kmf_middleware::event::action_to_driver_event(action) {
                if let Err(e) = writer.simulate_event(event) {
                    eprintln!("Input simulation failed: {}", e);
                }
            }
            kmf_protocol::send(Packet::Ok, stream).await?;
            Ok(false)
        }
        Packet::DropSend { filename } => {
            println!("Receiving file: {}", filename);
            if let Err(e) = kmf_middleware::file_transfer::receive_file(stream, &filename).await {
                eprintln!("File receive failed: {}", e);
                let _ = kmf_protocol::send(
                    Packet::Err {
                        code: ErrorCode::Internal,
                        message: "File transfer failed".to_string(),
                    },
                    stream,
                )
                .await;
            }
            Ok(false)
        }
        Packet::DropRequest { filename } => {
            println!("Sending file: {}", filename);
            match kmf_middleware::file_transfer::read_file(&filename).await {
                Ok(data) => {
                    kmf_protocol::send(Packet::Data(data), stream).await?;
                }
                Err(e) => {
                    eprintln!("Failed to read file: {}", e);
                    let _ = kmf_protocol::send(
                        Packet::Err {
                            code: ErrorCode::Internal,
                            message: "File read failed".to_string(),
                        },
                        stream,
                    )
                    .await;
                }
            }
            Ok(false)
        }
        Packet::EdgeL | Packet::EdgeR => {
            let _ = kmf_protocol::send(Packet::Ok, stream).await;
            Ok(false)
        }
        Packet::ClientQuit => Ok(true),
        Packet::Ok => Ok(false),
        Packet::Err { code, message } => {
            eprintln!("Server error ({}): {}", code, message);
            Ok(false)
        }
        _ => Ok(false),
    }
}
