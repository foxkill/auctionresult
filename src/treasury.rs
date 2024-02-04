//! # Treasury
//! 
//! 

// make usable.
mod deserializer;
// Make visible
pub mod treasury_type;
pub mod load;
pub mod print;

use chrono::NaiveDateTime;
use serde::Deserialize;

use deserializer::f64_from_string;
use deserializer::bool_from_string;

// Re-Export
pub use treasury_type::TreasuryType;
pub use load::load;

#[allow(dead_code)]
#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Treasury {
    cusip: String,
    #[serde(rename(deserialize = "type"))]
    treasury_type: TreasuryType,
    term: String,
    security_term : String,
    #[serde(deserialize_with = "bool_from_string")]
    reopening: bool,
    issue_date: NaiveDateTime,
    maturity_date: NaiveDateTime,
    #[serde(deserialize_with = "f64_from_string")]
    high_yield: f64,
    #[serde(deserialize_with = "f64_from_string")]
    interest_rate: f64,
    #[serde(deserialize_with = "f64_from_string")]
    high_discount_rate: f64,
    #[serde(deserialize_with = "f64_from_string")]
    high_investment_rate:  f64,
    #[serde(deserialize_with = "f64_from_string")]
    primary_dealer_accepted: f64,
    #[serde(deserialize_with = "f64_from_string")]
    bid_to_cover_ratio: f64,
    #[serde(deserialize_with = "f64_from_string")]
    total_accepted: f64,
}
// serializ_dt() must be implemented.
// #[serde(serialize_with = "serialize_dt", skip_serializing_if  = "Option::is_none")]

impl Treasury {
    pub fn cusip(&self) -> String {
        self.cusip.to_string()
    }    
}

pub trait TreasuryAccess {
    fn get(&self) -> Vec<Treasury>;
    fn url(&self) -> String;
}

#[cfg(test)]
mod tests;