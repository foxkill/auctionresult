//! # The library auctionresult.
//! 
//! 
mod tests;
pub mod latest;
pub mod treasury;
pub mod get;

// Re-exports.
pub use latest::Latest;
pub use get::Get;

pub use treasury::Treasury;
pub use treasury::TreasuryType;
pub use treasury::TreasuryAccess;
pub use treasury::print::vertically_print_out_treasury;