use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DriverCondition {
    #[serde(rename = "dobrý -žádné nepříznivé okolnosti nebyly zjiště")]
    Good,
    #[serde(rename = "pod vlivem alkoholu obsah alkoholu v krvi 1 ‰ a víc")]
    UnderTheInfluenceOfAlcoholAbove1Permille,
    #[serde(rename = "pod vlivem alkoholu, obsah alkoholu v krvi do 0,99 ‰")]
    UnderTheInfluenceOfAlcoholUnder1Permille,
    #[serde(rename = "unaven, usnul, náhlá fyzická indispozic")]
    TiredOrFellAsleep,
    #[serde(rename = "pod vlivem léků, narkoti")]
    UnderTheInfluenceOfMedicineOrDrugs,
    #[serde(rename = "nemoc, úraz apod.")]
    IllOrInjured,
    #[serde(rename = "invalid")]
    Disability,
    #[serde(rename = "řidič při jízdě zemřel (infarkt apod.")]
    DiedWhileDriving,
    #[serde(rename = "pokus o sebevraždu, sebevražd")]
    Suicide,
    #[serde(rename = "jiný nepříznivý sta")]
    OtherUnfavorableCondition,
}
