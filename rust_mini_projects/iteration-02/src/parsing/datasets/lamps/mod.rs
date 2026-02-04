use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lamp {
    #[serde(rename = "objectid")]
    pub object_id: u64,

    #[serde(rename = "evidenční_číslo")]
    pub evidence_number: String,

    #[serde(rename = "název_ulice")]
    pub address_name: String,

    #[serde(rename = "katastr")]
    pub kataster: String,

    #[serde(rename = "vlastník")]
    pub owner: String,

    #[serde(rename = "typ_sv__místa")]
    pub type_of_light_place: String,

    #[serde(rename = "typ_stožáru")]
    pub type_of_stick: String,

    #[serde(rename = "typ_svítidla")]
    pub type_of_light: String,

    #[serde(rename = "počet_svítidel")]
    pub numer_of_lights: u32,

    #[serde(rename = "datum_instalace_stožáru")]
    pub date_of_install: String,

    #[serde(rename = "latitude")]
    pub latitude: f64,

    #[serde(rename = "longitude")]
    pub longitude: f64,

    #[serde(rename = "globalid")]
    pub global_id: String,
}

impl PartialEq for Lamp {
    fn eq(&self, other: &Self) -> bool {
        self.object_id == other.object_id
    }
}
