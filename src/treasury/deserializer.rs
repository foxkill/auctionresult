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

// const FORMAT: &str = "%Y-%m-%d %H:%M:%S";

// pub fn serialize<S>(date: &Option<DateTime<Utc>>, serializer: S) -> Result<S::Ok, S::Error>
// where
//     S: Serializer,
// {
//     match date.is_some() {
//         true => {
//             let s = format!("{}", date.as_ref().unwrap().format(FORMAT));
//             serializer.serialize_str(&s)
//         }
//         false => serializer.serialize_none(),
//     }
// }

// "{\"a\":2.45,\"date\":\"2022-12-16 16:40:36\"}"
// TestStruct { a: 2.45, date: Some(2022-12-16T16:40:36Z) }

// "{\"a\":2.45,\"date\":null}"
// TestStruct { a: 2.45, date: None }

// fn f16des3<'de, D>(des: D) -> Result<f16, D::Error>
// where
//     D: Deserializer<'de>,
// {
//     f64::deserialize(des).and_then(|fv| Ok(f16::from_f64(fv)))
// }
