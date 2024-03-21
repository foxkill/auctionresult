//! # The Quality Module
//!
//! measures the quality of an auction.
mod auction_quality;
pub use self::auction_quality::AuctionQuality;

use crate::tenor::Tenor;
use crate::{
    treasury::{AuctionResult, AuctionResultError, Treasury, TreasuryAccess},
    Get, Latest,
};


/// The number of auctions to consider.
const LAST_AUCTIONS: usize = 5;

const BID_TO_COVER_RATIO_WEIGHT: f64 = 0.5;
const PRIMARY_DEALER_ACCEPTED_WEIGHT: f64 = 0.4;
const INDIRECT_BIDDER_WEIGHT: f64 = 0.075;
const DIRECT_BIDDER_WEIGHT: f64 = 0.025;

#[allow(dead_code)]
const WHEN_ISSUED_WEIGHT: f64 = 0.4;

#[derive(Default, Debug, Clone)]
pub struct QualityCommand {
    cusip: String,
    lookback_auctions: usize,
    host: String,
    // Info block.
    // treasury: Treasury,
    // bid_to_cover_ratio_prev: f64,
    // primary_dealers_prev: f64,
    // direct_bidders_prev: f64,
    // indirect_bidders_prev: f64,
    // quality: f64,
}



// Auction bid-to-cover ratio = 2.3 (desired range: 2.0-3.0) - Score: 75
// Primary dealer participation = 65% (desired range: 50%-70%) - Score: 85
// Weighted score (assuming 70% weight for bid-to-cover, 30% for participation):
// 75 * 0.7 + 85 * 0.3 = 79.5

impl QualityCommand {
    /// Return a new instance of the quality module.
    pub fn new(cusip: impl Into<String>, lookback_auctions: usize) -> Self {
        Self {
            cusip: cusip.into(),
            lookback_auctions: if lookback_auctions == 0 {
                LAST_AUCTIONS
            } else {
                lookback_auctions
            },
            host: "".to_owned(),
        }
    }

    /// Return the quality of an auction.
    pub fn calculate(&self) -> AuctionResult<AuctionQuality> {
        let mut get_command = Get::new(&self.cusip);

        if cfg!(test) {
            get_command.set_host(&self.host);
        }

        let treasuries = get_command.get()?;

        let Some(treasury) = treasuries.first() else {
            return Err(AuctionResultError::NoTreasury);
        };

        let mut aq = AuctionQuality::default();

        aq.quality = self.calculate_quality(treasury, &mut aq)?;
        aq.treasury = treasury.clone();
        aq.lookback_auctions = self.lookback_auctions;

        Ok(aq)
    }

    /// Return the CUSIP of the requested auction.
    pub fn cusip(&self) -> &str {
        self.cusip.as_str()
    }

    /// Return the number of auctions that are used to calculate the quality of the auction.
    pub fn get_number_of_lookback_auctions(&self) -> usize {
        self.lookback_auctions
    }

    #[allow(dead_code)]
    fn set_host(&mut self, host: impl Into<String>) {
        self.host = host.into();
    }

