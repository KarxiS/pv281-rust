use chrono::{NaiveDate, Weekday};

use crate::parsing::datasets::traffic_accident::TrafficAccidentRow;
use crate::parsing::datasets::traffic_accident::enums::{
    AccidentLocation, AccidentOutcome, AccidentTime, AlcoholCondition, AlcoholInCulprit, CausedBy,
    DriversView, MainCause, RoadCondition, RoadLocation, RoadType, TimeOfDay, Visibility,
    WeatherCondition,
};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Overview {
    pub global_id: String,
    pub object_id: isize,
    pub police_accident_id: i64,
    pub basic_settlement_unit: String,
    pub district: String,

    pub longitude: String,
    pub latitude: String,
    pub sjstk_x_coordinate: f64,
    pub sjstk_y_coordinate: f64,

    pub date: NaiveDate,
    pub time: AccidentTime,
    pub day_of_the_week: Weekday,
    pub time_of_day: TimeOfDay,

    pub alcohol: AlcoholCondition,
    pub alcohol_in_culprit: AlcoholInCulprit,

    pub accident_location: AccidentLocation,
    pub accident_aftermath: AccidentOutcome,

    pub location: RoadLocation,
    pub road_type: RoadType,
    pub road_condition: RoadCondition,

    pub visibility: Visibility,
    pub weather_condition: WeatherCondition,
    pub drivers_view: DriversView,

    pub main_cause: MainCause,
    pub detailed_cause: String,
    pub collision: String,
    pub caused_by: CausedBy,

    pub light_injuries_count: i64,
    pub heavy_injuries_count: i64,
    pub death_count: i64,
    pub damage_in_czk: i64,
}

impl From<TrafficAccidentRow> for Overview {
    fn from(row: TrafficAccidentRow) -> Self {
        Self {
            global_id: row.global_id,
            object_id: row.object_id,
            police_accident_id: row.police_accident_id,
            basic_settlement_unit: row.basic_settlement_unit,
            district: row.district,
            longitude: row.longitude,
            latitude: row.latitude,
            sjstk_x_coordinate: row.sjstk_x_coordinate,
            sjstk_y_coordinate: row.sjstk_y_coordinate,
            date: row.date,
            time: row.time,
            day_of_the_week: row.day_of_the_week,
            time_of_day: row.time_of_day,
            alcohol: row.alcohol,
            alcohol_in_culprit: row.alcohol_in_culprit,
            accident_location: row.accident_location,
            accident_aftermath: row.accident_outcome,
            location: row.location,
            road_type: row.road_type,
            road_condition: row.road_condition,
            visibility: row.visibility,
            weather_condition: row.weather_condition,
            drivers_view: row.drivers_view,
            main_cause: row.main_cause,
            detailed_cause: row.detailed_cause,
            collision: row.collision,
            caused_by: row.caused_by,
            light_injuries_count: row.light_injuries_count,
            heavy_injuries_count: row.heavy_injuries_count,
            death_count: row.death_count,
            damage_in_czk: row.damage_in_czk,
        }
    }
}

impl PartialEq for Overview {
    fn eq(&self, other: &Self) -> bool {
        self.police_accident_id == other.police_accident_id
    }
}
