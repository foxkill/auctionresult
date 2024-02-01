//! # The Auction Result App
// 

mod treasury;
mod macros;

use treasury::Treasury;

fn main() {
    let tr = Treasury::default();
    println!("{tr:?}");
}