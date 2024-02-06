//! # The client helper.
//!
//!
use std::str::FromStr;

use auctionresult::treasury::print::horizontally_print_out_treasury;
use auctionresult::vertically_print_out_treasury;
use auctionresult::Get;
use auctionresult::Latest;
use auctionresult::SecurityType;
use auctionresult::Treasury;
use auctionresult::TreasuryAccess;

use clap::Parser;
use clap::Subcommand;
use clap::ValueHint;

// The Parser.
#[derive(Debug, Parser)]
#[command(name = "ars")]
#[command(bin_name = "ars")]
#[command(author = "Stefan M. <foxkill@gmx.de>")]
#[command(
    about = "Retrieves information about US Treasury auctions and specific treasuries in particular."
)]
#[command(version)]
pub struct AuctionResultParser {
    #[arg(short = 'E', long, value_name = "vertical")]
    /// Display result not as a table.
    pub vertical: bool,
    #[clap(subcommand)]
    pub command: AuctionResultCommands,
}

// The Sub Commands.
#[derive(Debug, Subcommand)]
#[clap(author, version, about)]
pub enum AuctionResultCommands {
    #[command(arg_required_else_help = true)]
    /// Retrieve informations about a specific security.
    Get {
        #[arg(value_name = "cusip", value_hint = ValueHint::CommandString)]
        /// Retrieve the details of a treasury with the given cusip number.
        cusip: String,
    },

    /// Retrieves the latest aution results.
    #[command(arg_required_else_help = false)]
    Latest {
        #[arg(value_name = "type", long)]
        /// The security type.
        sectype: Option<String>,
        #[arg(value_name = "days", long)]
        /// The number of days we want to look back.
        days: Option<usize>,
    },
}

/// Handle the command get.
pub fn handle_get(args: &AuctionResultParser) {
    let AuctionResultCommands::Get { cusip } = &args.command else {
        panic!("Cannot extract cusip number!")
    };

    let get_command = Get::new(cusip);

    let response: Vec<Treasury> = get_command.get();
    if args.vertical {
        vertically_print_out_treasury(&response);
    } else {
        horizontally_print_out_treasury(&response);
    }
}

/// Handle the command lastest.
pub fn handle_latest(args: &AuctionResultParser) {
    let AuctionResultCommands::Latest { sectype, days } = &args.command else {
        panic!("Cannot extract the security type and/or the number of days to look back.")
    };

    let security_type = sectype.as_ref().map_or(SecurityType::Null, |st| {
        SecurityType::from_str(st).unwrap_or(SecurityType::Null)
    });

    let look_back_days = days.unwrap_or(0);
    let latest = Latest::new(security_type, look_back_days);

    let response: Vec<Treasury> = latest.get();
    if args.vertical {
        vertically_print_out_treasury(&response);
    } else {
        horizontally_print_out_treasury(&response);
    }
}
