use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RoadLocation {
    #[serde(rename = "na jízdním pruhu")]
    TrafficLane,
    #[serde(rename = "žádné z uvedenýc")]
    Other,
    #[serde(rename = "na kolejích tramvaje")]
    TramTracks,
    #[serde(rename = "mimo komunikaci")]
    OffRoad,
    #[serde(rename = "na chodníku nebo ostrůvku")]
    Sidewalk,
    #[serde(rename = "na krajnici")]
    HardShoulder,
    #[serde(rename = "na odstavném pruhu")]
    BreakdownLane,
    #[serde(rename = "na odbočovacím, připojovacím pruhu")]
    MergeLane,
    #[serde(rename = "na stezce pro cyklisty")]
    BicyclePath,
    #[serde(rename = "na pruhu pro pomalá vozidla")]
    SlowLane,
}
