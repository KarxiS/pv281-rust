use chrono::Weekday;
use serde::de::{Error, Unexpected};
use serde::{Deserialize, Deserializer, Serializer};

pub fn serialize<S>(day: &Weekday, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match day {
        Weekday::Mon => serializer.serialize_str("pondělí"),
        Weekday::Tue => serializer.serialize_str("úterý"),
        Weekday::Wed => serializer.serialize_str("středa"),
        Weekday::Thu => serializer.serialize_str("čtvrtek"),
        Weekday::Fri => serializer.serialize_str("pátek"),
        Weekday::Sat => serializer.serialize_str("sobota"),
        Weekday::Sun => serializer.serialize_str("neděle"),
    }
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<Weekday, D::Error>
where
    D: Deserializer<'de>,
{
    match String::deserialize(deserializer)?.as_str() {
        "pondělí" => Ok(Weekday::Mon),
        "úterý" => Ok(Weekday::Tue),
        "středa" => Ok(Weekday::Wed),
        "čtvrtek" => Ok(Weekday::Thu),
        "pátek" => Ok(Weekday::Fri),
        "sobota" => Ok(Weekday::Sat),
        "neděle" => Ok(Weekday::Sun),
        other => Err(Error::invalid_value(
            Unexpected::Str(other),
            &"any czech day of the week",
        )),
    }
}
