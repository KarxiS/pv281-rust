use base64::{Engine as _, engine::general_purpose};
use kmf_protocol::SerializationMode;
use kmf_protocol::config::ServerMessage;
use kmf_protocol::serialization::{deserialize_bin, serialize_bin};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::path::Path;
use std::str::FromStr;

/// Generic action/event for protocol, decoupled from driver crate.
#[derive(Debug, Serialize, Deserialize)]
pub enum GenericAction {
    MouseMove { x: i32, y: i32, wheel: i32 },
    MouseClick { button: String, pressed: bool },
    KeyPress { key: String, pressed: bool },
}

/// Parses a command string into a ServerMessage.
///
/// This function validates command syntax and arguments, returning `None` for invalid input.
///
/// # Arguments
///
/// * `input` - The command string to parse
///
/// # Returns
///
/// - `Some(ServerMessage)` if the command is valid
/// - `None` if the command is invalid or has wrong number of arguments
///   TODO: return error if functions fails
pub fn parse_command(input: &str) -> Option<ServerMessage> {
    let parts: Vec<&str> = input.split_whitespace().collect();
    if parts.is_empty() {
        return None;
    }

    let mode = SerializationMode::from_env();

    let Ok(command) = Commands::from_str(parts[0]) else {
        return None;
    };

    match command {
        Commands::Move if parts.len() == 3 => {
            let x = parts[1].parse::<i32>().ok()?;
            let y = parts[2].parse::<i32>().ok()?;
            let action = GenericAction::MouseMove { x, y, wheel: 0 };
            let value = serialize_action(mode, &action)?;
            Some(ServerMessage::Action(value))
        }
        Commands::Click if parts.len() == 3 => {
            let button = parts[1].to_string();
            let pressed = match detect_pressed_key(&parts) {
                Ok(value) => value,
                Err(value) => return value,
            };
            let action = GenericAction::MouseClick { button, pressed };
            let value = serialize_action(mode, &action)?;
            Some(ServerMessage::Action(value))
        }
        Commands::Key if parts.len() == 3 => {
            let key = parts[1].to_string();
            let pressed = match detect_pressed_key(&parts) {
                Ok(value) => value,
                Err(value) => return value,
            };
            let action = GenericAction::KeyPress { key, pressed };
            let value = serialize_action(mode, &action)?;
            Some(ServerMessage::Action(value))
        }
        Commands::File if parts.len() == 2 => {
            let file_path = parts[1].to_string();
            let path = Path::new(&file_path);
            if !path.exists() {
                eprintln!("File does not exist: {}", file_path);
                return None;
            }
            Some(ServerMessage::File { path: file_path })
        }
        Commands::Quit => Some(ServerMessage::Quit),
        _ => None,
    }
}

const DOWN_KEY: &str = "d";
const UP_KEY: &str = "u";

fn detect_pressed_key(parts: &[&str]) -> Result<bool, Option<ServerMessage>> {
    Ok(match parts[2] {
        DOWN_KEY => true,
        UP_KEY => false,
        _ => return Err(None),
    })
}

fn serialize_action<T: Serialize>(mode: SerializationMode, action: &T) -> Option<Value> {
    Some(match mode {
        SerializationMode::Json => serde_json::to_value(action).ok()?,
        SerializationMode::Binary => {
            // Use protocol's MessagePack serialization, then base64 encode for embedding in JSON
            let bin = serialize_bin(action);
            Value::String(general_purpose::STANDARD.encode(bin))
        }
    })
}

/// Deserializes a generic action/event from a serde_json::Value, supporting both JSON and base64-encoded MessagePack.
pub fn deserialize_action<T: for<'de> Deserialize<'de>>(
    mode: SerializationMode,
    value: &Value,
) -> Option<T> {
    match mode {
        SerializationMode::Json => serde_json::from_value(value.clone()).ok(),
        SerializationMode::Binary => {
            if let Value::String(s) = value {
                let bin = general_purpose::STANDARD.decode(s).ok()?;
                deserialize_bin(&bin).ok()
            } else {
                None
            }
        }
    }
}

/// Serializes a generic action/event and wraps it in ServerMessage::Action.
pub fn send_action<T: Serialize>(action: &T) -> Option<ServerMessage> {
    let mode = SerializationMode::from_env();
    let value = serialize_action(mode, action);
    // Send or throw
    Some(ServerMessage::Action(value.unwrap()))
}

/// Command types recognized by the server input parser.
pub enum Commands {
    /// Move mouse cursor
    Move,
    /// Click mouse button
    Click,
    /// Press keyboard key
    Key,
    /// Transfer file
    File,
    /// Disconnect all clients
    Quit,
}

impl FromStr for Commands {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "move" => Ok(Commands::Move),
            "click" => Ok(Commands::Click),
            "key" => Ok(Commands::Key),
            "file" => Ok(Commands::File),
            "quit" => Ok(Commands::Quit),
            _ => Err(()),
        }
    }
}
