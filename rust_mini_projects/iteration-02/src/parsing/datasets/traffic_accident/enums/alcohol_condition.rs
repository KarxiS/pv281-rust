use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AlcoholCondition {
    #[serde(rename = "Ne")]
    None,
    #[serde(rename = "Nezjišťován")]
    NotInvestigated,
    #[serde(rename = "Pod vlivem drog")]
    UnderTheInfluenceOfDrugs,
    #[serde(rename = "Pod vlivem alkoholu a drog")]
    UnderTheInfluenceOfDrugsAndAlcohol,
    #[serde(rename = "Ano, obsah alkoholu v krvi 1,5‰ a více")]
    Above1_5Permille,
    #[serde(rename = "Ano, obsah alkoholu v krvi od 1,0‰ do 1,5‰")]
    Between1_0and1_5Permille,
    #[serde(rename = "Ano, obsah alkoholu v krvi od 0,8‰ do 1,0‰")]
    Between0_8and1_0Permille,
    #[serde(rename = "Ano, obsah alkoholu v krvi od 0,5‰ do 0,8‰")]
    Between0_5and0_8Permille,
    #[serde(rename = "Ano, obsah alkoholu v krvi od 0,24‰ do 0,5‰")]
    Between0_24and0_5Permille,
    #[serde(rename = "Ano, obsah alkoholu v krvi do 0,24 ‰")]
    Below0_24Permille,
}
