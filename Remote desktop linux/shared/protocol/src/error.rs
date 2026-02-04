use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::{fmt, io};

/// Error types for all protocol operations
#[derive(Debug)]
pub enum ProtocolError {
    /// IO error during network communication
    Io(io::Error),
    /// Unknown or invalid packet type received
    InvalidPacketType(u8),
    /// Data deserialization or validation error
    InvalidData(String),
    /// Packet data was truncated or incomplete
    TruncatedPacket,
    /// Protocol-level error with error code
    Protocol { code: ErrorCode, message: String },
}

impl From<io::Error> for ProtocolError {
    fn from(err: io::Error) -> Self {
        ProtocolError::Io(err)
    }
}

impl From<ErrorCode> for ProtocolError {
    fn from(code: ErrorCode) -> Self {
        ProtocolError::Protocol {
            code,
            message: code.to_string(),
        }
    }
}

impl Display for ProtocolError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ProtocolError::Io(e) => write!(f, "IO error: {}", e),
            ProtocolError::InvalidPacketType(t) => write!(f, "Invalid packet type: {}", t),
            ProtocolError::InvalidData(msg) => write!(f, "Invalid data: {}", msg),
            ProtocolError::TruncatedPacket => write!(f, "Truncated packet"),
            ProtocolError::Protocol { code, message } => write!(f, "{}: {}", code, message),
        }
    }
}

impl Error for ProtocolError {}

impl ProtocolError {
    /// Creates a new protocol error with the given code and message
    pub fn new(code: ErrorCode, message: impl Into<String>) -> Self {
        ProtocolError::Protocol {
            code,
            message: message.into(),
        }
    }

    /// Gets the error code if this is a protocol error
    pub fn code(&self) -> Option<ErrorCode> {
        match self {
            ProtocolError::Protocol { code, .. } => Some(*code),
            ProtocolError::InvalidPacketType(_) => Some(ErrorCode::InvalidPacket),
            ProtocolError::InvalidData(_) => Some(ErrorCode::InvalidPacket),
            ProtocolError::TruncatedPacket => Some(ErrorCode::InvalidPacket),
            ProtocolError::Io(_) => Some(ErrorCode::Internal),
        }
    }
}

/// Error codes used in protocol error packets
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, PartialEq, Eq)]
#[repr(u8)]
pub enum ErrorCode {
    #[default]
    Unknown = 0,
    InvalidPacket = 1,
    NotFound = 2,
    Internal = 3,
    // Add more error codes as needed
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            ErrorCode::Unknown => "Unknown error",
            ErrorCode::InvalidPacket => "Invalid packet",
            ErrorCode::NotFound => "Not found",
            ErrorCode::Internal => "Internal error",
        };
        write!(f, "{}", s)
    }
}

impl TryFrom<u8> for ErrorCode {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => ErrorCode::Unknown,
            1 => ErrorCode::InvalidPacket,
            2 => ErrorCode::NotFound,
            3 => ErrorCode::Internal,
            _ => ErrorCode::Unknown,
        })
    }
}

impl From<ErrorCode> for u8 {
    fn from(code: ErrorCode) -> u8 {
        code as u8
    }
}
