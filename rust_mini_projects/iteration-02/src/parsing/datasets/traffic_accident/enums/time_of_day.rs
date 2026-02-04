use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TimeOfDay {
    #[serde(rename = "den")]
    Daytime,
    #[serde(rename = "noc")]
    Nighttime,
}
