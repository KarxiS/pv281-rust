use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DriversView {
    #[serde(rename = "dobré")]
    Unobstructed,
    #[serde(rename = "špatné - vlivem okolní zástavby (budovy, plné zábradlí apod.)")]
    ObstructedByBuildings,
    #[serde(rename = "výhled zakryt stojícím vozidlem")]
    ObstructedByStationaryVehicle,
    #[serde(rename = "jiné špatn")]
    ObstructedOtherwise,
    #[serde(
        rename = "špatné - vlivem průběhu komunikace, nebo podélného profilu nebo trasování (nepřehledný vrchol stoupání, zářez komunikace apod.)"
    )]
    ObstructedByRoadCurvature,
    #[serde(rename = "špatné - vlivem vegetace - trvale (stromy, keře apod.)")]
    ObstructedByPermanentVegetation,
    #[serde(rename = "špatné - vlivem vegetace - přechodně (tráva, obilí apod.)")]
    ObstructedByTemporaryVegetation,
}
