//! # This module retrieves the treasury data from the given url.
//!

use std::thread;
use reqwest::{blocking::get, blocking::Response};
use super::error::AuctionResultError;

/// Naive version of non blocking request.
pub fn load(url: impl Into<String>) -> Result<Response, AuctionResultError> {
    let url = url.into();
    let handle = thread::spawn(move || { get(url) });
    let response = handle.join().map_err(AuctionResultError::RequestErrorDyn)?;
    Ok(response?)
}
