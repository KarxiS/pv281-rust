use std::collections::HashMap;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::sync::{broadcast, oneshot};
use tokio::task::JoinHandle;

use kmf_driver::driver::{DeviceReader, DriverWriter};
use kmf_middleware::file_transfer;
use kmf_protocol::config::ServerMessage;
use kmf_protocol::{Packet, TransportFactory, TransportType};

use crate::driver_loop::DriverLoopContext;
use crate::status::MasterStatus;
pub use crate::status::MasterStatusSnapshot;

#[derive(Clone, Debug, serde::Serialize)]
pub struct ConnectedClientInfo {
    pub id: String,
    pub hostname: String,
    pub ip: String,
    pub status: String,
}

pub struct MasterService {
    running: Arc<AtomicBool>,
    handle: Mutex<Option<JoinHandle<()>>>,
    network_handle: Mutex<Option<JoinHandle<()>>>,
    tx: Mutex<Option<broadcast::Sender<ServerMessage>>>,
    status: Arc<Mutex<MasterStatus>>,
    clients: Arc<Mutex<Vec<ConnectedClientInfo>>>,
    client_stoppers: Arc<Mutex<HashMap<String, oneshot::Sender<()>>>>,
}

impl Default for MasterService {
    fn default() -> Self {
        Self::new()
    }
}

