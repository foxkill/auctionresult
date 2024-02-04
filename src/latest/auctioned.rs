//! # Module for retrieving the lastest auction results.
// #![allow(unused)]
use crate::treasury::{load, treasury_type::TreasuryType, Treasury, TreasuryAccess};

static AUCTIONED_URL: &str = "https://www.treasurydirect.gov/TA_WS/securities/auctioned";

#[derive(Debug, Default, PartialEq)]
pub struct Latest {
    days: usize,
    treasury_type: TreasuryType,
}

impl TreasuryAccess for Latest {
    fn get(&self) -> Vec<Treasury> {
        let url = self.url();
        let handle = load(url);

        let response = handle.join().unwrap_or(
            vec![Treasury::default()]
        );

        response
    }

    fn url(&self) -> String {
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

impl Latest {
    pub fn new(treasury_type: TreasuryType, days: usize) -> Self {
        Self {
            days: if days == 0 { 7 } else { days },
            treasury_type,
        }
    }

    pub fn days(&self) -> usize {
        self.days
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_return_the_lastest_auctions() {
        let latest = Latest::new(TreasuryType::Bill, 0).get();
        println!("{latest:#?}");
    }
}
