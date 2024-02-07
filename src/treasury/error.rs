//! # The error used throughout this libray
//! 
use std::any::Any;

#[derive(Debug)]
pub enum AuctionResultError {
    RequestError(reqwest::Error),
    RequestErrorDyn(Box<dyn Any + Send>),
    ParseError,
}

impl std::convert::From<reqwest::Error> for AuctionResultError {
    fn from(value: reqwest::Error) -> Self {
        AuctionResultError::RequestError(value)
    }
}

impl From<Box<dyn Any + Send>> for AuctionResultError {
    fn from(value: Box<dyn Any + Send>) -> Self {
       AuctionResultError::RequestErrorDyn(value) 
    }
}