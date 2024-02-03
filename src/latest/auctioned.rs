//! # Module for retrieving the lastest auction results.
// #![allow(unused)]
use reqwest::blocking::Client;

use crate::treasury::{treasury_type::TreasuryType, Treasury};

static AUCTIONED_URL: &str = "https://www.treasurydirect.gov/TA_WS/securities/auctioned";

#[derive(Debug, Default, PartialEq)]
pub struct Latest {
    days: usize,
    treasury_type: TreasuryType,
}

impl Latest {
    pub fn new(treasury_type: TreasuryType, days: usize) -> Self {
        Self {
            days: if days == 0 { 7 } else { days },
            treasury_type,
        }
    }

    pub fn get(&self) -> Vec<Treasury> {
        let client = Client::new();
        let Ok(resp) = client.get(self.url()).send() else {
            return vec![Treasury::default()];
        };

        let items: Vec<Treasury> = resp.json().unwrap_or(vec![Treasury::default()]);

        items
    }

    pub fn days(&self) -> usize {
        self.days
    }

    pub fn load(&self) -> String {
        let client = Client::new();
        let Ok(resp) = client.get(self.url()).send() else {
            let from = String::from("");
            return from;
        };
        resp.text().unwrap_or("".to_owned())
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_lastest_auctions() {
        let latest = Latest::new(TreasuryType::Bill, 0);
        let result = latest.get();
        println!("{result:#?}");
    }
}
