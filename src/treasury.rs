//! # Treasury
//!
//!

// make usable.
mod deserializer;

// Make visible
pub mod error;
pub mod load;
pub mod print;
pub mod security_type;

use chrono::NaiveDateTime;
use serde::Deserialize;

use deserializer::bool_from_string;
use deserializer::f64_from_string;

// Re-Export
pub use load::load;
pub use security_type::SecurityType;
pub use error::AuctionResultError;


const DEFAULT_SECURITY_DATE_FORMAT: &str = "%m/%d/%Y";

#[allow(dead_code)]
#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Treasury {
    cusip: String,
    #[serde(rename(deserialize = "type"))]
    security_type: SecurityType,
    term: String,
    security_term: String,
    original_security_term: String,
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
    high_investment_rate: f64,
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
    pub fn new() -> Self {
        Treasury::default()
    }

    pub fn cusip(&self) -> String {
        self.cusip.to_string()
    }

    pub fn get_fields<'a>(&self) -> Vec<&'a str> {
        let mut fields = vec![
            "Security Term",
            "CUSIP",
            "Reopening",
            "Security Type",
            "Issue Date",
            "Maturity Date",
            "Bid To Cover",
            "Dealers"
        ];

        if self.security_type == SecurityType::Bill {
            fields.push("High Rate");
            fields.push("Investment Rate");
        } else {
            fields.push("High Yield");
            fields.push("Interest Rate");
        }

        fields
    }

    pub fn get_default_date_fmt() -> &'static str {
        DEFAULT_SECURITY_DATE_FORMAT
    }

    pub fn get_term(&self) -> String {
        self.term.to_owned()
    }

    pub fn get_security_term(&self) -> String {
        self.security_term.to_owned()
    }
    
    pub fn get_original_security_term(&self) -> String {
        self.original_security_term.to_owned()
    }
}

pub type AuctionResult<T> = std::result::Result<T, AuctionResultError>;
pub type Treasuries = Vec<Treasury>;

pub trait TreasuryAccess<T> {
    fn get(&self) -> AuctionResult<T>;
    fn url(&self) -> String;
}

#[cfg(test)]
mod tests;
