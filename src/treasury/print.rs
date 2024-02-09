//! # A module that prints out treasuries.
//!
extern crate prettytable;

use crate::SecurityType;

use super::Treasury;
use prettytable::{
    format::{self, Alignment},
    row, Cell, Row, Table,
};

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

pub fn security_vprint(treasuries: &Vec<Treasury>) {
    let mut f = numfmt::Formatter::default();
    let mut table = Table::new();
    let datefmt = Treasury::get_default_date_fmt();

    // table.set_format(*format::consts::FORMAT_CLEAN);
    table.add_row(Row::from(Treasury::new().get_fields()));

    for treasury in treasuries {
        table.add_row(Row::new(vec![
            Cell::new(&treasury.security_term),
            Cell::new(&treasury.cusip),
            Cell::new_align(
                if treasury.reopening { "Yes" } else { "No" },
                Alignment::CENTER,
            ),
            Cell::new(&treasury.security_type.to_string()),
            Cell::new(&format!("{}", treasury.issue_date.format(datefmt))),
            Cell::new(&format!("{}", treasury.maturity_date.format(datefmt))),
            Cell::new_align(
                &format!("{:.2}", treasury.bid_to_cover_ratio),
                Alignment::RIGHT,
            ),
            Cell::new_align(f.fmt2(treasury.primary_dealer_accepted), Alignment::RIGHT),
            Cell::new_align(
                &format!(
                    "{:.3}%",
                    if treasury.security_type == SecurityType::Bill {
                        treasury.high_discount_rate
                    } else {
                        treasury.high_yield
                    }
                ),
                Alignment::RIGHT,
            ),
            Cell::new_align(
                &format!(
                    "{:.3}%",
                    if treasury.security_type == SecurityType::Bill {
                        treasury.high_investment_rate
                    } else {
                        treasury.interest_rate
                    }
                ),
                Alignment::RIGHT,
            ),
        ]));
    }

    table.printstd()
}

pub fn security_print(treasuries: &Vec<Treasury>) {
    let mut f = numfmt::Formatter::default();
    let mut table = Table::new();
    let datefmt = Treasury::get_default_date_fmt();
    table.set_format(*format::consts::FORMAT_CLEAN);

    for treasury in treasuries {
        table.add_row(row!["Security Term:", &treasury.security_term]);
        table.add_row(row!["CUSIP", &treasury.cusip]);
        table.add_row(row![
            "Reopening:",
            if treasury.reopening { "Yes" } else { "No" }
        ]);
        table.add_row(row!["Security Type:", &treasury.security_type]);
        table.add_row(row!["Issue Date:", treasury.issue_date.format(datefmt)]);
        table.add_row(row![
            "Maturity Date",
            treasury.maturity_date.format(datefmt)
        ]);
        table.add_row(row![
            "Maturity Date:",
            treasury.maturity_date.format(datefmt)
        ]);
        table.add_row(row!["Bid To Cover:", treasury.bid_to_cover_ratio]);
        table.add_row(row!["Dealers:", f.fmt2(treasury.primary_dealer_accepted)]);
        if treasury.security_type == SecurityType::Bill {
            table.add_row(row!["High Rate:", treasury.high_discount_rate]);
            table.add_row(row!["Investment Rate:", treasury.high_investment_rate]);
        } else {
            table.add_row(row!["High Yield:", treasury.high_yield]);
            table.add_row(row!["Interest Rate:", treasury.interest_rate]);
        }
        table.add_row(Row::empty());
    }

    table.printstd()
}

#[cfg(test)]
mod tests {
    use crate::{tests::fixture::api_multiple_items, Treasury};

    use super::*;
    #[test]
    fn it_should_print_out_treasury_horizontal() {
        let data = api_multiple_items();
        let treasuries: Vec<Treasury> = serde_json::from_str(data).unwrap();

        security_vprint(&treasuries);
    }
    #[test]
    fn it_should_print_out_treasury_vertical() {
        let data = api_multiple_items();
        let treasuries: Vec<Treasury> = serde_json::from_str(data).unwrap();

        security_print(&treasuries);
    }
}
