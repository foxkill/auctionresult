//! # The treasury type
//!
//! Represents the different types of treasuries.

use core::fmt;
use serde::Deserialize;
use std::str::FromStr;

#[derive(Debug)]
pub struct ConvertError;

#[derive(Debug, PartialEq, Deserialize, Default)]
pub(crate) enum TreasuryType {
    Bill,
    Note,
    Bond,
    #[serde(rename = "FRN")]
    Frn,
    #[serde(rename = "CMB")]
    Cmb,
    #[default]
    Null,
}

impl FromStr for TreasuryType {
    type Err = ConvertError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let variant = match s.to_uppercase().as_str() {
            "BILL" => TreasuryType::Bill,
            "NOTE" => TreasuryType::Note,
            "BOND" => TreasuryType::Bond,
            "FRN" => TreasuryType::Frn,
            "CMB" => TreasuryType::Cmb,
            _ => return Err(ConvertError),
        };

        Ok(variant)
    }
}

impl fmt::Display for TreasuryType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TreasuryType::Bill => write!(f, "Bill"),
            TreasuryType::Note => write!(f, "Note"),
            TreasuryType::Bond => write!(f, "Bond"),
            TreasuryType::Frn => write!(f, "FRN"),
            TreasuryType::Cmb => write!(f, "CMB"),
            TreasuryType::Null => write!(f, "Null"),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn it_should_convert_a_string_to_a_treasury_type() {
        let variants_as_str: [&str; 5] = ["bilL", "nOte", "Bond", "cmb", "frn"];
        let expected_variants: [TreasuryType; 5] = [
            TreasuryType::Bill,
            TreasuryType::Note,
            TreasuryType::Bond,
            TreasuryType::Cmb,
            TreasuryType::Frn,
        ];

        for (k, v) in variants_as_str.iter().enumerate() {
            assert_eq!(expected_variants[k], TreasuryType::from_str(v).unwrap());
        }
    }

    #[test]
    fn to_string() {
        let expected_variants: [TreasuryType; 6] = [
            TreasuryType::Bill,
            TreasuryType::Note,
            TreasuryType::Bond,
            TreasuryType::Cmb,
            TreasuryType::Frn,
            TreasuryType::Null,
        ];

        let result_variants: [&str; 6] = [
            "Bill",
            "Note",
            "Bond",
            "CMB",
            "FRN",
            "Null",
        ];

        for (index, variant) in expected_variants.iter().enumerate() {
            assert_eq!(result_variants[index], variant.to_string());
        }
    }
}
