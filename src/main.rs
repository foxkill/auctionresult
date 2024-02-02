//! # The Auction Result App
// 

mod treasury;
mod macros;
mod latest;

use treasury::Treasury;

use crate::{latest::Latest, treasury::treasury_type::TreasuryType};

fn main() {
    let tr = Treasury::default();
    let l = Latest::new(TreasuryType::Null, 0);
    l.days();
    l.get();
    println!("{l:#?} -> {tr:?}");
}