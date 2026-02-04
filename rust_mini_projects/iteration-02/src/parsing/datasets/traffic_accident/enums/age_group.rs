use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AgeGroup {
    #[serde(rename = "0-6")]
    Between0and6,
    #[serde(rename = "7-11")]
    Between7and11,
    #[serde(rename = "12-15")]
    Between12and15,
    #[serde(rename = "16-18")]
    Between16and18,
    #[serde(rename = "19-24")]
    Between19and24,
    #[serde(rename = "24-32")]
    Between24and32,
    #[serde(rename = "33-44")]
    Between33and44,
    #[serde(rename = "45-60")]
    Between45and60,
    #[serde(rename = "61-70")]
    Between61and70,
    #[serde(rename = "71 a více")]
    Above71,
}
