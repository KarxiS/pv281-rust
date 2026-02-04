use crate::config::{protocol_structure::*, ServerConfig};
use crate::error::ErrorCode;
use crate::serialization::SerializationMode;
use serde_json::Value;

/// Protocol type identifiers
#[repr(u8)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum PacketType {
    /// Acknowledgment response (0)
    #[default]
    Ok = 0,
    /// Error response with message (1)
    Err = 1,
    /// Initial handshake with configuration (2)
    ServerHello = 2,
    /// Generic action/event (3)
    Action = 3,
    /// Client-initiated disconnect (4)
    ClientQuit = 4,
    /// Server sending file to client (5)
    DropSend = 5,
    /// Server requesting file from client (6)
    DropRequest = 6,
    /// File data payload (7)
    Data = 7,
    /// Cursor hit left edge (8)
    EdgeL = 8,
    /// Cursor hit right edge (9)
    EdgeR = 9,
}

impl TryFrom<u8> for PacketType {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Self::Ok,
            1 => Self::Err,
            2 => Self::ServerHello,
            3 => Self::Action,
            4 => Self::ClientQuit,
            5 => Self::DropSend,
            6 => Self::DropRequest,
            7 => Self::Data,
            8 => Self::EdgeL,
            9 => Self::EdgeR,
            _ => return Err(()),
        })
    }
}

/// Protocol packets used definitions
#[derive(Debug, Clone)]
pub enum Packet {
    Ok,
    Err {
        code: ErrorCode,
        message: String,
    },
    ServerHello(ServerConfig),
    Action(Value),
    ClientQuit,

    /// File being sent to client (includes filename)
    DropSend {
        filename: String,
    },
    /// File being requested from client (includes filename)
    DropRequest {
        filename: String,
    },
    /// Raw file data or other binary payload
    Data(Vec<u8>),
    /// Notification that cursor hit left screen edge
    EdgeL,
    /// Notification that cursor hit right screen edge
    EdgeR,
}

impl Packet {
    const fn packet_type(&self) -> PacketType {
        match self {
            Self::Ok => PacketType::Ok,
            Self::Err { .. } => PacketType::Err,
            Self::ServerHello(_) => PacketType::ServerHello,
            Self::Action(_) => PacketType::Action,
            Self::ClientQuit => PacketType::ClientQuit,
            Self::DropSend { .. } => PacketType::DropSend,
            Self::DropRequest { .. } => PacketType::DropRequest,
            Self::Data(_) => PacketType::Data,
            Self::EdgeL => PacketType::EdgeL,
            Self::EdgeR => PacketType::EdgeR,
        }
    }

    /// Serializes this protocol with mode to bytes for transmission
    ///
    /// # Format
    ///
    /// - Simple packets: `[type:u8]`
    /// - Data packets: `[type:u8][len:u32][data:bytes]`
    ///
    /// All multibyte integers use big-endian (network) byte order
    ///
    /// # Returns
    ///
    /// A `Vec<u8>` containing the serialized protocol data
    pub fn serialize(&self) -> Vec<u8> {
        self.serialize_with_mode(SerializationMode::from_env())
    }

    pub fn serialize_with_mode(&self, mode: SerializationMode) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.push(self.packet_type() as u8);

