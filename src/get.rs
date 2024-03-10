//! # The Get Module
extern crate cusip;
use crate::treasury::{load::load, AuctionResult, AuctionResultError, Treasuries, TreasuryAccess};
use cusip as cu;

#[cfg(test)]
static HOST: &str = "";

#[cfg(not(test))]
static HOST: &str = "https://www.treasurydirect.gov/TA_WS";

// Use pub(create) for testing puposes.
pub(crate) static TREASURIES_URL: &str = "/securities/search";

pub struct Get {
    cusip: String,
    host: String,
}

impl TreasuryAccess<Treasuries> for Get {
    fn get(&self) -> AuctionResult<Treasuries> {
        // Check the cusip number, before using it.
        if !cu::validate(&self.cusip) {
            return Err(AuctionResultError::ParseCusip);
        }

        let url = self.url();
        let response = load(url)?;

        let treasuries: Treasuries = response.json()?;
        Ok(treasuries)
    }

    fn url(&self) -> String {
        format!(
            "{}{}?cusip={}&format=json",
            self.host, TREASURIES_URL, self.cusip
        )
    }
}

impl Get {
    /// Create a new Get module from a cusip number.
    pub fn new(cusip: impl Into<String>) -> Self {
        Self {
            cusip: cusip.into(),
            host: String::from(HOST),
        }
    }

    pub fn set_host(&mut self, host: impl Into<String>) {
        self.host = host.into();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::fixture::api_single_item;
    use mockito::Matcher;

    const TEST_CUSIP: &str = "91282CJQ5";
    
    #[test]
    fn it_should_correctly_build_an_url() {
        // if cfg!(target_os = "windows") {
        let g = Get::new(TEST_CUSIP);
        assert_eq!(
            format!("{}?cusip={}&format=json", TREASURIES_URL, TEST_CUSIP),
            g.url()
        );
    }
    #[test]
    fn it_should_handle_an_invalid_cusip() {
        // if cfg!(target_os = "windows") {
        let g = Get::new("x1");

        let result = g.get();
        assert!(result.is_err());
    }
    #[test]
    fn it_should_correctly_call_get() {
        let mut server = mockito::Server::new();
        let host = server.url();

        let mut g = Get::new(TEST_CUSIP);

        g.set_host(host);

        server
            .mock("GET", TREASURIES_URL)
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("cusip".into(), TEST_CUSIP.into()),
                Matcher::UrlEncoded("format".into(), "json".into()),
            ]))
            .with_body(api_single_item())
            .create();

        let v = g.get().unwrap();
        assert_eq!(TEST_CUSIP, v[0].cusip());
    }

    #[test]
    fn it_should_correctly_handle_invalid_response() {
        let mut server = mockito::Server::new();
        let host = server.url();

        let mut g = Get::new(TEST_CUSIP);

        g.set_host(host);

        server
            .mock("GET", TREASURIES_URL)
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("cusip".into(), TEST_CUSIP.into()),
                Matcher::UrlEncoded("format".into(), "json".into()),
            ]))
            .with_body(api_single_item())
            .with_status(500)
            .create();

        let result = g.get();
        // Handle the error via match.
        // if let AuctionResultError::RequestError(err) = result.as_ref().unwrap_err() {
        //     println!("{:?}", err.status());
        // }
        assert!(result.is_err());
    }

    #[test]
    fn it_should_correctly_handle_a_connection_error() {
        let mut g = Get::new(TEST_CUSIP);
        // Make sure that nothing is listening on that port.
        g.set_host("https://localhost:12000");
        let result = g.get();
        // println!("{result:#?}");
        assert!(result.is_err());
    }
}
