//! # The Quality Module
//!
//! measures the quality of an auction.

#![allow(unused)]

use crate::{
    treasury::{self, Treasuries},
    TreasuryAccess,
};

/// The number of auctions to consider.
const LAST_AUCTIONS: usize = 5;

const BID_TO_COVER_RATIO_WEIGHT: f64 = 0.3;
const PRIMARY_DEALER_ACCEPTED_WEIGHT: f64 = 0.2;
const INDIRECT_BIDDER_WEIGHT: f64 = 0.1;
const WHEN_ISSUED_WEIGHT: f64 = 0.4;

#[derive(Default, Debug)]
pub struct Quality {
    sum_bid_to_cover: f64,
    sum_primary_dealers: f64,
    sum_indirect_bidders: f64,
    quality: f64,
}

// Auction bid-to-cover ratio = 2.3 (desired range: 2.0-3.0) - Score: 75
// Primary dealer participation = 65% (desired range: 50%-70%) - Score: 85
// Weighted score (assuming 70% weight for bid-to-cover, 30% for participation):
// 75 * 0.7 + 85 * 0.3 = 79.5

impl Quality {
    pub fn new() -> Self {
        Self::default()
    }

    /// Return the quality of the auction.
    //
    /// [`treasuries`]: The treasuries to consider.
    /// [`lookback_auctions`]: The _number_ of auctions to consider in the past.
    pub fn auction_quality(&mut self, treasuries: &Treasuries, lookback_auctions: usize) -> f64 {
        (
            self.sum_bid_to_cover,
            self.sum_primary_dealers,
            self.sum_indirect_bidders,
        ) = self.ratio_mean(treasuries, 5);

        let treasury = treasuries.first().unwrap();

        // println!(
        //     "Treasury first: {:.2}->{:.2} {:.2}->{:.2} {:.2}->{:.2}",
        //     treasury.get_bid_to_cover_ratio(),
        //     self.sum_bid_to_cover,
        //     treasury.get_percentage_debt_purchased_by_dealers(),
        //     self.sum_primary_dealers,
        //     treasury.get_percentage_debt_purchased_by_indirects(),
        //     self.sum_indirect_bidders
        // );

        let diff_bid_to_cover =
            (treasury.get_bid_to_cover_ratio() - self.sum_bid_to_cover) * BID_TO_COVER_RATIO_WEIGHT;
        let diff_primary_dealers = (treasury.get_percentage_debt_purchased_by_dealers()
            - self.sum_primary_dealers)
            * -PRIMARY_DEALER_ACCEPTED_WEIGHT;
        let diff_indirect_bidders = (treasury.get_percentage_debt_purchased_by_indirects()
            - self.sum_indirect_bidders)
            * -INDIRECT_BIDDER_WEIGHT;

        // Round to 3 decimal places.
        ((diff_bid_to_cover + diff_primary_dealers + diff_indirect_bidders) * 1000.0).round()
            / 1000.0
    }

    /// Calculate the mean of the _bid to cover_, _primary dealers_ and _indirect_bidders_ of
    /// the last [`last_auctions`] auctions.
    fn ratio_mean(&self, treasuries: &Treasuries, last_auctions: usize) -> (f64, f64, f64) {
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
        latest::auctioned::AUCTIONED_URL, quality::LAST_AUCTIONS, tenor::Tenor,
        tests::fixture::api_many_items, treasury::Treasuries, Latest, SecurityType, TreasuryAccess,
    };

    use super::Quality;

    #[test]
    fn it_should_assess_the_quality_of_an_auction() {
        let mut server = mockito::Server::new();
        let mut latest = Latest::new(SecurityType::Bill, 8, Tenor::default());

        latest.set_host(server.url());

        server
            .mock("GET", AUCTIONED_URL)
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("type".into(), latest.get_security_type().to_string()),
                Matcher::UrlEncoded("days".into(), latest.get_days().to_string()),
            ]))
            .with_body(api_many_items())
            .create();

        let response: Treasuries = latest.get().unwrap();

        let mut quality = Quality::new();
        let q = quality.auction_quality(&response, LAST_AUCTIONS);
        // println!("Auction Quality is: {:#?}->{:.3}", quality, q);
        assert_eq!(-0.545, q);
    }
}
