use crate::quic::make_server_endpoint;
use crate::quic::quic_single_socket;
use crate::quic::QuinnStream;
use crate::stream::AsyncStream;
use std::io;
use tokio::net::{TcpListener, TcpStream};

/// Transport type configuration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TransportType {
    #[default]
    Tcp,
    Quic,
}

impl std::str::FromStr for TransportType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "tcp" => Ok(TransportType::Tcp),
            "quic" => Ok(TransportType::Quic),
            _ => Err(format!(
                "Unknown transport type: '{}'. Use 'tcp' or 'quic'",
                s
            )),
        }
    }
}

/// Transport factory for creating connections
pub struct TransportFactory;

impl TransportFactory {
    /// Create a server listener for the specified transport type
    pub async fn bind_server(
        transport: TransportType,
        addr: &str,
    ) -> io::Result<Box<dyn ServerListener>> {
        match transport {
            TransportType::Tcp => {
                let listener = TcpListener::bind(addr).await?;
                // use tcp module's public TcpServerListener
                Ok(Box::new(crate::tcp::TcpServerListener::new(listener)))
            }
            TransportType::Quic => {
                let socket_addr = addr.parse().map_err(|e| {
                    io::Error::new(
                        io::ErrorKind::InvalidInput,
                        format!("Invalid address: {}", e),
                    )
                })?;

                let (endpoint, _server_cert) = make_server_endpoint(socket_addr)
                    .map_err(|e| io::Error::other(format!("{}", e)))?;

                Ok(Box::new(
                    crate::quic::quic_single_socket::QuinnServerListener::new(endpoint),
                ))
            }
        }
    }

    /// Create a client connection for the specified transport type
    pub async fn connect_client(
        transport: TransportType,
        addr: &str,
    ) -> io::Result<Box<dyn AsyncStream>> {
        match transport {
            TransportType::Tcp => {
                let stream = TcpStream::connect(addr).await?;
                Ok(Box::new(stream))
            }
            TransportType::Quic => {
                // reuse existing helper implemented in quic_single_socket module
                let quinn_stream = quic_single_socket::quic_client_stream(addr)
                    .await
                    .map_err(|e| io::Error::other(format!("{}", e)))?;

                Ok(Box::new(QuinnStream::new(quinn_stream.0, quinn_stream.1)))
            }
        }
    }
}

/// Trait for server listeners (abstracts TcpListener, QuicListener, etc.)
#[async_trait::async_trait]
pub trait ServerListener: Send {
    /// Accept a new connection
    async fn accept(&mut self) -> io::Result<(Box<dyn AsyncStream>, String)>;
}
