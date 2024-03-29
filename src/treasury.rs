//! # Treasury

#[allow(dead_code)]
// make usable.
mod deserializer;

// Make visible
pub mod error;
pub mod load;
pub mod print;
pub mod security_type;

use chrono::NaiveDateTime;
use serde::Deserialize;

// Own serializers for special treasury specific types.
use deserializer::bool_from_string;
use deserializer::f64_from_string;

// Re-Export
pub use error::AuctionResultError;
pub use load::load;
pub use security_type::SecurityType;

const DEFAULT_SECURITY_DATE_FORMAT: &str = "%m/%d/%Y";

#[allow(dead_code)]
#[derive(Debug, Default, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Treasury {
    // 1
    cusip: String,
    // 2
    issue_date: NaiveDateTime,
    // 3
    #[serde(rename(deserialize = "type"))]
    security_type: SecurityType,
    // 4
    security_term: String,
    // 5
    maturity_date: NaiveDateTime,
    // 6
    #[serde(deserialize_with = "f64_from_string")]
    interest_rate: f64,
    // 28
    #[serde(deserialize_with = "f64_from_string")]
    bid_to_cover_ratio: f64,
    // 35
    #[serde(deserialize_with = "f64_from_string")]
    competitive_accepted: f64,
    // 42
    #[serde(deserialize_with = "f64_from_string")]
    direct_bidder_accepted: f64,
    // 53
    #[serde(deserialize_with = "f64_from_string")]
    high_discount_rate: f64,
    // 54
    #[serde(deserialize_with = "f64_from_string")]
    high_investment_rate: f64,
    // 57
    #[serde(deserialize_with = "f64_from_string")]
    high_yield: f64,
    // 59
    #[serde(deserialize_with = "f64_from_string")]
    indirect_bidder_accepted: f64,
    // 84
    original_security_term: String,
    // 90
    #[serde(deserialize_with = "f64_from_string")]
    primary_dealer_accepted: f64,
    // 92
    #[serde(deserialize_with = "bool_from_string")]
    reopening: bool,
    // 103
    term: String,
    // 106
    #[serde(deserialize_with = "f64_from_string")]
    total_accepted: f64,
}
// serializ_dt() must be implemented.
// #[serde(serialize_with = "serialize_dt", skip_serializing_if  = "Option::is_none")]

impl Treasury {
    pub fn new() -> Self {
        Treasury::default()
    }

    /// Return the CUSIP number of the treasury.
    pub fn cusip(&self) -> &str {
        self.cusip.as_str()
    }

    /// Return if an auction is re-openend.
    pub fn is_reopening(&self) -> bool {
        self.reopening
    }

    /// Return the field headers to construct the output of the treasury.
    pub fn get_fields<'a>(&self) -> Vec<&'a str> {
        let mut fields = vec![
            "Security Term",
            "CUSIP",
            "Reopening",
            "Security Type",
            "Issue Date",
            "Maturity Date",
            "Bid To Cover",
            "Dealers %",
            "Directs %",
            "Indirects %",
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

    /// Return the default date format used in the print and vprint methods.
    pub fn get_default_date_fmt() -> &'static str {
        DEFAULT_SECURITY_DATE_FORMAT
    }

    /// Return the term string of the treasury structure.
    pub fn get_term(&self) -> &str {
        self.term.as_str()
    }

    /// Return the security term string of the treasury structure.
    pub fn get_security_term(&self) -> &str {
        self.security_term.as_str()
    }

    /// Return the original security term string of the treasury structure.
    pub fn get_original_security_term(&self) -> &str {
        self.original_security_term.as_str()
    }

    /// Get the security type of the treasury.
    pub fn get_security_type(&self) -> SecurityType {
        self.security_type.to_owned()
    }

    /// Calculate the percentage of debt that was accepted by primary dealers.
    pub fn get_percentage_debt_purchased_by_dealers(&self) -> f64 {
        (self.primary_dealer_accepted / self.competitive_accepted) * 100.0
    }

    /// Calculate the percentage of debt that was accepted by direct bidders.
    pub fn get_percentage_debt_purchased_by_directs(&self) -> f64 {
        (self.direct_bidder_accepted / self.competitive_accepted) * 100.0
    }

    /// Calculate the percentage of debt that was accepted by indirect bidders.
    pub fn get_percentage_debt_purchased_by_indirects(&self) -> f64 {
        (self.indirect_bidder_accepted / self.competitive_accepted) * 100.0
    }

    /// Returns the get bid to cover ratio of this [`Treasury`].
    pub fn get_bid_to_cover_ratio(&self) -> f64 {
        self.bid_to_cover_ratio
    }

    /// Returns the get high yield of this [`Treasury`].
    pub fn get_high_yield(&self) -> f64 {
        if self.security_type == SecurityType::Bill {
            self.high_discount_rate
        } else {
            self.high_yield
        }
    }

    /// Return the issue date.
    pub fn get_issue_date(&self) -> NaiveDateTime {
        self.issue_date
    }

    /// Returns the get high discount rate of this [`Treasury`].
    pub fn get_interest_rate(&self) -> f64 {
        if self.security_type == SecurityType::Bill {
            self.high_investment_rate
        } else {
            self.interest_rate
        }
    }
}

/// Define a convienience type for the return values.
pub type AuctionResult<T> = std::result::Result<T, AuctionResultError>;

/// Define an opaque type for returning a treasury list.
pub type Treasuries = Vec<Treasury>;

/// The trait that all auction result modules must implement.
pub trait TreasuryAccess<T> {
    fn get(&self) -> AuctionResult<T>;
    fn url(&self) -> String;
}

#[cfg(test)]
mod tests;
