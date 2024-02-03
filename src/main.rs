//! # The Auction Result App
// 

mod treasury;
mod macros;
pub mod latest;

use crate::{latest::Latest, treasury::TreasuryType};


fn main() {
    // let tr = Treasury::default();
    let l = Latest::new(TreasuryType::Bill, 0);
    println!("{l:#?}")
}