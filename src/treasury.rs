//! # Treasury
//! 

#[derive(Debug)]
pub struct Treasury {
    cusip: &str,
    stype: &str,
    term: &str,
    securityTerm : &str,
    reopening: bool,
    issueDate: f64,
    maturityDate : f64,
    highYield: f32,
    interestRate:f32,
    highDiscountRate:f32,
    highInvestmentRate: f32,
    asResults: bool,
    mlFilenameCompetitiveResults: &str,
    primaryDealerAccepted: f64,
    bidToCoverRatio: f32,
    totalAccepted: f32,
}