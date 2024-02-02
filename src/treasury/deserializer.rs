//! # Special desericalizers for the treasery json schema.
//!
//!
use serde::{de, Deserialize};
use serde_json::Value;

/// Deserialize bool from String with custom value mapping
pub fn bool_from_string<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: de::Deserializer<'de>,
{
    match String::deserialize(deserializer)?.as_ref() {
        "Yes" => Ok(true),
        "No" => Ok(false),
        other => Err(de::Error::invalid_value(
            de::Unexpected::Str(other),
            &"Yes or No",
        )),
    }
}

pub fn f64_from_string<'de, D: de::Deserializer<'de>>(deserializer: D) -> Result<f64, D::Error> {
    Ok(match Value::deserialize(deserializer)? {
        Value::String(s) => s.parse::<f64>().unwrap_or(0.0),
        Value::Number(num) => num.as_f64().unwrap(),
        _ => return Err(de::Error::custom("wrong type")),
    })
}

// fn f16des3<'de, D>(des: D) -> Result<f16, D::Error>
// where
//     D: Deserializer<'de>,
// {
//     f64::deserialize(des).and_then(|fv| Ok(f16::from_f64(fv)))
// }
