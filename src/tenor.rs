//! # Parser for tenors.
//!
const RE: &str = r"^(?P<security>\d+)([-])?(?P<term>\w+)$";

macro_rules! re {
    ($re:ident $(,)?) => {{
        static ONCE: std::sync::OnceLock<regex::Regex> = std::sync::OnceLock::new();
        ONCE.get_or_init(|| regex::Regex::new($re).unwrap())
    }};
}

use crate::treasury::{AuctionResult, AuctionResultError};
#[derive(Debug, PartialEq, Default)]
pub struct Tenor {
    security: u32,
    term: String,
    shortcut: String,
}

impl std::fmt::Display for Tenor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.security, self.term)
    }
}

impl Tenor {
    /// Contruct [`Tenor`] by parsing.
    pub fn parse(s: impl Into<String>) -> AuctionResult<Self> {
        let s = s.into();

        if s.is_empty() {
            return Ok(Tenor::default());
        }
        // Guard
        let Some(captures) = re!(RE).captures(&s) else {
            return Err(AuctionResultError::ParseTenor);
        };

        let security: &str = captures.name("security").map_or("", |m| m.as_str());
        let period: &str = captures.name("term").map_or("", |m| m.as_str());

        if security.is_empty() || period.is_empty() {
            return Err(AuctionResultError::ParseTenor);
        }
        let shortcut = period.to_lowercase().chars().next().unwrap_or('x');
        let term = match shortcut {
            'y' => "Year",
            'w' => "Weeks",
            'm' => "Months",
            'd' => "Day",
            _ => return Err(AuctionResultError::ParseTenor),
        };

        // Guard
        let Ok(sec) = security.parse::<u32>() else {
            return Err(AuctionResultError::ParseTenor);
        };

        Ok(Self {
            term: term.to_owned(),
            security: sec,
            shortcut: shortcut.to_string(),
        })
    }

    /// Detect if the tenor is empty.
    pub fn is_empty(&self) -> bool {
        self.term.is_empty() && self.security == 0
    }
    
    /// Return the security.
    pub fn security(&self) -> u32 {
        self.security
    } 

    /// Return the shortcut. Can be: "y", "m", "w" or "d".
    pub fn shortcut(&self) -> &str {
        self.shortcut.as_str()
    }

    /// Return the term.
    pub fn term(&self) -> &str {
        self.term.as_str()
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
        assert_eq!(tenor.to_string(), "10-Year");
    }

    #[test]
    fn it_should_return_a_parse_error() {
        let result = Tenor::parse("--Y");

        assert!(matches!(result, Err(AuctionResultError::ParseTenor)));
    }
}
