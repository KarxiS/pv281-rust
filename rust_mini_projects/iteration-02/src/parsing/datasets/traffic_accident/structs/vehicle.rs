use crate::parsing::datasets::traffic_accident::TrafficAccidentRow;
use crate::parsing::datasets::traffic_accident::enums::{DriverCondition, DriverInfluenced};
use crate::parsing::datasets::traffic_accident::structs::vehicle_occupant::VehicleOccupant;

#[derive(Debug, Clone, PartialEq)]
pub struct Vehicle {
    pub object_id: isize,
    pub vehicle_id_within_accident: Option<i64>,
    pub vehicle_type: String,
    pub driver_condition: Option<DriverCondition>,
    pub driver_influenced: Option<DriverInfluenced>,
    pub occupants: Vec<VehicleOccupant>,
}

impl From<TrafficAccidentRow> for Vehicle {
    fn from(row: TrafficAccidentRow) -> Self {
        Self {
            object_id: row.object_id,
            vehicle_id_within_accident: row.vehicle_id_within_accident,
            vehicle_type: row.vehicle_type,
            driver_condition: row.driver_condition,
            driver_influenced: row.driver_influenced,
            occupants: Vec::new(),
        }
    }
}
