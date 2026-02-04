use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AccidentOutcome {
    #[serde(rename = "nehoda pouze s hmotnou škodou")]
    PropertyDamageOnly,
    #[serde(rename = "nehoda s následky na životě nebo zdraví")]
    InjuriesOrDeath,
}
