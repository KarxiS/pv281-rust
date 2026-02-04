use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DriverInfluenced {
    #[serde(rename = "0")] // jiné ovlivnění
    OtherInfluence = 0,
    #[serde(rename = "1")] // řidič nebyl ovlivněn
    NoInfluence,
    #[serde(rename = "2")] // oslněn sluncem
    BlindedByTheSun,
    #[serde(rename = "3")] // oslněn světlomety jiného vozidla
    BlindedByAnotherVehicle,
    #[serde(rename = "4")] // ovlivněn jednáním jiného účastníka silničního provozu
    InfluencedBySomeone,
    #[serde(rename = "5")] // ovlivněn při vyhýbání lesní zvěři, domácímu zvířectvu apod.
    InfluencedDueToAvoidingAnimals,
}
