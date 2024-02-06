//! # This module retrieves the treasury data from the given url.
//!

use super::error::AuctionResultError;
use reqwest::blocking::{get, Response};
use std::thread;

/// Naive version of non blocking request.
pub fn load(url: impl Into<String>) -> Result<Response, AuctionResultError> {
    let url = url.into();
    let handle = thread::spawn(move || get(url));
    let thread_result = handle.join();

    // Joining the thread failed.
    let Ok(tr) = thread_result else {
        return Err(AuctionResultError::RequestErrorDyn(thread_result.unwrap_err()));
    };

    // Invalid server response.
    let Ok(response) = tr else {
        return Err(AuctionResultError::RequestError(tr.unwrap_err()));
    };

    response.error_for_status().map_err(AuctionResultError::RequestError)
}
