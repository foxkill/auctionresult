//! # The Get Module
extern crate cusip;
use cusip as cu;
use crate::treasury::{load::load, AuctionResult, AuctionResultError, Treasuries, TreasuryAccess};

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

impl TreasuryAccess<Treasuries> for Get {
    fn get(&self) -> AuctionResult<Treasuries> {
        // Check the cusip number, before using it.
        if !cu::validate(&self.cusip) {
            return Err(AuctionResultError::ParseError);
        }

        let url = self.url();
        let response = load(url)?;

        let treasuries: Treasuries = response.json()?;
        Ok(treasuries)
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

        let v = g.get().unwrap();
        assert_eq!(CUSIP, v[0].cusip());
    }

    #[test]
    fn it_should_correctly_handle_invalid_response() {
        let mut server = mockito::Server::new();
        let host = server.url();

        let mut g = self::get(CUSIP);

        g.set_host(host);

        server
            .mock("GET", TREASURIES_URL)
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("cusip".into(), CUSIP.into()),
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
        let mut g = self::get(CUSIP);
        // Make sure that nothing is listening on that port.
        g.set_host("https://localhost:12000");
        let result = g.get();
        println!("{result:#?}");
        assert!(result.is_err());
    }
}
