use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Protocol version for compatibility checks
/// Used in the ServerHello
pub const PROTOCOL_VERSION: u32 = 1;

/// Protocol structure constants
pub mod protocol_structure {
    pub const PACKET_TYPE_SIZE: usize = 1;
    pub const ERROR_CODE_SIZE: usize = 1;
    pub const LENGTH_PREFIX_SIZE: usize = 4;
    pub const MIN_ERROR_PACKET_SIZE: usize =
        PACKET_TYPE_SIZE + ERROR_CODE_SIZE + LENGTH_PREFIX_SIZE;
    pub const MIN_PAYLOAD_PACKET_SIZE: usize = PACKET_TYPE_SIZE + LENGTH_PREFIX_SIZE;
    pub const ERROR_CODE_OFFSET: usize = PACKET_TYPE_SIZE;
    pub const ERROR_LENGTH_OFFSET: usize = PACKET_TYPE_SIZE + ERROR_CODE_SIZE;
    pub const PAYLOAD_LENGTH_OFFSET: usize = PACKET_TYPE_SIZE;
}

/// Client configuration sent during the ServerHello handshake
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    /// Protocol version for compatibility
    pub version: u32,
    /// Screen width in pixels
    pub screen_width: u32,
    /// Screen height in pixels
    pub screen_height: u32,
    /// Client hostname for identification
    pub hostname: String,
}

/// Information about a connected peer
#[derive(Debug, Clone)]
pub struct PeerInfo {
    /// IP address of the peer
    pub ip: String,
    /// Hostname of the peer
    pub hostname: String,
    /// Screen width in pixels
    pub screen_width: u32,
    /// Screen height in pixels
    pub screen_height: u32,
}

impl PeerInfo {
    pub fn new(ip: String, config: ServerConfig) -> Self {
        Self {
            ip,
            hostname: config.hostname,
            screen_width: config.screen_width,
            screen_height: config.screen_height,
        }
    }
}

/// Messages broadcast from master to all connected slaves
///
/// The master uses a broadcast channel to send these messages to all slave handlers.
/// Each slave handler then translates these into protocol packets.
#[derive(Clone, Debug)]
pub enum ServerMessage {
    /// An action to be executed on slaves
    Action(Value),
    /// A file to be transferred to slaves
    File { path: String },
    /// Signal to disconnect all slaves gracefully
    Quit,
}
