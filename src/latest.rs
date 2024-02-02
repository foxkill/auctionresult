//! # Module for retrieving the lastest auction results.
//
//
use crate::Treasury;
use crate::treasury::treasury_type::TreasuryType;

static AUCTIONED_URL: &str = "https://www.treasurydirect.gov/TA_WS/securities/auctioned";

#[cfg(test)]
mod tests;

#[derive(Debug, Default)]
pub struct Latest {
    days: usize,
    treasury_type: TreasuryType,
}

impl Latest {
    pub fn new(treasury_type: TreasuryType, days: usize) -> Self {
        Self { 
            days: if days == 0 { 7 } else { days },
            treasury_type
        }
    }

    pub fn get(&self) -> Vec<Treasury> {
        vec![Treasury::default()]
    }

    pub fn days(&self) -> usize {
        self.days
    }

    pub fn load(&self) -> usize {
        0
    }

    pub fn url(&self) -> String {
        let mut url = String::from(AUCTIONED_URL);
        if self.treasury_type != TreasuryType::Null {
            url.push_str("?type=");
            url.push_str(&self.treasury_type.to_string());
            url.push_str("&days=");
            url.push_str(&self.days.to_string());
        } else {
            url.push_str("?days=");
            url.push_str(&self.days.to_string());
        }
        url
    }
}

