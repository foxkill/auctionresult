//! # The test module for the latest module.
use crate::treasury::treasury_type::TreasuryType;

use super::Latest;

#[test]
fn get_lastest_auctions() {
    let latest = Latest::new(TreasuryType::Bond, 0);
    let result = latest.get();
    let url = latest.url();
    println!("{result:#?}");
    println!("{url:#?}");
}

