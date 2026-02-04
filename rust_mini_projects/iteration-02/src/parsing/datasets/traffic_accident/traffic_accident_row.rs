use chrono::{NaiveDate, Weekday};
use serde::{Deserialize, Serialize};

use crate::parsing::datasets::traffic_accident::enums::{
    AccidentLocation, AccidentOutcome, AccidentTime, AgeGroup, AlcoholCondition, AlcoholInCulprit,
    CausedBy, DriverCondition, DriverInfluenced, DriversView, MainCause, PersonDetail,
    PersonalOutcome, RoadCondition, RoadLocation, RoadType, TimeOfDay, Visibility,
    WeatherCondition,
};
use crate::parsing::datasets::traffic_accident::structs::{
    Overview, Pedestrian, Vehicle, VehicleOccupant,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrafficAccidentRow {
    // ########
    // ACCIDENT
    // ########
    #[serde(rename = "GlobalID")]
    pub global_id: String,
    #[serde(rename = "OBJECTID")]
    pub object_id: isize,
    #[serde(rename = "id")]
    pub police_accident_id: i64,
    #[serde(rename = "ZSJ")]
    pub basic_settlement_unit: String,
    #[serde(rename = "MC")]
    pub district: String,

    #[serde(rename = "x")]
    pub longitude: String,
    #[serde(rename = "y")]
    pub latitude: String,
    #[serde(rename = "d")]
    pub sjstk_x_coordinate: f64,
    #[serde(rename = "e")]
    pub sjstk_y_coordinate: f64,

    #[serde(with = "crate::parsing::datasets::traffic_accident::serde::accident_date")]
    #[serde(rename = "datum")]
    pub date: NaiveDate,
    #[serde(rename = "cas")]
    #[serde(with = "crate::parsing::datasets::traffic_accident::serde::time")]
    pub time: AccidentTime,
    #[serde(rename = "den_v_tydnu")]
    #[serde(with = "crate::parsing::datasets::traffic_accident::serde::weekday")]
    pub day_of_the_week: Weekday,
    #[serde(rename = "doba")]
    pub time_of_day: TimeOfDay,

    #[serde(rename = "alkohol")]
    pub alcohol: AlcoholCondition,
    #[serde(rename = "alkohol_vinik")]
    pub alcohol_in_culprit: AlcoholInCulprit,

    #[serde(rename = "misto_nehody")]
    pub accident_location: AccidentLocation,
    #[serde(rename = "nasledky")]
    pub accident_outcome: AccidentOutcome,

    #[serde(rename = "situovani")]
    pub location: RoadLocation,
    #[serde(rename = "druh_komun")]
    pub road_type: RoadType,
    #[serde(rename = "stav_vozovky")]
    pub road_condition: RoadCondition,

    #[serde(rename = "viditelnost")]
    pub visibility: Visibility,
    #[serde(rename = "povetrnostni_podm")]
    pub weather_condition: WeatherCondition,
    #[serde(rename = "rozhled")]
    pub drivers_view: DriversView,

    #[serde(rename = "hlavni_pricina")]
    pub main_cause: MainCause,
    #[serde(rename = "pricina")]
    pub detailed_cause: String,
    #[serde(rename = "srazka")]
    pub collision: String,
    #[serde(rename = "zavineni")]
    pub caused_by: CausedBy,

    #[serde(rename = "lehce_zran_os")]
    pub light_injuries_count: i64,
    #[serde(rename = "tezce_zran_os")]
    pub heavy_injuries_count: i64,
    #[serde(rename = "usmrceno_os")]
    pub death_count: i64,
    #[serde(rename = "hmotna_skoda")]
    pub damage_in_czk: i64,

    // ########
    // VEHICLE
    // ########
    #[serde(rename = "id_vozidla")]
    pub vehicle_id_within_accident: Option<i64>,
    #[serde(rename = "druh_vozidla")]
    pub vehicle_type: String,
    #[serde(rename = "stav_ridic")]
    pub driver_condition: Option<DriverCondition>,
    #[serde(rename = "ovlivneni_ridice")]
    pub driver_influenced: Option<DriverInfluenced>,

    // ########
    // PERSON IN VEHICLE
    // ########
    #[serde(rename = "osoba")]
    pub person: String,
    #[serde(rename = "ozn_osoba")]
    pub person_clarification: Option<PersonDetail>,
    #[serde(rename = "pohlavi")]
    pub gender: String,
    #[serde(rename = "vek_skupina")]
    pub age_group: Option<AgeGroup>,

    #[serde(
        rename = "lz",
        with = "crate::parsing::datasets::traffic_accident::serde::bool"
    )]
    pub had_light_injuries: bool,
    #[serde(
        rename = "tz",
        with = "crate::parsing::datasets::traffic_accident::serde::bool"
    )]
    pub had_heavy_injuries: bool,
    #[serde(
        rename = "smrt",
        with = "crate::parsing::datasets::traffic_accident::serde::bool"
    )]
    pub had_died: bool,
    #[serde(
        rename = "smrt_po",
        with = "crate::parsing::datasets::traffic_accident::serde::optional_bool"
    )]
    pub had_died_after_24_hours: Option<bool>,
    #[serde(rename = "smrt_dny")]
    pub days_after_accident_till_death: Option<i64>,
    #[serde(rename = "nasledek")]
    pub outcome: Option<PersonalOutcome>,

    // ########
    // PEDESTRIAN
    // ########
    #[serde(rename = "chovani_chodce")]
    pub pedestrian_behavior: String,
    #[serde(rename = "kategorie_chodce")]
    pub pedestrian_category: String,
    #[serde(rename = "nasledky_chodce")]
    pub pedestrian_outcome: Option<PersonalOutcome>,
    #[serde(rename = "stav_chodce")]
    pub pedestrian_condition: String,
    #[serde(rename = "situace_nehody")]
    pub pedestrian_situation: String,
    #[serde(rename = "prvni_pomoc")]
    pub pedestrian_first_aid: String,
}

