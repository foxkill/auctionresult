//! # The Quality Module
//!
//! measures the quality of an auction.

use crate::tenor::Tenor;
use crate::{
    treasury::{AuctionResult, AuctionResultError, Treasury, TreasuryAccess},
    Get, Latest,
};

/// The number of auctions to consider.
const LAST_AUCTIONS: usize = 5;

const BID_TO_COVER_RATIO_WEIGHT: f64 = 0.3;
const PRIMARY_DEALER_ACCEPTED_WEIGHT: f64 = 0.2;
const INDIRECT_BIDDER_WEIGHT: f64 = 0.1;
#[allow(dead_code)]
const WHEN_ISSUED_WEIGHT: f64 = 0.4;

#[derive(Default, Debug)]
pub struct Quality {
    cusip: String,
    lookback_auctions: usize,
    host: String,
}

// Auction bid-to-cover ratio = 2.3 (desired range: 2.0-3.0) - Score: 75
// Primary dealer participation = 65% (desired range: 50%-70%) - Score: 85
// Weighted score (assuming 70% weight for bid-to-cover, 30% for participation):
// 75 * 0.7 + 85 * 0.3 = 79.5

impl Quality {
    pub fn new(cusip: impl Into<String>, lookback_auctions: usize) -> Self {
        Self {
            cusip: cusip.into(),
            lookback_auctions: if lookback_auctions == 0 { LAST_AUCTIONS } else { lookback_auctions },
            host: "".to_owned(),
        }
    }

    pub fn cusip(&self) -> String {
        self.cusip.to_owned()
    }

    pub fn get(&mut self) -> AuctionResult<f64> {
        let mut get_command = Get::new(&self.cusip);

        if cfg!(test) {
            get_command.set_host(&self.host);
        }

        let treasuries = get_command.get();

        let treasuries = treasuries?;

        let Some(treasury) = treasuries.first() else {
            return Err(AuctionResultError::NoTreasury);
        };

        let quality = self.calculate_quality(treasury)?;

        Ok(quality)
    }

    #[allow(dead_code)]
    fn set_host(&mut self, host: impl Into<String>) {
        self.host = host.into();
    }

    /// Return the quality of the auction.
    //
    /// [`treasuries`]: The treasuries to consider.
    /// [`lookback_auctions`]: The _number_ of auctions to consider in the past.
    pub fn calculate_quality(
        &self,
        treasury: &Treasury
    ) -> AuctionResult<f64> {
        // Get the term of the treasury.
        let tenor = Tenor::parse(treasury.get_original_security_term())?;

        // Get the security type.
        let security_type = treasury.get_security_type();

        // Create the lastest module and search for auctions that were happening
        // before the given auction.
        let mut latest = Latest::new(security_type, 0, tenor);

        if cfg!(test) {
            latest.set_host(&self.host);
        }

        let lastest_auctions = latest.get()?;

        // Make sure we can look at the lastest X number of auctions.
        if lastest_auctions.len() < self.lookback_auctions + 1 {
            return Err(AuctionResultError::NoTreasury);
        }

        // Find the auction with given cusip.
        let Some(pos) = lastest_auctions
            .iter()
            .position(|s| *s.cusip() == treasury.cusip()) else {
            return Err(AuctionResultError::NoTreasury);
        };

        // Make sure we can look behind the lastest X number of auctions.
        if pos + self.lookback_auctions + 1 > lastest_auctions.len() {
            return Err(AuctionResultError::NoTreasury);
        }

        let treasuries = lastest_auctions
            .iter()
            .skip(pos + 1)
            .filter(|s| !s.is_reopening())
            .take(self.lookback_auctions)
            .collect::<Vec<&Treasury>>();

        let (
            sum_bid_to_cover,
            sum_primary_dealers,
            sum_indirect_bidders,
        ) = self.ratio_mean(&treasuries, self.lookback_auctions);

        let treasury = treasuries.first().unwrap();

        let diff_bid_to_cover =
            (treasury.get_bid_to_cover_ratio() - sum_bid_to_cover) * BID_TO_COVER_RATIO_WEIGHT;
        let diff_primary_dealers = (treasury.get_percentage_debt_purchased_by_dealers()
            - sum_primary_dealers)
            * -PRIMARY_DEALER_ACCEPTED_WEIGHT;
        let diff_indirect_bidders = (treasury.get_percentage_debt_purchased_by_indirects()
            - sum_indirect_bidders)
            * -INDIRECT_BIDDER_WEIGHT;

        // Round to 3 decimal places.
        let quality = ((diff_bid_to_cover + diff_primary_dealers + diff_indirect_bidders) * 1000.0).round() / 1000.0;

        Ok(quality)
    }

