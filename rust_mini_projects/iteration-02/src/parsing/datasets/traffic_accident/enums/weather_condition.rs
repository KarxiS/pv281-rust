use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum WeatherCondition {
    #[serde(rename = "neztížené")]
    Favorable,
    #[serde(rename = "na počátku deště, slabý déšť, mrholení apod.")]
    StartingToRain,
    #[serde(rename = "déšť")]
    Rain,
    #[serde(rename = "sněžení")]
    Snow,
    #[serde(rename = "tvoří se námraza, náledí")]
    GlazeIce,
    #[serde(rename = "mlha")]
    Fog,
    #[serde(rename = "nárazový vítr (boční, vichřice apod.)")]
    HeavyWind,
    #[serde(rename = "jiné ztížené")]
    OtherAdverseCondition,
}
