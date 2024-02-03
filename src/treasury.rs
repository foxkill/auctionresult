//! # Treasury
//! 
//! 
// Make usable for this module.
mod deserializer;
// Make visible
pub mod treasury_type;

use serde::Deserialize;
use deserializer::f64_from_string;
use deserializer::bool_from_string;

// Re-Export
pub use treasury_type::TreasuryType;

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
    issue_date: String,
    maturity_date: String,
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

impl Treasury {
    pub fn cusip(&self) -> String {
        self.cusip.to_string()
    }    
}

#[cfg(test)]
mod tests;