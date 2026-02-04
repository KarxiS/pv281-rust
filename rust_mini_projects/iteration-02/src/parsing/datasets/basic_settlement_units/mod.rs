use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasicSettlementUnit {
    #[serde(rename = "Kód")]
    pub id: u64,
    #[serde(rename = "Název ZSJ")]
    pub name: String,
    #[serde(rename = "Kód katastrálního území")]
    pub cadastral_id: u64,
    #[serde(rename = "Název katastrálního území")]
    pub cadastral_name: String,
    #[serde(rename = "Kód Obce")]
    pub municipality_id: u64,
    #[serde(rename = "Název Obce")]
    pub municipality_name: String,
    #[serde(rename = "Kód Okresu")]
    pub district_id: Option<u64>,
    #[serde(rename = "Název Okresu")]
    pub district_name: Option<String>,
}

impl PartialEq for BasicSettlementUnit {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
