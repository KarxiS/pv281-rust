use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PersonalOutcome {
    #[serde(rename = "bez zranění")]
    NoInjury,
    #[serde(rename = "lehké zranění")]
    LightInjury,
    #[serde(rename = "těžké zranění")]
    HeavyInjury,
    #[serde(rename = "usmrcení")]
    Death,
}
