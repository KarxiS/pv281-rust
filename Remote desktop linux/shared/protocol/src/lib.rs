// Public modules
pub mod config;
pub mod error;
pub mod packet;
mod quic;
pub mod serialization;
pub mod stream;
pub mod tcp;
pub mod transport;

// Re-export commonly used types for convenience
pub use config::{PeerInfo, ServerConfig, PROTOCOL_VERSION};
pub use error::{ErrorCode, ProtocolError};
pub use packet::{Packet, PacketType};
pub use serialization::{receive, send, SerializationMode};
pub use stream::AsyncStream;
pub use transport::{TransportFactory, TransportType};
