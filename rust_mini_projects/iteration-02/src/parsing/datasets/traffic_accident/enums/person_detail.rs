use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PersonDetail {
    #[serde(rename = "připoutaný bezpečnostními pásy (i na zadních sedadlech)")]
    WearingASeatbelt,
    #[serde(rename = "nepřipoutaný bezpečnostními pásy")]
    NotWearingASeatbelt,
    #[serde(rename = "s přilbou (pouze u motocyklistů, příp. cyklistů)")]
    WearingAHelmet,
    #[serde(rename = "bez přilby (pouze u motocyklistů, příp. cyklistů)")]
    NotWearingAHelmet,
    #[serde(rename = "bezpečnostní vak (airbag) v činnosti - osoba připoutaná")]
    AirbagDeployedWhileWearingASeatbelt,
    #[serde(rename = "bezpečnostní vak (airbag) v činnosti - osoba nepřipoutaná")]
    AirbagDeployedWhileNotWearingASeatbelt,
    #[serde(rename = "sedící v dětské sedačce")]
    InABoosterSeat,
    #[serde(rename = "vozidlo nevybaveno dětskou sedačkou")]
    NotInABoosterSeat,
}
