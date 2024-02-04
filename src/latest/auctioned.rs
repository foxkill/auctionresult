//! # Module for retrieving the lastest auction results.
// #![allow(unused)]
use crate::treasury::{load, treasury_type::TreasuryType, Treasury, TreasuryAccess};

#[cfg(not(test))]
static AUCTIONED_URL: &str = "https://www.treasurydirect.gov/TA_WS/securities/auctioned";
#[cfg(test)]
static AUCTIONED_URL: &str = "/securities/auctioned";

#[derive(Debug, Default, PartialEq)]
pub struct Latest {
    days: usize,
    treasury_type: TreasuryType,
    #[cfg(test)]
    host: String,
}

impl TreasuryAccess for Latest {
    fn get(&self) -> Vec<Treasury> {
        let url = self.url();
        let handle = load(url);

        handle.join().unwrap_or(
            vec![Treasury::default()]
        )
    }

    fn url(&self) -> String {
        let mut url = String::from(AUCTIONED_URL);
        if self.treasury_type != TreasuryType::Null {
            #[cfg(test)]
            url.insert_str(0,&self.host);

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
            #[cfg(test)]
            host: "".to_owned(),
        }
    }

    pub fn days(&self) -> usize {
        self.days
    }
}

#[cfg(test)]
mod tests {
    use mockito::Matcher;

    use crate::tests::fixture::api_multiple_items;

    use super::*;

    #[test]
    fn it_should_return_the_lastest_auctions() {
        let mut server = mockito::Server::new();
        let mut latest = Latest::new(TreasuryType::Bill, 8);

        latest.host = server.url();

        server
            .mock("GET", AUCTIONED_URL)
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("type".into(), TreasuryType::Bill.to_string()),
                Matcher::UrlEncoded("days".into(), "8".into()),
            ]))
            .with_body(api_multiple_items())
            .create();

        let response = latest.get();
        assert_eq!(11, response.len());
    }
}
