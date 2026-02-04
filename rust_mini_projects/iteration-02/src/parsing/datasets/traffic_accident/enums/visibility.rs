use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Visibility {
    #[serde(rename = "ve dne, viditelnost nezhoršená vlivem povětrnostních podmínek")]
    DaytimeVisibilityNormal,
    #[serde(rename = "ve dne, zhoršená viditelnost (svítání, soumrak)")]
    DaytimeVisibilityWorsenedByLessLight,
    #[serde(
        rename = "ve dne, zhoršená viditelnost vlivem povětrnostních podmínek (mlha, sněžení, déšť apod.)"
    )]
    DaytimeVisibilityWorsenedByWeather,
    #[serde(
        rename = "v noci - s veřejným osvětlením, viditelnost nezhoršená vlivem povětrnostních podmínek"
    )]
    NighttimeVisibilityWithStreetLightsNormal,
    #[serde(
        rename = "v noci - bez veřejného osvětlení, viditelnost nezhoršená vlivem povětrnostních podmínek"
    )]
    NighttimeVisibilityWithoutStreetLightsNormal,
    #[serde(
        rename = "v noci - s veřejným osvětlením, zhoršená viditelnost vlivem povětrnostních podmínek (mlha, déšť, sněžení apod.)"
    )]
    NighttimeVisibilityWithStreetLightsWorsenedByWeather,
    #[serde(
        rename = "v noci - bez veřejného osvětlení, viditelnost zhoršená vlivem povětrnostních podmínek (mlha, déšť, sněžení apod."
    )]
    NighttimeVisibilityWithoutStreetLightsWorsenedByWeather,
}
