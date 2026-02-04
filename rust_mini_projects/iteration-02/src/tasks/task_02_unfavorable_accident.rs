use crate::parsing::data_access::DataAccess;
use crate::parsing::datasets::traffic_accident::enums::DriverCondition::Good;
use crate::parsing::datasets::traffic_accident::enums::DriverInfluenced::NoInfluence;
use crate::parsing::datasets::traffic_accident::enums::DriversView::Unobstructed;
use crate::parsing::datasets::traffic_accident::enums::RoadCondition::DryAndContaminated;
use crate::parsing::datasets::traffic_accident::enums::Visibility::DaytimeVisibilityNormal;
use crate::parsing::datasets::traffic_accident::enums::WeatherCondition::Favorable;
use crate::parsing::datasets::traffic_accident::enums::{
    AccidentTime, DriverCondition, DriverInfluenced, DriversView, RoadCondition, Visibility,
    WeatherCondition,
};
use std::option::Option;

pub fn road_condition_is_unsafe(x: &RoadCondition) -> bool {
    !matches!(x, DryAndContaminated)
}

pub fn weather_condition_is_unsafe(x: &WeatherCondition) -> bool {
    !matches!(x, Favorable)
}

pub fn visibility_is_unsafe(x: &Visibility) -> bool {
    !matches!(x, DaytimeVisibilityNormal)
}

pub fn drivers_view_is_unsafe(x: &DriversView) -> bool {
    !matches!(x, Unobstructed)
}

pub fn driver_condition_is_unsafe(x: &Option<DriverCondition>) -> bool {
    !matches!(x, Some(Good))
}

pub fn driver_influenced_is_unsafe(x: &Option<DriverInfluenced>) -> bool {
    !matches!(x, Some(NoInfluence))
}

/// Search for the least favorable accident on record, one where the odds were stacked against the drivers.
pub fn unfavorable_accident() -> Option<String> {
    let data = DataAccess::accidents();

    //RoadCondition,WeatherCondition, Visibility,DriversView,
    // DriverCondition,DriverInfluenced,DriverCondition - i need to check this

    //i will use scoring approach(count all OR's (like in onechips c++ when couting errors)) instead of filter -
    // i want to get the "worst" so it will always return something if there wont be all conditions met
    let least_favourable_counts = data.iter().map(|x| {
        let mut count = 0;
        if road_condition_is_unsafe(&x.overview.road_condition) {
            count += 1;
        }
        if weather_condition_is_unsafe(&x.overview.weather_condition) {
            count += 1;
        }
        if visibility_is_unsafe(&x.overview.visibility) {
            count += 1;
        }
        if drivers_view_is_unsafe(&x.overview.drivers_view) {
            count += 1;
        }

        if !x.participant_vehicles.is_empty() {
            let only_known_condition: Vec<_> = x
                .participant_vehicles
                .iter()
                .filter(|v| v.driver_condition.is_some())
                .collect();
            if !only_known_condition.is_empty()
                && only_known_condition
                    .iter()
                    .all(|v| driver_condition_is_unsafe(&v.driver_condition))
            {
                count += 1;
            }

            let only_known_influenced: Vec<_> = x
                .participant_vehicles
                .iter()
                .filter(|v| v.driver_influenced.is_some())
                .collect();
            if !only_known_influenced.is_empty()
                && only_known_influenced
                    .iter()
                    .all(|v| driver_influenced_is_unsafe(&v.driver_influenced))
            {
                count += 1;
            }
        }
        (x, count)
    });
    let max_count = least_favourable_counts.max_by_key(|(_, y)| *y);

    match max_count {
        Some((accident, _)) => {
            let time_string_builder = match accident.overview.time {
                AccidentTime::Exact(time) => time.format("%k:%M").to_string(),
                AccidentTime::Hour(time) => {
                    format!(" {}:??", time)
                }
                AccidentTime::Unknown => " ??:??".to_string(),
            };

            Some(format!(
                "{}{}",
                accident.overview.date.format("%-d. %-m. %Y"),
                time_string_builder
            ))
        }
        None => None,
    }
}
