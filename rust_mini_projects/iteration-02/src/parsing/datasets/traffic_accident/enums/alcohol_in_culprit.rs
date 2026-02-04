use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AlcoholInCulprit {
    #[serde(rename = "ne")]
    None,
    #[serde(rename = "nezjišťováno")]
    NotInvestigated,
    #[serde(rename = "ano")]
    Yes,
}
