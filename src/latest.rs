//! # Module for retrieving the lastest auction results.
//
//
use crate::Treasury;
use crate::treasury::treasury_type::TreasuryType;

static AUCTIONED_URL: &str = "https://www.treasurydirect.gov/TA_WS/securities/auctioned";


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

    fn load(&self) -> usize {
        0
    }

    fn url(&self) -> String {
        String::from(AUCTIONED_URL)
    }
}

#[cfg(test)]
mod tests;