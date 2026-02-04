use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AccidentLocation {
    #[serde(rename = "žádné nebo žádné z uvedených")]
    Other,
    #[serde(rename = "parkoviště přiléhající ke komunikaci")]
    ParkingLot,
    #[serde(rename = "přechod pro chodce")]
    PedestrianCrossing,
    #[serde(rename = "v blízkosti přechodu pro chodce (do vzdálenosti 20 m)")]
    NearPedestrianCrossing,
    #[serde(rename = "most, nadjezd, podjezd, tunel")]
    BridgeOverpassUnderpassTunnel,
    #[serde(rename = "zastávka tramvaje, autobusu, trolejbusu bez nástup. ostrůvku")]
    PublicTransportStopWithoutIsland,
    #[serde(rename = "zastávka autobusu, trolejbusu, tramvaje s nástup. ostrůvkem")]
    PublicTransportStopWithIsland,
    #[serde(rename = "výjezd z parkoviště, lesní cesty apod. (pol.36=7,8)")]
    EnteringRoadFromParkingLot,
    #[serde(rename = "čerpadlo pohonných hmot")]
    GasStation,
    #[serde(
        rename = "železniční přejezd nezabezpečený závorami ani světelným výstražným zařízením"
    )]
    RailwayCrossingsUnprotected,
    #[serde(rename = "železniční přejezd zabezpečený")]
    RailwayCrossingsProtected,
}
