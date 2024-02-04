//! # This module retrieves the treasury data from the given url.
//!

use std::thread::{self, JoinHandle};

use reqwest::blocking::Client;
use super::Treasury;

/// Naive version of non blocking request get.
pub fn load(url: impl Into<String>) -> JoinHandle<Vec<Treasury>> {
    let client = Client::new();
    let def = vec![Treasury::default()];
    let url = url.into();

    thread::spawn(move || {
        let Ok(resp) = client.get(url).send() else {
            return def;
        };
        resp.json().unwrap_or(def)
    })
}
