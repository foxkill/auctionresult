//! # The test suite.
//!
//!
extern crate serde;

use chrono::DateTime;

use crate::{tests::fixture::{api_empty_items, api_empty_response, api_multiple_items, api_single_item, MULTIPLE_ITEMS_COUNT}, Treasury};

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
    let fxt = api_multiple_items();
    let result: Vec<Treasury> = serde_json::from_str(fxt).unwrap_or_else(|_| {
        vec![Treasury::default()]
    });
    assert!(result.len() == MULTIPLE_ITEMS_COUNT);
}

#[test]
fn deserialize_single_item() {
    let fxt = api_single_item();
    let result: Vec<Treasury> = serde_json::from_str(fxt).unwrap();
    assert!(result.len() == 1);
}

#[test]
fn deserialize_empty_item() {
    let fxt = api_empty_items();
    let result: Vec<Treasury> = serde_json::from_str(fxt).unwrap();
    assert!(result.is_empty());
}

#[test]
fn deserialize_empty_response() {
    let fxt = api_empty_response();
    let result: Result<Vec<Treasury>, serde_json::Error> = serde_json::from_str(fxt);
    assert!(result.is_err());
}