//! # Parser for tenors.
//!

const RE: &str = r"^(?P<security>\d+)([-])?(?P<term>\w+)$";

macro_rules! re {
    ($re:ident $(,)?) => {{
        static ONCE: std::sync::OnceLock<regex::Regex> = std::sync::OnceLock::new();
        ONCE.get_or_init(|| regex::Regex::new($re).unwrap())
    }};
}

use std::num::ParseIntError;

use crate::treasury::{AuctionResult, AuctionResultError};
pub struct Tenor {
    security: u32,
    term: String,
}

impl Tenor {
    pub fn parse(s: &str) -> AuctionResult<Tenor> {
        // Guard
        let Some(captures) = re!(RE).captures(s) else {
            return Err(AuctionResultError::Parse);
        };

        let security: &str = captures.name("security").map_or("", |m| m.as_str());
        let period: &str = captures.name("term").map_or("", |m| m.as_str());

        if security.is_empty() || period.is_empty() {
            return Err(AuctionResultError::Parse);
        }

        let term = match period.to_lowercase().chars().next().unwrap_or(' ') {
            'y' => "Year",
            'w' => "Weeks",
            'd' => "Days",        
            _ => "",
        };

        if term.is_empty() {
            return Err(AuctionResultError::Parse);
        }

        let sec = security.parse::<u32>();
        if sec.is_err() {
            return Err(AuctionResultError::Parse);
        }

        Ok(Tenor {
            term: term.to_owned(),
            security: sec.unwrap()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_correctly_parse_a_tenor() {
        let result = Tenor::parse("10-Y");
        
        assert!(result.is_ok());
        let tenor = result.unwrap();
        assert_eq!(tenor.security, 10);
        assert_eq!(tenor.term, "Year");
    }
}