        match self {
            Self::Ok | Self::ClientQuit | Self::EdgeL | Self::EdgeR => {}
            Self::Err { code, message } => {
                buf.push(*code as u8);
                Self::insert_into_buf(&mut buf, message.as_bytes());
            }
            Self::ServerHello(config) => {
                let bytes = Self::serialize_into(mode, config);
                Self::insert_into_buf(&mut buf, &bytes);
            }
            Self::Action(action) => match mode {
                SerializationMode::Json => {
                    let bytes = Self::serialize_into(mode, action);
                    Self::insert_into_buf(&mut buf, &bytes);
                }
                SerializationMode::Binary => match action {
                    Value::String(encoded) => {
                        Self::insert_into_buf(&mut buf, encoded.as_bytes());
                    }
                    _ => {
                        let bytes = Self::serialize_into(mode, action);
                        Self::insert_into_buf(&mut buf, &bytes);
                    }
                },
            },
            Self::DropSend { filename } | Self::DropRequest { filename } => {
                Self::insert_into_buf(&mut buf, filename.as_bytes());
            }
            Self::Data(bytes) => {
                Self::insert_into_buf(&mut buf, bytes);
            }
        }
        buf
    }

    fn serialize_into<T: serde::Serialize>(mode: SerializationMode, value: &T) -> Vec<u8> {
        match mode {
            SerializationMode::Json => crate::serialization::serialize_json(value),
            SerializationMode::Binary => crate::serialization::serialize_bin(value),
        }
    }

    fn insert_into_buf(buf: &mut Vec<u8>, bytes: &[u8]) {
        buf.extend_from_slice(&(bytes.len() as u32).to_be_bytes());
        buf.extend_from_slice(bytes);
    }

    /// Deserializes a protocol packet from bytes using the default serialization mode.
    ///
    /// # Arguments
    ///
    /// * `data` - A byte slice containing the serialized protocol packet.
    ///
    /// # Returns
    ///
    /// * `Ok(Packet)` if deserialization is successful.
    /// * `Err(String)` if the data is invalid or cannot be deserialized.
    pub fn deserialize(data: &[u8]) -> Result<Self, String> {
        Self::deserialize_with_mode(data, SerializationMode::from_env())
    }

    /// Deserializes a protocol from bytes using the specified serialization mode.
    pub fn deserialize_with_mode(data: &[u8], mode: SerializationMode) -> Result<Self, String> {
        if data.is_empty() {
            return Err("Empty protocol data".to_string());
        }
        let packet_type = PacketType::try_from(data[0])
            .map_err(|_| format!("Unknown protocol type: {}", data[0]))?;

        match packet_type {
            PacketType::Ok => Ok(Self::Ok),
            PacketType::ClientQuit => Ok(Self::ClientQuit),
            PacketType::EdgeL => Ok(Self::EdgeL),
            PacketType::EdgeR => Ok(Self::EdgeR),
            PacketType::Err => Self::deserialize_error(data),
            PacketType::ServerHello => {
                let payload = Self::read_payload(data, PAYLOAD_LENGTH_OFFSET, "ServerHello")?;
                let config = Self::deserialize_from(mode, payload)?;
                Ok(Self::ServerHello(config))
            }
            PacketType::Action => {
                let payload = Self::read_payload(data, PAYLOAD_LENGTH_OFFSET, "Action")?;
                let action = match mode {
                    SerializationMode::Json => Self::deserialize_from(mode, payload)?,
                    SerializationMode::Binary => Value::String(
                        String::from_utf8(payload.to_vec())
                            .map_err(|_| "Invalid Action payload encoding".to_string())?,
                    ),
                };
                Ok(Self::Action(action))
            }
            PacketType::DropSend | PacketType::DropRequest => {
                let payload = Self::read_payload(data, PAYLOAD_LENGTH_OFFSET, "Drop")?;
                let filename = String::from_utf8_lossy(payload).to_string();
                if packet_type == PacketType::DropSend {
                    Ok(Self::DropSend { filename })
                } else {
                    Ok(Self::DropRequest { filename })
                }
            }
            PacketType::Data => {
                let payload = Self::read_payload(data, PAYLOAD_LENGTH_OFFSET, "Data")?;
                Ok(Self::Data(payload.to_vec()))
            }
        }
    }

    /// Reads a length-prefixed payload from the data buffer.
    ///
    /// # Arguments
    ///
    /// * `data` - The complete packet data
    /// * `offset` - The byte offset where the length prefix starts
    /// * `packet_name` - Name of the packet type for error messages
    ///
    /// # Returns
    ///
    /// A slice containing the payload data
    fn read_payload<'a>(
        data: &'a [u8],
        offset: usize,
        packet_name: &str,
    ) -> Result<&'a [u8], String> {
        if data.len() < offset + LENGTH_PREFIX_SIZE {
            return Err(format!("Invalid {} protocol", packet_name));
        }

        let len = u32::from_be_bytes([
            data[offset],
            data[offset + 1],
            data[offset + 2],
            data[offset + 3],
        ]) as usize;

        let payload_start = offset + LENGTH_PREFIX_SIZE;
        let payload_end = payload_start + len;

        if data.len() < payload_end {
            return Err(format!("Truncated {} protocol", packet_name));
        }

        Ok(&data[payload_start..payload_end])
    }

    /// Deserializes error packet from data.
    fn deserialize_error(data: &[u8]) -> Result<Self, String> {
        if data.len() < MIN_ERROR_PACKET_SIZE + PACKET_TYPE_SIZE {
            return Err("Invalid error protocol".to_string());
        }

        let code = ErrorCode::try_from(data[ERROR_CODE_OFFSET]).unwrap_or(ErrorCode::Unknown);
        let payload = Self::read_payload(data, ERROR_LENGTH_OFFSET, "Error")?;
        let message = String::from_utf8_lossy(payload).to_string();

        Ok(Self::Err { code, message })
    }

    /// Deserializes a value from bytes according to the serialization mode.
    fn deserialize_from<T: serde::de::DeserializeOwned>(
        mode: SerializationMode,
        data: &[u8],
    ) -> Result<T, String> {
        match mode {
            SerializationMode::Json => crate::serialization::deserialize_json(data),
            SerializationMode::Binary => crate::serialization::deserialize_bin(data),
        }
    }
}
