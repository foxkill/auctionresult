//! # The error used throughout this libray
//! 
use std::any::Any;

#[derive(Debug)]
pub enum AuctionResultError {
    Request(reqwest::Error),
    RequestDyn(Box<dyn Any + Send>),
    // Could not parse cusip number.
    Parse,
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