//! # Module for retrieving the lastest auction results.
#[cfg(test)]
use mockall::automock;
// #![allow(unused)]
use reqwest::blocking::Client;

use crate::treasury::{treasury_type::TreasuryType, Treasury, TreasuryAccess};

static AUCTIONED_URL: &str = "https://www.treasurydirect.gov/TA_WS/securities/auctioned";

#[derive(Debug, Default, PartialEq)]
pub struct Latest {
    days: usize,
    treasury_type: TreasuryType,
}

#[cfg_attr(test, automock)]
impl TreasuryAccess for Latest {
    fn get(&self) -> Vec<Treasury> {
        let client = Client::new();
        let Ok(resp) = client.get(self.url()).send() else {
            return vec![Treasury::default()];
        };

        resp.json().unwrap_or(vec![Treasury::default()])
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
    fn get_lastest_auctions() {
        let mut latest = MockLatest::new();
        latest
            .expect_get()
            .returning(|| vec![Treasury::default()]);

        let result = latest.get();
        println!("{result:#?}");
    }
}
