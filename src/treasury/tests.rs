//! # The test suite.
//!
//!
extern crate serde;

pub(crate) mod fixture;

use chrono::DateTime;
use crate::treasury::{tests::fixture::MULTIPLE_ITEMS_COUNT, Treasury};

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
fn deserialize_multiple_items() {
    let fxt = fixture::api_multiple_items();
    let result: Vec<Treasury> = serde_json::from_str(fxt).unwrap_or_else(|_| {
        vec![Treasury::default()]
    });
    assert!(result.len() == MULTIPLE_ITEMS_COUNT);
}

#[test]
fn deserialize_single_item() {
    let fxt = fixture::api_single_item();
    let result: Vec<Treasury> = serde_json::from_str(fxt).unwrap();
    assert!(result.len() == 1);
}

#[test]
fn deserialize_empty_item() {
    let fxt = fixture::api_empty_items();
    let result: Vec<Treasury> = serde_json::from_str(fxt).unwrap();
    assert!(result.len() == 1);
}