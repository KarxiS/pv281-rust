use base64::{Engine as _, engine::general_purpose};
use kmf_driver::event::{DriverEvent, KeyboardPress, MouseButton, MouseClick, MouseMove};
use kmf_protocol::SerializationMode;
use kmf_protocol::serialization::deserialize_bin;
use serde_json::Value;

use crate::command::GenericAction;

pub fn action_to_driver_event(action: Value) -> Result<DriverEvent, String> {
    let mode = SerializationMode::from_env();
    println!("[ACTION] Raw value: {}", action);
    let generic = match mode {
        SerializationMode::Json => serde_json::from_value::<GenericAction>(action)
            .map_err(|e| format!("Failed to deserialize GenericAction: {}", e))?,
        SerializationMode::Binary => match action {
            Value::String(encoded) => {
                let bytes = general_purpose::STANDARD
                    .decode(encoded)
                    .map_err(|e| format!("Failed to decode base64: {}", e))?;
                deserialize_bin::<GenericAction>(&bytes)
                    .map_err(|e| format!("Failed to deserialize GenericAction: {}", e))?
            }
            _ => return Err("Expected base64 string for binary action".to_string()),
        },
    };

    generic_action_to_driver_event(generic)
}

fn generic_action_to_driver_event(action: GenericAction) -> Result<DriverEvent, String> {
    match action {
        GenericAction::MouseMove { x, y, wheel } => {
            Ok(DriverEvent::MouseMove(MouseMove { x, y, wheel }))
        }
        GenericAction::MouseClick { button, pressed } => {
            let button = match button.to_lowercase().as_str() {
                "left" => MouseButton::Left,
                "right" => MouseButton::Right,
                "middle" => MouseButton::Middle,
                _ => return Err(format!("Unknown mouse button: {}", button)),
            };
            Ok(DriverEvent::MouseClick(MouseClick { button, pressed }))
        }
        GenericAction::KeyPress { key, pressed } => {
            let key = key
                .parse::<u16>()
                .map_err(|_| format!("Invalid key code: {}", key))?;
            Ok(DriverEvent::KeyboardPress(KeyboardPress { key, pressed }))
        }
    }
}
