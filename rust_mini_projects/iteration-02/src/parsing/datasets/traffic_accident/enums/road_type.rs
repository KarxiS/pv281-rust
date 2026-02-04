use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RoadType {
    #[serde(rename = "komunikace sledovaná (ve vybraných městech)")]
    CityRoad,
    #[serde(rename = "komunikace místní")]
    LocalRoad,
    #[serde(rename = "uzel (křižovatka sledovaná ve vybraných městech)")]
    Crossroads,
    #[serde(rename = "komunikace účelová - ostatní (parkoviště, odpočívky apod.")]
    PrivateRoad,
    #[serde(rename = "dálnice")]
    Highway,
    #[serde(rename = "komunikace účelová - polní a lesní cesty atd.")]
    CountryRoad,
    #[serde(rename = "silnice I. třídy")]
    FirstClassRoad,
    #[serde(rename = "silnice II. třídy")]
    SecondClassRoad,
    #[serde(rename = "silnice III. třídy")]
    ThirdClassRoad,
}