    /// Calculate the mean of the _bid to cover_, _primary dealers_ and _indirect_bidders_ of
    /// the last [`last_auctions`] auctions.
    fn ratio_mean(&self, treasuries: &[&Treasury], last_auctions: usize) -> (f64, f64, f64) {
        if treasuries.len() < last_auctions + 1 {
            return (0.0, 0.0, 0.0);
        }

        let mut sum_primary = 0.0;
        let mut sum_indirect = 0.0;
        let mut sum_bid_to_cover = 0.0;

        for treasury in treasuries.iter().skip(1).take(last_auctions) {
            sum_primary += treasury.get_percentage_debt_purchased_by_dealers();
            sum_indirect += treasury.get_percentage_debt_purchased_by_indirects();
            sum_bid_to_cover += treasury.get_bid_to_cover_ratio();
        }

        (
            sum_bid_to_cover / last_auctions as f64,
            sum_primary / last_auctions as f64,
            sum_indirect / last_auctions as f64,
        )
    }
}

#[cfg(test)]
mod tests {
    use mockito::Matcher;

    use crate::{
        get::TREASURIES_URL,
        latest::auctioned::AUCTIONED_URL,
        tests::fixture::{api_30y_bond_item, api_30y_very_old_bond_item, api_many_items},
    };

    use super::*;
    #[allow(dead_code)]
    const OUT_OF_BOUND_CUSIP: &str = "912810EY0";
    const TEST_CUSIP: &str = "912810SH2";

    #[test]
    fn it_should_assess_the_quality_of_an_auction() {
        let mut server = mockito::Server::new();

        let mut quality = Quality::new(TEST_CUSIP, LAST_AUCTIONS);
        quality.set_host(server.url());

        // println!("{:#?}", server.url());

        server
            .mock("GET", TREASURIES_URL)
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("cusip".into(), TEST_CUSIP.into()),
                Matcher::UrlEncoded("format".into(), "json".into()),
            ]))
            .with_body(api_30y_bond_item())
            .create();

        server
            .mock("GET", AUCTIONED_URL)
            .match_query(Matcher::AllOf(vec![Matcher::UrlEncoded(
                "type".into(),
                "Bond".into(),
            )]))
            .with_body(api_many_items())
            .create();

        let auction_quality = quality.get().unwrap();

        assert_eq!(-0.465, auction_quality);
    }

    #[test]
    fn it_should_correctly_handle_out_of_bound_conditions() {
        let mut server = mockito::Server::new();

        let mut quality = Quality::new(OUT_OF_BOUND_CUSIP, LAST_AUCTIONS);
        quality.set_host(server.url());

        server
            .mock("GET", TREASURIES_URL)
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("cusip".into(), quality.cusip()),
                Matcher::UrlEncoded("format".into(), "json".into()),
            ]))
            .with_body(api_30y_very_old_bond_item())
            .create();

        server
            .mock("GET", AUCTIONED_URL)
            .match_query(Matcher::AllOf(vec![Matcher::UrlEncoded(
                "type".into(),
                "Bond".into(),
            )]))
            .with_body(api_many_items())
            .create();

        let auction_quality = quality.get();

        assert!(auction_quality.is_err());
    }
}
