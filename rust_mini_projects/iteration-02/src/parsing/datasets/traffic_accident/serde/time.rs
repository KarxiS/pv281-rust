use chrono::{NaiveTime, Timelike};
use serde::de::{Error, Unexpected};
use serde::{Deserialize, Deserializer, Serializer};

use crate::parsing::datasets::traffic_accident::enums::AccidentTime;

pub fn serialize<S>(time: &AccidentTime, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match time {
        AccidentTime::Unknown => {
            serializer.serialize_str(&AccidentTime::unknown_time().to_string())
        }
        AccidentTime::Hour(hour) => serializer.serialize_str(&format!("{}60", hour)),
        AccidentTime::Exact(t) => serializer.serialize_str(&format!("{}{}", t.hour(), t.minute())),
    }
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<AccidentTime, D::Error>
where
    D: Deserializer<'de>,
{
    match u32::deserialize(deserializer)? {
        unknown if unknown == AccidentTime::unknown_time() => Ok(AccidentTime::Unknown),
        hour_only if minutes(hour_only) == 60 => match hours(hour_only).try_into() {
            Ok(hours) => Ok(AccidentTime::Hour(hours)),
            Err(_) => Err(Error::invalid_type(
                Unexpected::Unsigned(hour_only as u64),
                &"valid HH60 timestamp",
            )),
        },
        exact => {
            let maybe_time = NaiveTime::from_hms_opt(hours(exact), minutes(exact), 0);
            match maybe_time {
                None => Err(Error::invalid_value(
                    Unexpected::Unsigned(exact as u64),
                    &"valid HHMM timestamp",
                )),
                Some(time) => Ok(AccidentTime::Exact(time)),
            }
        }
    }
}

fn hours(time: u32) -> u32 {
    time / 100
}

fn minutes(time: u32) -> u32 {
    time % 100
}
