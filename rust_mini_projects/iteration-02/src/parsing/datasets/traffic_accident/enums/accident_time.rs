use chrono::NaiveTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AccidentTime {
    Unknown,
    Hour(u8),
    Exact(NaiveTime),
}

impl AccidentTime {
    pub fn unknown_time() -> u32 {
        2560
    }
}
