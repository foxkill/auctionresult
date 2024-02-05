//! # The treasury type
//!
//! Represents the different types of treasuries.
use core::fmt;
use serde::Deserialize;
use std::str::FromStr;

#[derive(Debug)]
pub struct ConvertError;

#[derive(Debug, PartialEq, Deserialize, Default, Clone)]
pub enum SecurityType {
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

impl FromStr for SecurityType {
    type Err = ConvertError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let variant = match s.to_uppercase().as_str() {
            "BILL" => SecurityType::Bill,
            "NOTE" => SecurityType::Note,
            "BOND" => SecurityType::Bond,
            "FRN" => SecurityType::Frn,
            "CMB" => SecurityType::Cmb,
            _ => return Err(ConvertError),
        };

        Ok(variant)
    }
}

impl fmt::Display for SecurityType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SecurityType::Bill => write!(f, "Bill"),
            SecurityType::Note => write!(f, "Note"),
            SecurityType::Bond => write!(f, "Bond"),
            SecurityType::Frn => write!(f, "FRN"),
            SecurityType::Cmb => write!(f, "CMB"),
            SecurityType::Null => write!(f, "Null"),
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
        let expected_variants: [SecurityType; 5] = [
            SecurityType::Bill,
            SecurityType::Note,
            SecurityType::Bond,
            SecurityType::Cmb,
            SecurityType::Frn,
        ];

        for (k, v) in variants_as_str.iter().enumerate() {
            assert_eq!(expected_variants[k], SecurityType::from_str(v).unwrap());
        }
    }

    #[test]
    fn to_string() {
        let expected_variants: [SecurityType; 6] = [
            SecurityType::Bill,
            SecurityType::Note,
            SecurityType::Bond,
            SecurityType::Cmb,
            SecurityType::Frn,
            SecurityType::Null,
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
