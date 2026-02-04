use crate::parsing::datasets::traffic_accident::TrafficAccidentRow;
use crate::parsing::datasets::traffic_accident::enums::{AgeGroup, PersonDetail, PersonalOutcome};

#[derive(Debug, Clone, PartialEq)]
pub struct VehicleOccupant {
    pub object_id: isize,
    pub person: String,
    pub person_detail: Option<PersonDetail>,
    pub gender: String,
    pub age_group: Option<AgeGroup>,

    pub had_light_injuries: bool,
    pub had_heavy_injuries: bool,
    pub had_died: bool,
    pub had_died_after_24_hours: Option<bool>,
    pub days_after_accident_till_death: Option<i64>,
    pub outcome: Option<PersonalOutcome>,
}

impl From<TrafficAccidentRow> for VehicleOccupant {
    fn from(row: TrafficAccidentRow) -> Self {
        Self {
            object_id: row.object_id,
            person: row.person,
            person_detail: row.person_clarification,
            gender: row.gender,
            age_group: row.age_group,
            had_light_injuries: row.had_light_injuries,
            had_heavy_injuries: row.had_heavy_injuries,
            had_died: row.had_died,
            had_died_after_24_hours: row.had_died_after_24_hours,
            days_after_accident_till_death: row.days_after_accident_till_death,
            outcome: row.outcome,
        }
    }
}
