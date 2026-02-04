use serde::de::{Error, Unexpected};
use serde::{self, Deserialize, Deserializer, Serializer};

pub fn serialize<S>(is_injured: &bool, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(if *is_injured { "1" } else { "0" })
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    match s.as_str() {
        "1" => Ok(true),
        "0" => Ok(false),
        other => Err(Error::invalid_value(Unexpected::Str(other), &"1")),
    }
}
