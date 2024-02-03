//! # The Get Module

use reqwest::blocking::Client;
use crate::Treasury;

static TREASURIES_URL: &str = "https://www.treasurydirect.gov/TA_WS/securities/search";

pub struct Get {
    cusip: String,
}

impl Get {
    pub fn new(cusip: &str) -> Self  {
        Self { cusip: cusip.to_owned() }
    }

    pub fn get(&self) -> Treasury {
        let client = Client::new();
        let Ok(resp) = client.get(self.url()).send() else {
            return Treasury::default();
        };

        let mut items: Vec<Treasury> = resp.json().unwrap_or(vec![Treasury::default()]);

        items.remove(0)
    }

    fn url(&self) -> String {
        format!("{TREASURIES_URL}?cusip={}&format=json", self.cusip)
    }
}

#[cfg(test)]
mod tests {
    use crate::get::{Get, TREASURIES_URL};

    const CUSIP: &str = "91282CJQ5";
    #[test]
    fn url() {
        let g = Get::new(CUSIP);
        assert_eq!(format!("{}?cusip={}&format=json", TREASURIES_URL, CUSIP), g.url());
    }

    #[test]
    fn get() {
        let g = Get::new(CUSIP);
        let v = g.get();
        assert_eq!(CUSIP, v.cusip());
    }
}