//! # The error used throughout this libray
//! 

use std::any::Any;

#[derive(Debug)]
pub enum AuctionResultError {
    RequestError(reqwest::Error),
    RequestErrorDyn(Box<dyn Any + Send>),
}

impl std::convert::From<reqwest::Error> for AuctionResultError {
    fn from(value: reqwest::Error) -> Self {
        AuctionResultError::RequestError(value)
    }
}