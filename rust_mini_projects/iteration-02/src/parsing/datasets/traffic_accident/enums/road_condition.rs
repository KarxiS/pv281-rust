use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RoadCondition {
    #[serde(rename = "povrch suchý, neznečistěný")]
    DryAndUncontaminated,
    #[serde(rename = "povrch mokrý")]
    Wet,
    #[serde(rename = "na vozovce je náledí, ujetý sníh - neposypané")]
    IcyAndNotSprinkled,
    #[serde(rename = "na vozovce je náledí, ujetý sníh - posypané")]
    IcyAndSprinkled,
    #[serde(rename = "souvislá sněhová vrstva, rozbředlý sníh")]
    Snow,
    #[serde(rename = "povrch suchý, znečistěný (písek, prach, listí, štěrk atd.)")]
    DryAndContaminated,
    #[serde(rename = "jiný stav povrchu vozovky v době nehod")]
    Other,
    #[serde(rename = "na vozovce je bláto")]
    Muddy,
    #[serde(rename = "náhlá změna stavu vozovky (námraza na mostu, místní náledí)")]
    SuddenChange,
    #[serde(rename = "na vozovce je rozlitý olej, nafta apod.")]
    SpiltOil,
}
