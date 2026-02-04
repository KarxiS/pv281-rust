use crate::parsing::datasets::traffic_accident::TrafficAccidentRow;
use crate::parsing::datasets::traffic_accident::enums::PersonalOutcome;

#[derive(Debug, Clone, PartialEq)]
pub struct Pedestrian {
    pub object_id: isize,
    pub vehicle_type: String,
    pub pedestrian_behavior: String,
    pub pedestrian_category: String,
    pub pedestrian_outcome: Option<PersonalOutcome>,
    pub pedestrian_condition: String,
    pub pedestrian_situation: String,
    pub pedestrian_first_aid: String,
}

impl From<TrafficAccidentRow> for Pedestrian {
    fn from(row: TrafficAccidentRow) -> Self {
        Self {
            object_id: row.object_id,
            vehicle_type: row.vehicle_type,
            pedestrian_behavior: row.pedestrian_behavior,
            pedestrian_category: row.pedestrian_category,
            pedestrian_outcome: row.pedestrian_outcome,
            pedestrian_condition: row.pedestrian_condition,
            pedestrian_situation: row.pedestrian_situation,
            pedestrian_first_aid: row.pedestrian_first_aid,
        }
    }
}
