//! # The Get Module
use crate::{treasury::{load::load, TreasuryAccess}, Treasury};

#[cfg(test)]
static URL: &str = "";

#[cfg(not(test))]
static URL: &str = "https://www.treasurydirect.gov/TA_WS";

static TREASURIES_URL: &str = "/securities/search";

pub struct Get {
    cusip: String,
    url: String,
}

pub fn get(cusip: &str) -> Get {
    Get::new(cusip)
}

impl TreasuryAccess for Get {
    fn get(&self) -> Vec<Treasury> {
        let def = vec![Treasury::default()];
        let url = self.url();

        let handle = load(url);
        handle.join().unwrap_or(def)
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
            cusip: cusip.to_string(),
            url: URL.to_string(),
        }
    }

    #[cfg(test)]
    fn set_host(&mut self, url: impl Into<String>) {
        self.url = url.into();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::fixture::api_single_item;
    use mockito::Matcher;

    const CUSIP: &str = "91282CJQ5";
    #[test]
    fn it_should_correctly_build_an_url() {
        // if cfg!(target_os = "windows") {
        let g = Get::new(CUSIP);
        assert_eq!(
            format!("{}?cusip={}&format=json", TREASURIES_URL, CUSIP),
            g.url()
        );
    }

    #[test]
    fn it_should_correctly_call_get() {
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
