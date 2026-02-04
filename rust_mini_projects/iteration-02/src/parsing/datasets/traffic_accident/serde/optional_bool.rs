use serde::{Deserialize, Deserializer, Serializer};

pub fn serialize<S>(is_injured: &Option<bool>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(match is_injured {
        None => "",
        Some(is_injured) => {
            if *is_injured {
                "1"
            } else {
                "0"
            }
        }
    })
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    match s.as_str() {
        "1" => Ok(Some(true)),
        "0" => Ok(Some(false)),
        _other => Ok(None),
    }
}
