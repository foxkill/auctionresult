//! # Treasury
//! 
//! 
use serde::{de::{self, Unexpected}, Deserialize, Deserializer};
use serde_json::Value;

mod treasury_type;

/// Deserialize bool from String with custom value mapping
fn bool_from_string<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    match String::deserialize(deserializer)?.as_ref() {
        "Yes" => Ok(true),
        "No" => Ok(false),
        other => Err(de::Error::invalid_value(
            Unexpected::Str(other),
            &"Yes or No",
        )),
    }
}

fn de_f64_or_string_as_f64<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Option<f64>, D::Error> {
  Ok(match Value::deserialize(deserializer)? {
    Value::String(s) => s.parse().ok(),
    Value::Number(num) => num.as_f64(),
    _ => None,
  })
}
// fn de_f64_or_string_as_f64<'de, D: Deserializer<'de>>(deserializer: D) -> Result<f64, D::Error> {
//   Ok(match Value::deserialize(deserializer)? {
//     Value::String(s) => s.parse().map_err(de::Error::custom)?,
//     Value::Number(num) => num.as_f64().ok_or_else(|| de::Error::custom("Invalid number"))?,
//     _ => return Err(de::Error::custom("wrong type")),
//   })
// }
#[allow(dead_code)]
#[derive(Debug, Default, Deserialize)]
pub struct Treasury<'a> {
    cusip: &'a str,
    #[serde(rename(deserialize = "type"))]
    treasury_type: &'a str,
    term: &'a str,
    #[serde(rename="securityTerm")]
    security_term : &'a str,
    #[serde(deserialize_with = "bool_from_string")]
    reopening: bool,
    #[serde(rename="issueDate")]
    issue_date: &'a str,
    #[serde(rename="maturityDate")]
    maturity_date: &'a str,
    #[serde(rename="highYield")]
    high_yield: f64,
    #[serde(rename="interestRate")]
    interest_rate: f64,
    #[serde(rename="highDiscountRate")]
    high_discount_rate: f64,
    #[serde(rename="highInvestmentRate")]
    high_investment_rate:  f64,
    #[serde(rename="asResults")]
    as_results: bool,
    #[serde(rename="mlFileNameCompetitiveResult")]
    ml_filename_competitive_results: &'a str,
    #[serde(rename="primaryDealerAccepted")]
    primary_dealer_dccepted: f64,
    #[serde(rename="bidToCoverRatio")]
    bid_to_cover_ratio: f32,
    #[serde(rename="totalAccepted")]
    total_accepted: f32,
}

#[cfg(test)]
mod tests;