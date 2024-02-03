//! # The Get Module
use crate::{treasury::TreasuryAccess, Treasury};
use reqwest::blocking::Client;

#[cfg(test)]
static URL: &str = "";

#[cfg(not(test))]
static URL: &str = "https://www.treasurydirect.gov/TA_WS";

static TREASURIES_URL: &str = "/securities/search";

pub struct Get {
    cusip: String,
    url: String,
}

impl TreasuryAccess for Get {
    fn get(&self) -> Vec<Treasury> {
        let client = Client::new();

        let url = self.url();
        let Ok(resp) = client.get(url).send() else {
            return vec![Treasury::default()];
        };

        // resp.json().unwrap_or(vec![Treasury::default()])
        let j = resp.json().unwrap();

        j
    }

    fn url(&self) -> String {
        format!(
            "{}{}?cusip={}&format=json",
            self.url, TREASURIES_URL, self.cusip
        )
    }
}

impl Get {
    pub fn new(cusip: &str) -> Self {
        Self {
            cusip: cusip.to_owned(),
            url: URL.to_string(),
        }
    }

    fn set_host(&mut self, url: impl Into<String>) {
        self.url = url.into();
    }
}

#[cfg(test)]
mod tests {
    use mockito::Matcher;
    use super::*;

    const CUSIP: &str = "91282CJQ5";
    #[test]
    fn url() {
        let g = Get::new(CUSIP);
        assert_eq!(
            format!("{}?cusip={}&format=json", TREASURIES_URL, CUSIP),
            g.url()
        );
    }

    #[test]
    fn get() {
        let mut server = mockito::Server::new();
        let host = server.url();

        let mut g = Get::new(CUSIP);

        g.set_host(host);

        server
            .mock("GET", TREASURIES_URL)
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("cusip".into(), CUSIP.into()),
                Matcher::UrlEncoded("format".into(), "json".into()),
            ]))
            .with_body(api_single_item())
            .create();

        let v = g.get();
        assert_eq!(CUSIP, v[0].cusip());
    }
}