    /// Return the quality of the auction.
    //
    /// [`treasuries`]: The treasuries to consider.
    /// [`lookback_auctions`]: The _number_ of auctions to consider in the past.
    fn calculate_quality(&self, treasury: &Treasury, auction_quality: &mut AuctionQuality) -> AuctionResult<f64> {
        // Get the term of the treasury specified by the given cusip.
        let tenor = Tenor::parse(treasury.get_term())?;

        // Get the security type.
        let security_type = treasury.get_security_type();

        // Create the lastest module and search for auctions that were held
        // before the given auction.
        let mut latest = Latest::new(security_type, 0, tenor);

        if cfg!(test) {
            latest.set_host(&self.host);
        }

        let lastest_auctions = latest.get()?;

        // Make sure we can look at the lastest X number of auctions.
        if lastest_auctions.len() < self.lookback_auctions + 1 {
            return Err(AuctionResultError::OutOfBounds);
        }

        // Find the auction with given cusip.
        let Some(pos) = lastest_auctions
            .iter()
            .position(|s| s.cusip() == treasury.cusip())
        else {
            return Err(AuctionResultError::NoTreasury);
        };

        // Make sure we can look behind the lastest X number of auctions.
        if pos + self.lookback_auctions + 1 > lastest_auctions.len() {
            return Err(AuctionResultError::OutOfBounds);
        }

        let treasuries = lastest_auctions
            .iter()
            .skip(pos + 1)
            .take(self.lookback_auctions)
            .collect::<Vec<&Treasury>>();

        self.ratio_mean(&treasuries, auction_quality, self.lookback_auctions);

        // Capture values.
        // self.treasury = treasury.clone();
        // self.bid_to_cover_ratio = sum_bid_to_cover;

        let diff_primary_dealers = (treasury.get_percentage_debt_purchased_by_dealers()
            - auction_quality.primary_dealers_prev)
            * -PRIMARY_DEALER_ACCEPTED_WEIGHT;

        let diff_direct_bidders = (treasury.get_percentage_debt_purchased_by_directs()
            - auction_quality.direct_bidders_prev)
            * -DIRECT_BIDDER_WEIGHT;

        let diff_indirect_bidders = (treasury.get_percentage_debt_purchased_by_indirects()
            - auction_quality.indirect_bidders_prev)
            * -INDIRECT_BIDDER_WEIGHT;

        let diff_bid_to_cover =
            (treasury.get_bid_to_cover_ratio() - auction_quality.bid_to_cover_ratio_prev) * BID_TO_COVER_RATIO_WEIGHT;

        // Round to 3 decimal places.
        let quality = ((diff_primary_dealers
            + diff_direct_bidders
            + diff_indirect_bidders
            + diff_bid_to_cover)
            * 1000.0)
            .round()
            / 1000.0;

        Ok(quality)
    }

    /// Calculate the mean of the _bid to cover_, _primary dealers_ and _indirect_bidders_ of
    /// the last [`last_auctions`] auctions.
    fn ratio_mean(&self, treasuries: &[&Treasury], auction_quality: &mut AuctionQuality, last_auctions: usize) {
        let (sum_primary_dealers, sum_direct_bidders, sum_bid_to_cover, sum_indirect_bidders) =
            treasuries.iter().take(last_auctions).fold(
                (0., 0., 0., 0.),
                |(
                    sum_primary_dealers,
                    sum_direct_bidders,
                    sum_bid_to_cover,
                    sum_indirect_bidders,
                ),
                 treasury| {
                    (
                        sum_primary_dealers + treasury.get_percentage_debt_purchased_by_dealers(),
                        sum_direct_bidders + treasury.get_percentage_debt_purchased_by_directs(),
                        sum_bid_to_cover + treasury.get_bid_to_cover_ratio(),
                        sum_indirect_bidders + treasury.get_percentage_debt_purchased_by_indirects(),
                    )
                },
            );

        auction_quality.primary_dealers_prev = sum_primary_dealers / last_auctions as f64;
        auction_quality.direct_bidders_prev = sum_direct_bidders / last_auctions as f64;
        auction_quality.indirect_bidders_prev = sum_indirect_bidders / last_auctions as f64;
        auction_quality.bid_to_cover_ratio_prev = sum_bid_to_cover / last_auctions as f64;
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

        let mut quality = QualityCommand::new(TEST_CUSIP, LAST_AUCTIONS);
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

        let auction_quality = quality.calculate().unwrap();

        assert_eq!(-2.127, auction_quality.get());
    }

    #[test]
    fn it_should_correctly_handle_out_of_bound_conditions() {
        let mut server = mockito::Server::new();

        let mut quality = QualityCommand::new(OUT_OF_BOUND_CUSIP, LAST_AUCTIONS);
        quality.set_host(server.url());

        server
            .mock("GET", TREASURIES_URL)
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("cusip".into(), quality.cusip().into()),
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

        let auction_quality = quality.calculate();

        assert!(auction_quality.is_err());
    }
}
