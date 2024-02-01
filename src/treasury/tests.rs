//! # The test suite.
//!
//!
mod fixture;

extern crate serde;

use std::thread::Thread;

use serde::{Serialize, Deserialize};
use chrono::DateTime;

use crate::treasury::Treasury;

#[test]
fn date_time_parse() {
    let mut md = String::from("2025-12-31T00:00:00");

    let dt = DateTime::parse_from_str(&md, "%Y-%m-%dT%H:%M:%S");
    md.push_str("+00:00");
    let dt_json = DateTime::parse_from_rfc3339(&md);
    // println!("{dt:?} {dt_json:?}");
    assert!(dt.is_err());
    assert!(dt_json.is_ok());
}

#[test]
fn deserialize_treasury_struct() {
    let fxt = fixture::get_json();
    let result: Vec<Treasury> = serde_json::from_str(fxt).unwrap();
    println!("{:#?}", result);
}
