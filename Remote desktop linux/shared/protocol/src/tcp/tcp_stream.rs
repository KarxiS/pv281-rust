use std::io;
use tokio::net::TcpListener;

use crate::stream::AsyncStream;
use crate::transport::ServerListener;

/// TCP implementation of ServerListener
pub struct TcpServerListener {
    listener: TcpListener,
}

impl TcpServerListener {
    /// Create a new TcpServerListener wrapping a `TcpListener`.
    pub fn new(listener: TcpListener) -> Self {
        Self { listener }
    }
}

#[async_trait::async_trait]
impl ServerListener for TcpServerListener {
    async fn accept(&mut self) -> io::Result<(Box<dyn AsyncStream>, String)> {
        let (socket, addr) = self.listener.accept().await?;
        Ok((Box::new(socket), addr.to_string()))
    }
}
