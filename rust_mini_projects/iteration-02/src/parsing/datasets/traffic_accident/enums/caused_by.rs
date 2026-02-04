use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CausedBy {
    #[serde(rename = "řidičem motorového vozidla")]
    MotorizedVehicleDriver,
    #[serde(rename = "chodcem")]
    Pedestrian,
    #[serde(rename = "řidičem nemotorového vozidla")]
    NonMotorizedVehicleDriver,
    #[serde(rename = "lesní zvěří, domácím zvířectvem")]
    Animal,
    #[serde(rename = "jiné zaviněn")]
    Other,
    #[serde(rename = "technickou závadou vozidla")]
    VehicleMalfunction,
    #[serde(rename = "závadou komunikace")]
    RoadMalfunction,
    #[serde(rename = "jiným účastníkem silničního provozu")]
    OtherRoadUser,
}
