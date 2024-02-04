//! # A module that prints out treasuries.
//! 
use super::Treasury;

// Security Term:   3-Year
// CUSIP:           912828YF1
// Reopening:       No
// Security Type:   Note
// Issue Date:      09/16/2019
// Maturity Date:   09/15/2022
// Bid To Cover:    0.00
// Dealers:         0.00%
// High Yield:      1.573%
// Interest Rate:   1.500%

pub fn vertically_print_out_treasury(treasuries: &Vec<Treasury>) {
    for treasury in treasuries {
        // println!("cusip: {}", treasury.cusip());
        println!("Security Term:\t{}", treasury.security_term);
        println!("CUSIP:\t\t{}", treasury.cusip);
        println!("Reopening:\t{}", if treasury.reopening {"Yes"} else {"No"});
        println!("Security Type:\t{}", treasury.treasury_type);
        println!("Maturity Date:\t{}", treasury.maturity_date.format("%m/%d/%Y"));
        println!("Bid To Cover:\t{}", treasury.bid_to_cover_ratio);
        println!("Dealers:\t{}", treasury.primary_dealer_accepted);
        println!("High Yield:\t{}", treasury.high_yield);
        println!("Interest Rate:\t{}", treasury.interest_rate);
        println!();
    }
}

#[cfg(test)]
mod tests {
    use crate::{tests::fixture::api_multiple_items, Treasury};

    use super::vertically_print_out_treasury;

    #[test]
    fn it_should_print_out_treasury_vertical() {
        let data = api_multiple_items();
        let treasuries: Vec<Treasury> = serde_json::from_str(data).unwrap();

        vertically_print_out_treasury(&treasuries);        
    }
}
