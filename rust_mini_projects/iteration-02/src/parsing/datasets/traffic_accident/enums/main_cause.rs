use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MainCause {
    #[serde(rename = "nesprávný způsob jízdy")]
    ImproperDriving,
    #[serde(rename = "nedání přenosti v jízdě")]
    FailureToYield,
    #[serde(rename = "nepřiměřená rychlost jízdy")]
    InappropriateSpeed,
    #[serde(rename = "nezaviněno řidičem")]
    NotCausedByDriver,
    #[serde(rename = "nesprávé předjíždění")]
    IncorrectOvertaking,
    #[serde(rename = "technická závada vozidla")]
    TechnicalFault,
}
