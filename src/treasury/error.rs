//! # The error used throughout this libray
//!
use std::any::Any;

#[derive(Debug)]
pub enum AuctionResultError {
    Request(reqwest::Error),
    RequestDyn(Box<dyn Any + Send>),
    // Could not parse cusip number.
    ParseCusip,
    ParseTenor,
    NoTreasury,
    OutOfBounds,
}

impl From<reqwest::Error> for AuctionResultError {
    fn from(value: reqwest::Error) -> Self {
        AuctionResultError::Request(value)
    }
}

impl From<Box<dyn Any + Send>> for AuctionResultError {
    fn from(value: Box<dyn Any + Send>) -> Self {
        AuctionResultError::RequestDyn(value)
    }
}

// This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// `Result<_, Parse>`. That way you don't need to do that manually.
// impl<E> From<E> for AuctionResultError
// where E: Into<reqwest::Error>,
// {
//     fn from(err: E) -> Self {
//         Self(err.into())
//     }
// }
