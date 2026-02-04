//! This example demonstrates how to make multiple outgoing connections on a single UDP socket.
//!
//! Checkout the `README.md` for guidance.

use std::{error::Error, io};

use quinn::{Endpoint, RecvStream, SendStream};

use crate::quic::{make_client_endpoint, make_server_endpoint, QuinnStream};
use crate::transport::ServerListener;
use crate::AsyncStream;

pub async fn quic_client_stream(
    addr: &str,
) -> Result<(SendStream, RecvStream), Box<dyn Error + Send + Sync + 'static>> {
    let server_addr = addr.parse().map_err(|e| {
        io::Error::new(io::ErrorKind::InvalidInput, format!("Invalid address: {e}"))
    })?;
    // TODO: use proper addresses
    // First create a temporary server to get the certificate
    let (_temp_endpoint, server_cert) =
        make_server_endpoint("0.0.0.0:0".parse().unwrap()).map_err(io::Error::other)?;

    // Use existing mod module helper function
    let endpoint = make_client_endpoint("0.0.0.0:0".parse().unwrap(), &[server_cert.as_ref()])
        .map_err(io::Error::other)?;

    let connection = endpoint
        .connect(server_addr, "localhost")
        .map_err(io::Error::other)?
        .await
        .map_err(io::Error::other)?;

    let quinn_stream = connection.open_bi().await.map_err(io::Error::other)?;
    Ok(quinn_stream)
}

/// QUIC implementation of ServerListener
pub struct QuinnServerListener {
    endpoint: Endpoint,
}

impl QuinnServerListener {
    /// Create a new QuinnServerListener from a `quinn::Endpoint`.
    pub const fn new(endpoint: Endpoint) -> Self {
        Self { endpoint }
    }
}

#[async_trait::async_trait]
impl ServerListener for QuinnServerListener {
    async fn accept(&mut self) -> io::Result<(Box<dyn AsyncStream>, String)> {
        let connecting =
            self.endpoint.accept().await.ok_or_else(|| {
                io::Error::new(io::ErrorKind::UnexpectedEof, "No more connections")
            })?;
        let remote_addr = connecting.remote_address().to_string();
        let connection = connecting.await.map_err(io::Error::other)?;
        let bi_stream = connection.accept_bi().await.map_err(io::Error::other)?;
        Ok((
            Box::new(QuinnStream::new(bi_stream.0, bi_stream.1)),
            remote_addr,
        ))
    }
}
