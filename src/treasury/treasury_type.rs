//! # The treasury type
//! 
//! The different types of treasuries

use std::str::FromStr;

#[derive(Debug)]
pub struct ConvertError;

#[derive(Debug, PartialEq)]
pub enum TreasuryType {
    Bill,
    Note,
    Bond,
    Frn,
    Cmb,
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

        for (k,v) in variants_as_str.iter().enumerate() {
            assert_eq!(expected_variants[k], TreasuryType::from_str(v).unwrap());
        }
    }
}