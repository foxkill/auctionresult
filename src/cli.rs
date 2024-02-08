//! # The client helper.
//!
//!
use std::process::exit;
use std::str::FromStr;

use auctionresult::treasury::print::horizontally_print_out_treasury;
use auctionresult::treasury::AuctionResultError;
use auctionresult::vertically_print_out_treasury;
use auctionresult::Get;
use auctionresult::Latest;
use auctionresult::SecurityType;
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
        /// Filter for a specfic tenor, i. e. 10y (for all Ten Year notes)
        #[arg(value_name = "tenor", long)]
        tenor: Option<String>,
    },
}

fn handle_error(e: AuctionResultError) -> i32 {
    match e {
        AuctionResultError::Request(req) => {
            println!("Invalid Request, status code {}", req.status().unwrap());
            1
        },
        AuctionResultError::RequestDyn(_) => {
            println!("Invalid dynamic request");
            2
        },
        AuctionResultError::Parse => {
            println!("Could not parse cusip number.");
            3
        }
    }
}

/// Handle the command get.
pub fn handle_get(args: &AuctionResultParser) {
    let AuctionResultCommands::Get { cusip } = &args.command else {
        println!("Cannot extract cusip number.");
        exit(0);
    };

    let get_command = Get::new(cusip);

    let treasuries = match get_command.get() {
        Ok(vec) => vec,
        Err(e) => {
            exit(handle_error(e))
        }
    };

    if args.vertical {
        vertically_print_out_treasury(&treasuries);
    } else {
        horizontally_print_out_treasury(&treasuries);
    }
}

/// Handle the command lastest.
pub fn handle_latest(args: &AuctionResultParser) {
    let AuctionResultCommands::Latest { sectype, days, tenor } = &args.command else {
        panic!("Cannot extract the security type and/or the number of days to look back.")
    };

    let security_type = sectype.as_ref().map_or(SecurityType::Null, |st| {
        SecurityType::from_str(st).unwrap_or(SecurityType::Null)
    });

    let look_back_days = days.unwrap_or(0);
    let default_tenor = String::from("");
    let tenor = tenor.as_ref().unwrap_or(&default_tenor);

    println!("{}", tenor);
    let latest_command = Latest::new(security_type, look_back_days);

    let response = latest_command.get();

    let securities = match response {
        Ok(vec) => vec,
        Err(e) => {
            exit(handle_error(e))
        }
    };
    
    if args.vertical {
        vertically_print_out_treasury(&securities);
    } else {
        horizontally_print_out_treasury(&securities);
    }
}
