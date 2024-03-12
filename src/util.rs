// !# Contains utility functions.

use cusip as cu;

/// Validate a cusip number.
pub fn validate_cusip(cusip: impl Into<String>) -> bool {
    cu::validate(cusip.into().as_str())
}