/*!
# The library auctionresult.
*/
pub mod get;
pub mod latest;
pub mod tenor;
#[warn(missing_docs)]
mod tests;
pub mod treasury;

// #[cfg(feature = "quality")]
pub mod quality;

// Re-exports - available modules.
pub use get::Get;
pub use latest::Latest;

// Re-export Treasury Types
pub use treasury::print::security_vprint;
pub use treasury::SecurityType;
// pub use self::treasury::Treasury;
// pub use self::treasury::TreasuryAccess;
// pub use self::treasury::AuctionResult;
// pub use self::treasury::AuctionResultError;
// pub use self::treasury::Treasuries;
