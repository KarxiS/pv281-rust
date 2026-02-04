pub use crate::parsing::datasets::traffic_accident::structs::overview::Overview;
pub use crate::parsing::datasets::traffic_accident::structs::pedestrian::Pedestrian;
pub use crate::parsing::datasets::traffic_accident::structs::vehicle::Vehicle;
pub use crate::parsing::datasets::traffic_accident::structs::vehicle_occupant::VehicleOccupant;

mod overview;
mod pedestrian;
mod vehicle;
mod vehicle_occupant;

#[derive(Debug, Clone)]
pub struct TrafficAccident {
    pub overview: Overview,
    pub participant_vehicles: Vec<Vehicle>,
    pub participant_pedestrians: Vec<Pedestrian>,
}

impl PartialEq for TrafficAccident {
    fn eq(&self, other: &Self) -> bool {
        self.overview == other.overview
    }
}