impl MasterService {
    pub fn new() -> Self {
        Self {
            running: Arc::new(AtomicBool::new(false)),
            handle: Mutex::new(None),
            network_handle: Mutex::new(None),
            tx: Mutex::new(None),
            status: Arc::new(Mutex::new(MasterStatus::default())),
            clients: Arc::new(Mutex::new(Vec::new())),
            client_stoppers: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn get_status(&self) -> MasterStatusSnapshot {
        let status = self.status.lock().expect("Failed to lock status");
        MasterStatusSnapshot::from(&*status)
    }

    pub fn get_clients(&self) -> Vec<ConnectedClientInfo> {
        self.clients.lock().expect("Failed to lock clients").clone()
    }

    pub fn disconnect_client(&self, id: &str) {
        let mut stoppers = self
            .client_stoppers
            .lock()
            .expect("Failed to lock stoppers");
        if let Some(tx) = stoppers.remove(id) {
            let _ = tx.send(());
        }
    }

    pub fn start(&self, mouse: Option<String>, keyboard: Option<String>) -> Result<(), String> {
        if self.running.load(Ordering::SeqCst) {
            return Err("Master is already running".to_string());
        }

        eprintln!(
            "[MasterService] Starting with mouse={:?}, keyboard={:?}",
            mouse, keyboard
        );
        let (reader, writer) = init_driver(mouse, keyboard)?;

        let running = self.running.clone();
        running.store(true, Ordering::SeqCst);

        let (tx, _rx) = broadcast::channel::<ServerMessage>(100);
        *self.tx.lock().expect("Failed to lock tx") = Some(tx.clone());

        {
            let mut status = self.status.lock().expect("Failed to lock status");
            status.reset();
        }

        let h = Self::spawn_driver_loop(
            running.clone(),
            reader,
            writer,
            tx.clone(),
            self.status.clone(),
        );
        *self.handle.lock().expect("Failed to lock handle") = Some(h);

        let net_h = Self::spawn_network_loop(
            running.clone(),
            tx.clone(),
            self.clients.clone(),
            self.client_stoppers.clone(),
        );
        *self
            .network_handle
            .lock()
            .expect("Failed to lock network handle") = Some(net_h);

        Ok(())
    }

    pub fn send_file(&self, path: String) -> Result<(), String> {
        if !self.running.load(Ordering::SeqCst) {
            return Err("Master is not running".to_string());
        }

        let tx = self.tx.lock().expect("Failed to lock tx");
        let Some(sender) = tx.as_ref() else {
            return Err("Master is not ready".to_string());
        };

        sender
            .send(ServerMessage::File { path })
            .map_err(|_| "No clients connected".to_string())?;
        Ok(())
    }

    fn spawn_driver_loop(
        running: Arc<AtomicBool>,
        mut reader: DeviceReader,
        writer: DriverWriter,
        tx: broadcast::Sender<ServerMessage>,
        status_mutex: Arc<Mutex<MasterStatus>>,
    ) -> JoinHandle<()> {
        tokio::spawn(async move {
            eprintln!("MasterService: Local Driver Loop started");

            let (width, height) = {
                let s = status_mutex.lock().expect("Failed to lock status");
                (s.master_width, s.master_height)
            };

            let mut ctx = DriverLoopContext::new(
                writer,
                tx,
                status_mutex.clone(),
                running.clone(),
                width,
                height,
            );

            println!("[CAL] Calibration started: Move mouse to bottom-right and press 'c'.");

            while running.load(Ordering::SeqCst) {
                if let Ok(events) = reader.read_events() {
                    for event in events {
                        if !ctx.handle_event(event, &mut reader) {
                            break;
                        }
                    }
                }
                tokio::time::sleep(Duration::from_millis(10)).await;
            }

            if ctx.inputs_grabbed {
                let _ = reader.ungrab_inputs();
            }
            {
                let mut status = status_mutex.lock().expect("Failed to lock status");
                status.running = false;
                status.remote_mode = false;
            }
            eprintln!("MasterService: Local Driver Loop stopped");
        })
    }

    fn spawn_network_loop(
        running: Arc<AtomicBool>,
        tx_for_network: broadcast::Sender<ServerMessage>,
        clients: Arc<Mutex<Vec<ConnectedClientInfo>>>,
        stoppers: Arc<Mutex<HashMap<String, oneshot::Sender<()>>>>,
    ) -> JoinHandle<()> {
        tokio::spawn(async move {
            let bind_addr = "0.0.0.0:8081";
            let transport = TransportType::Tcp;

            match TransportFactory::bind_server(transport, bind_addr).await {
                Ok(mut listener) => {
                    println!("MasterService (Net): Listening on {}", bind_addr);

                    while running.load(Ordering::SeqCst) {
                        if let Ok(Ok((socket, addr))) =
                            tokio::time::timeout(Duration::from_millis(500), listener.accept())
                                .await
                        {
                            println!("New client connected: {}", addr);
                            let rx = tx_for_network.subscribe();
                            let (stop_tx, stop_rx) = oneshot::channel();

                            {
                                let mut guard = stoppers.lock().expect("Failed to lock stoppers");
                                guard.insert(addr.clone(), stop_tx);
                            }

                            spawn_client_handler(
                                socket,
                                rx,
                                addr,
                                clients.clone(),
                                stoppers.clone(),
                                stop_rx,
                            );
                        }
                    }
                    let _ = tx_for_network.send(ServerMessage::Quit);
                }
                Err(e) => {
                    eprintln!("Failed to bind server: {}", e);
                }
            }
        })
    }

    pub fn stop(&self) {
        if self.running.load(Ordering::SeqCst) {
            println!("MasterService: Stopping...");
            self.running.store(false, Ordering::SeqCst);
            let mut status = self.status.lock().expect("Failed to lock status");
            status.running = false;
            status.remote_mode = false;

            *self.tx.lock().expect("Failed to lock tx") = None;

            let mut stoppers = self
                .client_stoppers
                .lock()
                .expect("Failed to lock stoppers");
            for (_, tx) in stoppers.drain() {
                let _ = tx.send(());
            }
        }
    }

    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }
}

fn init_driver(
    mouse: Option<String>,
    keyboard: Option<String>,
) -> Result<(DeviceReader, DriverWriter), String> {
    let mut devices = Vec::new();

    if let Some(path_str) = keyboard {
        let path = PathBuf::from_str(&path_str).map_err(|e| e.to_string())?;
        let device = DeviceReader::open_path(path, false, true)
            .map_err(|e| format!("Failed to open keyboard: {}", e))?;
        devices.push(device);
    }

    if let Some(path_str) = mouse {
        let path = PathBuf::from_str(&path_str).map_err(|e| e.to_string())?;
        let device = DeviceReader::open_path(path, false, true)
            .map_err(|e| format!("Failed to open mouse: {}", e))?;
        devices.push(device);
    }

    let reader =
        DeviceReader::new(devices).map_err(|e| format!("Failed to init DriverReader: {}", e))?;

    let keys = reader.available_keys().unwrap_or_default();
    let axes = reader.available_axes().unwrap_or_default();

    let writer =
        DriverWriter::new(keys, axes).map_err(|e| format!("Failed to init DriverWriter: {}", e))?;

    Ok((reader, writer))
}

fn spawn_client_handler(
    mut socket: Box<dyn kmf_protocol::AsyncStream>,
    mut rx: broadcast::Receiver<ServerMessage>,
    addr: String,
    clients: Arc<Mutex<Vec<ConnectedClientInfo>>>,
    stoppers: Arc<Mutex<HashMap<String, oneshot::Sender<()>>>>,
    mut stop_rx: oneshot::Receiver<()>,
) {
    tokio::spawn(async move {
        match kmf_protocol::receive(&mut socket).await {
            Ok(Packet::ServerHello(config)) => {
                println!("[INFO] Client config: {:?}", config);

                let client_id = addr.clone();
                {
                    let mut guard = clients.lock().expect("Failed to lock clients");
                    guard.push(ConnectedClientInfo {
                        id: client_id.clone(),
                        hostname: config.hostname.clone(),
                        ip: addr.clone(),
                        status: "online".into(),
                    });
                }

                loop {
                    tokio::select! {
                        _ = &mut stop_rx => {
                            println!("Client {} disconnect requested by master", client_id);
                            let _ = kmf_protocol::send(Packet::ClientQuit, &mut socket).await;
                            break;
                        }
                        msg = rx.recv() => {
                            match msg {
                                Ok(ServerMessage::Action(action)) => {
                                    if let Err(e) = kmf_protocol::send(Packet::Action(action), &mut socket).await {
                                        eprintln!("[ERROR] Send action failed: {}", e);
                                        break;
                                    }
                                    let _ = kmf_protocol::receive(&mut socket).await;
                                }
                                Ok(ServerMessage::File { path }) => {
                                    println!("[Master] Sending file to {}: {}", client_id, path);
                                    if let Err(e) = file_transfer::send_file(&mut socket, &path).await {
                                        eprintln!("[ERROR] Send file failed: {}", e);
                                        break;
                                    }
                                }
                                Ok(ServerMessage::Quit) => {
                                    let _ = kmf_protocol::send(Packet::ClientQuit, &mut socket).await;
                                    break;
                                }
                                _ => {}
                            }
                        }
                    }
                }

                println!("Client disconnected: {}", client_id);
                {
                    let mut guard = clients.lock().expect("Failed to lock clients");
                    if let Some(pos) = guard.iter().position(|c| c.id == client_id) {
                        guard.remove(pos);
                    }
                }
                {
                    let mut guard = stoppers.lock().expect("Failed to lock stoppers");
                    guard.remove(&client_id);
                }
            }
            Ok(other) => {
                eprintln!("[ERROR] Expected ServerHello, got {:?}", other);
            }
            Err(e) => {
                eprintln!("[ERROR] Failed to receive ServerHello: {}", e);
            }
        }
    });
}