impl TrafficAccidentRow {
    pub fn granulate(self) -> (Overview, Vehicle, Pedestrian, VehicleOccupant) {
        let overview = Overview {
            global_id: self.global_id,
            object_id: self.object_id,
            police_accident_id: self.police_accident_id,
            basic_settlement_unit: self.basic_settlement_unit,
            district: self.district,
            longitude: self.longitude,
            latitude: self.latitude,
            sjstk_x_coordinate: self.sjstk_x_coordinate,
            sjstk_y_coordinate: self.sjstk_y_coordinate,
            date: self.date,
            time: self.time,
            day_of_the_week: self.day_of_the_week,
            time_of_day: self.time_of_day,
            alcohol: self.alcohol,
            alcohol_in_culprit: self.alcohol_in_culprit,
            accident_location: self.accident_location,
            accident_aftermath: self.accident_outcome,
            location: self.location,
            road_type: self.road_type,
            road_condition: self.road_condition,
            visibility: self.visibility,
            weather_condition: self.weather_condition,
            drivers_view: self.drivers_view,
            main_cause: self.main_cause,
            detailed_cause: self.detailed_cause,
            collision: self.collision,
            caused_by: self.caused_by,
            light_injuries_count: self.light_injuries_count,
            heavy_injuries_count: self.heavy_injuries_count,
            death_count: self.death_count,
            damage_in_czk: self.damage_in_czk,
        };

        let vehicle = Vehicle {
            object_id: self.object_id,
            vehicle_id_within_accident: self.vehicle_id_within_accident,
            vehicle_type: self.vehicle_type.clone(),
            driver_condition: self.driver_condition,
            driver_influenced: self.driver_influenced,
            occupants: Vec::new(),
        };

        let pedestrian = Pedestrian {
            object_id: self.object_id,
            vehicle_type: self.vehicle_type,
            pedestrian_behavior: self.pedestrian_behavior,
            pedestrian_category: self.pedestrian_category,
            pedestrian_outcome: self.pedestrian_outcome,
            pedestrian_condition: self.pedestrian_condition,
            pedestrian_situation: self.pedestrian_situation,
            pedestrian_first_aid: self.pedestrian_first_aid,
        };

        let vehicle_occupant = VehicleOccupant {
            object_id: self.object_id,
            person: self.person,
            person_detail: self.person_clarification,
            gender: self.gender,
            age_group: self.age_group,
            had_light_injuries: self.had_light_injuries,
            had_heavy_injuries: self.had_heavy_injuries,
            had_died: self.had_died,
            had_died_after_24_hours: self.had_died_after_24_hours,
            days_after_accident_till_death: self.days_after_accident_till_death,
            outcome: self.outcome,
        };

        (overview, vehicle, pedestrian, vehicle_occupant)
    }

    pub fn is_pedestrian(&self) -> bool {
        self.person.as_str() == "chodec"
    }

    pub fn is_occupant(&self) -> bool {
        !self.person.is_empty() && !self.is_pedestrian()
    }
}
