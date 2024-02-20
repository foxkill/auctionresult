//! The result

use crate::treasury::Treasury;

#[derive(Default, Debug)]
pub struct AuctionQuality {
    pub (in crate::quality) treasury: Treasury,
    pub (in crate::quality) lookback_auctions: usize,
    pub (in crate::quality) bid_to_cover_ratio_prev: f64,
    pub (in crate::quality) primary_dealers_prev: f64,
    pub (in crate::quality) direct_bidders_prev: f64,
    pub (in crate::quality) indirect_bidders_prev: f64,
    pub (in crate::quality) quality: f64,
}

impl AuctionQuality {
    /// Return the cusip of the requested auction.
    pub fn cusip(&self) -> String {
        self.treasury.cusip().to_owned()
    }

    /// Get the bid to cover ratio of this [`Auction`].
    pub fn get_bid_to_cover_ratio(&self) -> f64 {
        self.bid_to_cover_ratio_prev
    }

    /// Get the percentage of debt that was accepted by primary dealers for this [`Auction`].
    pub fn get_percentage_debt_purchased_by_dealers(&self) -> f64 {
        self.primary_dealers_prev
    }

    /// Returns the get percentage debt purchased by directs of this [`Auction`].
    pub fn get_percentage_debt_purchased_by_directs(&self) -> f64 {
        self.direct_bidders_prev
    }

    /// Returns the get percentage debt purchased by indirects of this [`Auction`].
    pub fn get_percentage_debt_purchased_by_indirects(&self) -> f64 {
        self.indirect_bidders_prev
    }

    /// Return the stored treasury.
    pub fn get_treasury(&self) -> Treasury {
        self.treasury.to_owned()
    }

    pub fn get(&self) -> f64 {
        self.quality
    }

    /// Return the number of auctions that are used to calculate the quality of the auction.
    pub fn get_number_of_lookback_auctions(&self) -> usize {
        self.lookback_auctions
    }
}