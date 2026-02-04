use crate::error::{ErrorCode, ProtocolError};
use crate::stream::AsyncStream;
use crate::{Packet, PacketType};

use dotenvy::dotenv;
use rmp_serde as rmps;
use serde::{Deserialize, Serialize};
use std::env;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

/// Sends a packet over a AsyncStream stream
///
/// # Arguments
///
/// * `packet` - The packet to send
/// * `stream` - The AsyncStream stream to send on
///
/// # Returns
///
/// - `Ok(())` if the packet was sent successfully
/// - `Err(ProtocolError)` if writing or flushing failed
pub async fn send<S: AsyncStream>(packet: Packet, stream: &mut S) -> Result<(), ProtocolError> {
    let data = packet.serialize();
    stream.write_all(&data).await?;
    stream.flush().await?; // CRITICAL: Flush immediately for real-time communication
    Ok(())
}

/// Receives a packet from an AsyncStream stream
///
/// This is a **blocking** operation that waits for a complete packet to arrive
///
/// # Arguments
///
/// * `stream` - The TCP stream to receive from
///
/// # Returns
///
/// - `Ok(Packet)` if a valid packet was received
/// - `Err(ProtocolError)` if reading failed or the packet was invalid
pub async fn receive<S: AsyncStream>(stream: &mut S) -> Result<Packet, ProtocolError> {
    let mode = SerializationMode::from_env();
    let packet_type = read_packet_type(stream).await?;

    match packet_type {
        // Handle simple packets (no additional data)
        PacketType::Ok => Ok(Packet::Ok),
        PacketType::ClientQuit => Ok(Packet::ClientQuit),
        PacketType::EdgeL => Ok(Packet::EdgeL),
        PacketType::EdgeR => Ok(Packet::EdgeR),
        PacketType::Err => {
            let (code, message) = read_error(stream).await?;
            Ok(Packet::Err { code, message })
        }
        // Packets with data payload
        PacketType::ServerHello => {
            let data = read_data_payload(stream).await?;
            let config = deserialize(mode, &data)?;
            Ok(Packet::ServerHello(config))
        }
        PacketType::Action => {
            let data = read_data_payload(stream).await?;
            let action = deserialize(mode, &data)?;
            Ok(Packet::Action(action))
        }
        PacketType::DropSend => {
            let data = read_data_payload(stream).await?;
            let filename = String::from_utf8_lossy(&data).to_string();
            Ok(Packet::DropSend { filename })
        }
        PacketType::DropRequest => {
            let data = read_data_payload(stream).await?;
            let filename = String::from_utf8_lossy(&data).to_string();
            Ok(Packet::DropRequest { filename })
        }
        // File data
        PacketType::Data => {
            let data = read_data_payload(stream).await?;
            Ok(Packet::Data(data))
        }
    }
}

/// Reads the packet type byte from the stream
pub async fn read_packet_type<S: AsyncStream>(stream: &mut S) -> Result<PacketType, ProtocolError> {
    let mut type_buf = [0u8; 1];
    stream.read_exact(&mut type_buf).await?;
    PacketType::try_from(type_buf[0]).map_err(|_| ProtocolError::InvalidPacketType(type_buf[0]))
}

/// Reads an error code and message from the stream
async fn read_error<S: AsyncStream>(stream: &mut S) -> Result<(ErrorCode, String), ProtocolError> {
    // Read error code
    let mut code_buf = [0u8; 1];
    stream.read_exact(&mut code_buf).await?;
    let code = ErrorCode::try_from(code_buf[0]).unwrap_or(ErrorCode::Unknown);

    // Read message length
    let mut len_buf = [0u8; 4];
    stream.read_exact(&mut len_buf).await?;
    let len = u32::from_be_bytes(len_buf) as usize;

    // Read message
    let mut msg_buf = vec![0u8; len];
    stream.read_exact(&mut msg_buf).await?;
    let message = String::from_utf8_lossy(&msg_buf).to_string();

    Ok((code, message))
}

/// Reads a length-prefixed data payload from the stream
pub async fn read_data_payload<S: AsyncStream>(stream: &mut S) -> Result<Vec<u8>, ProtocolError> {
    // Read length (4 bytes, big-endian)
    let mut len_buf = [0u8; 4];
    stream.read_exact(&mut len_buf).await?;
    let len = u32::from_be_bytes(len_buf) as usize;

    // Read data
    let mut data = vec![0u8; len];
    stream.read_exact(&mut data).await?;

    Ok(data)
}

/// Deserializes data according to the serialization mode
fn deserialize<T>(mode: SerializationMode, data: &[u8]) -> Result<T, ProtocolError>
where
    T: for<'de> Deserialize<'de>,
{
    match mode {
        SerializationMode::Json => deserialize_json(data)
            .map_err(|e| ProtocolError::InvalidData(format!("JSON error: {}", e))),
        SerializationMode::Binary => deserialize_bin(data)
            .map_err(|e| ProtocolError::InvalidData(format!("MessagePack error: {}", e))),
    }
}

/// Serialization mode for packet payloads
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SerializationMode {
    Json,
    Binary,
}

impl SerializationMode {
    /// Reads the serialization mode from env(defaults JSON)
    pub fn from_env() -> Self {
        // Prefer an explicitly-set environment variable (e.g. tests or runtime overrides)
        if let Ok(val) = std::env::var("PROTOCOL_SERIALIZATION") {
            return if val.eq_ignore_ascii_case("binary") {
                SerializationMode::Binary
            } else {
                SerializationMode::Json
            };
        }

        // If not present, try loading from a .env file and read again
        dotenv().ok();
        match env::var("PROTOCOL_SERIALIZATION") {
            Ok(val) if val.eq_ignore_ascii_case("binary") => SerializationMode::Binary,
            _ => SerializationMode::Json,
        }
    }
}

// Helper functions for serialization (used by packet.rs)

pub fn serialize_json<T: Serialize>(value: &T) -> Vec<u8> {
    serde_json::to_vec(value).unwrap_or_default()
}

pub fn deserialize_json<T: for<'de> Deserialize<'de>>(data: &[u8]) -> Result<T, String> {
    serde_json::from_slice(data).map_err(|e| format!("JSON deserialization error: {}", e))
}

pub fn serialize_bin<T: Serialize>(value: &T) -> Vec<u8> {
    rmps::to_vec(value).expect("Failed to serialize to MessagePack")
}

pub fn deserialize_bin<T: for<'de> Deserialize<'de>>(data: &[u8]) -> Result<T, String> {
    rmps::from_slice(data).map_err(|e| format!("MessagePack deserialization error: {}", e))
}
