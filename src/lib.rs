//! # The library auctionresult.
//! 
//! 
pub mod latest;
pub mod treasury;
pub mod get;

// Re-exports.
pub use latest::Latest;
pub use treasury::Treasury;
pub use treasury::TreasuryType;