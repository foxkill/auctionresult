/*!
# The library auctionresult.
*/
pub mod get;
pub mod latest;
pub mod tenor;
#[warn(missing_docs)]
mod tests;
pub mod treasury;

#[cfg(feature = "quality")]
pub mod quality;

// Re-exports - available modules.
pub use get::Get;
pub use latest::Latest;

pub use treasury::print::security_vprint;
pub use treasury::SecurityType;
pub use treasury::Treasury;
pub use treasury::TreasuryAccess;
