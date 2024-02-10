//! # The Quality Module
//!
//! measures the quality of an auction.

#![allow(unused)]

use crate::{treasury::Treasuries, TreasuryAccess};

const BID_TO_COVER_RATION_WEIGHT: f32 = 0.4;
const PRIMARY_DEALER_ACCEPTED_WEIGHT: f32 = 0.3;
const WHEN_ISSUED_WEIGHT: f32 = 0.7;

pub struct Quality {
    #[cfg(test)]
    host: String,
}c

// Auction bid-to-cover ratio = 2.3 (desired range: 2.0-3.0) - Score: 75
// Primary dealer participation = 65% (desired range: 50%-70%) - Score: 85
// Weighted score (assuming 70% weight for bid-to-cover, 30% for participation):
// 75 * 0.7 + 85 * 0.3 = 79.5

impl TreasuryAccess<Treasuries> for Quality {
    fn get(&self) -> crate::treasury::AuctionResult<Treasuries> {
        todo!()
    }

    fn url(&self) -> String {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_should_assess_the_quality_of_an_auction() {
        println!("This is just the beginning");
    }
}
