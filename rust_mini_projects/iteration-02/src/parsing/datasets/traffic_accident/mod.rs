pub mod enums;
mod serde;
mod structs;
mod traffic_accident_row;

pub use structs::{TrafficAccident, Vehicle};
pub use traffic_accident_row::TrafficAccidentRow;
