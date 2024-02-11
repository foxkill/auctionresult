//! # Module for retrieving the lastest auction results.
// #![allow(unused)]
use crate::{
    tenor::Tenor,
    treasury::{load, security_type::SecurityType, AuctionResult, Treasuries, TreasuryAccess},
};

#[cfg(not(test))]
static AUCTIONED_URL: &str = "https://www.treasurydirect.gov/TA_WS/securities/auctioned";
#[cfg(test)]
pub (crate) static AUCTIONED_URL: &str = "/securities/auctioned";

/// Descriptor of the Latest module.
#[derive(Debug, Default, PartialEq)]
pub struct Latest {
    days: usize,
    security_type: SecurityType,
    tenor: Tenor,
    #[cfg(test)]
    host: String,
}

impl TreasuryAccess<Treasuries> for Latest {
    fn get(&self) -> AuctionResult<Treasuries> {
        let url = self.url();
        let response = load(url)?;

        let treasuries: Treasuries = response.json()?;
        let compare_to = self.tenor.to_string();

        Ok(if self.tenor.is_empty() {
            treasuries
        } else {
            treasuries
                .into_iter()
                .filter(|t| { 
                    *t.get_term() == compare_to ||
                    *t.get_security_term() == compare_to ||
                    *t.get_original_security_term() == compare_to
                })
                .collect::<Treasuries>()
        })
    }

    fn url(&self) -> String {
        let mut url = String::from(AUCTIONED_URL);

        #[cfg(test)]
        url.insert_str(0, &self.host);

        if self.security_type != SecurityType::Null {
            url.push_str("?type=");
            url.push_str(&self.security_type.to_string());
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
    /// Create a new Latest module from the given security type, ie [`Bond`, `Note`], etc.,
    /// the number of [`days`] to look back and a filter for the tenor, which can be
    /// for example: [`10y`, `10-Y`, `10-years`] or any other specifier of a time 
    /// range. If the number of [`days`] is equal to [`0`] the default of [`7`] 
    /// days is applied.
    pub fn new(treasury_type: SecurityType, days: usize, tenor: Tenor) -> Self {
        Self {
            days: if days == 0 { 7 } else { days },
            security_type: treasury_type,
            tenor,
            #[cfg(test)]
            host: "".to_owned(),
        }
    }

    pub fn get_days(&self) -> usize {
        self.days
    }

    pub fn get_security_type(&self) -> SecurityType {
        self.security_type.to_owned()
    }

    #[cfg(test)]
    pub fn set_host(&mut self, host: String) {
        self.host = host
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
        let mut latest = Latest::new(SecurityType::Bill, 8, Tenor::default());

        latest.host = server.url();

        server
            .mock("GET", AUCTIONED_URL)
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("type".into(), latest.get_security_type().to_string()),
                Matcher::UrlEncoded("days".into(), latest.get_days().to_string()),
            ]))
            .with_body(api_multiple_items())
            .create();

        let response = latest.get().unwrap();
        assert_eq!(11, response.len());
    }

    #[test]
    pub fn it_should_correctly_apply_tenor_filter() {
        let sectype = SecurityType::Null;
        let mut server = mockito::Server::new();
        let mut latest = Latest::new(sectype.clone(), 8, Tenor::parse("2y").unwrap());

        latest.host = server.url();

        server
            .mock("GET", AUCTIONED_URL)
            .match_query(Matcher::UrlEncoded(
                "days".into(),
                "8".into(),
            ))
            .with_body(api_multiple_items())
            .create();

        let response = latest.get().unwrap();
        assert_eq!(2, response.len());
    }
}
