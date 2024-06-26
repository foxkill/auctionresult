//! # The client helper.
//!
//!

#[cfg(feature = "quality")]
use auctionresult::quality;

use auctionresult::security_vprint;
use auctionresult::tenor::Tenor;
use auctionresult::treasury::print::security_print;
use auctionresult::treasury::AuctionResultError;
use auctionresult::treasury::TreasuryAccess;
use auctionresult::Get;
use auctionresult::Latest;
use auctionresult::SecurityType;

use std::process::exit;
use std::str::FromStr;

use clap::Parser;
use clap::Subcommand;
use clap::ValueHint;
use reqwest::StatusCode;

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

    #[cfg(feature = "quality")]
    #[command(arg_required_else_help = false)]
    /// Prints the quality of the auction.
    Quality {
        #[arg(value_name = "cusip", value_hint = ValueHint::CommandString)]
        /// Retrieve the details of a treasury with the given cusip number.
        cusip: String,
        /// Determine the number of auctions to look back.
        #[arg(value_name = "lookback", long)]
        lookback: Option<usize>,
    },
}

/// Handle the error by printing the error message and returning the exit code.
fn handle_error(e: AuctionResultError) -> i32 {
    match e {
        AuctionResultError::Request(req) => {
            println!(
                "Invalid Request, status code: {}",
                req.status().unwrap_or(StatusCode::IM_A_TEAPOT)
            );
            1
        }
        AuctionResultError::RequestDyn(_) => {
            println!("Invalid dynamic request");
            2
        }
        AuctionResultError::ParseCusip => {
            println!("Could not parse cusip number.");
            3
        }
        AuctionResultError::ParseTenor => {
            println!("Could not parse tenor.");
            4
        }
        AuctionResultError::NoTreasury => todo!(),
        AuctionResultError::OutOfBounds => todo!(),
    }
}

/// Handle the command get.
pub fn handle_get(args: &AuctionResultParser) {
    let AuctionResultCommands::Get { cusip } = &args.command else {
        exit(handle_error(AuctionResultError::ParseCusip));
    };

    let get_command = Get::new(cusip);

    let treasuries = match get_command.get() {
        Ok(vec) => vec,
        Err(e) => exit(handle_error(e)),
    };

    (if args.vertical {
        security_vprint
    } else {
        security_print
    })(&treasuries)
}

/// Handle the command lastest.
pub fn handle_latest(args: &AuctionResultParser) {
    let AuctionResultCommands::Latest {
        sectype,
        days,
        tenor,
    } = &args.command
    else {
        panic!("Cannot extract the security type and/or the number of days to look back.")
    };

    let security_type = sectype.as_ref().map_or(SecurityType::Null, |st| {
        let stype = SecurityType::from_str(st);

        if stype.is_err() {
            eprintln!("Could not parse security type: {:?}", sectype.as_ref().unwrap());
            exit(1);
        };

        stype.unwrap()
    });

    let look_back_days = days.unwrap_or(0);
    let default_tenor = String::from("");
    let tenor_str = tenor.as_ref().unwrap_or(&default_tenor);

    let Ok(tenor) = Tenor::parse(tenor_str) else {
        println!("Error parsing tenor option!");
        exit(4);
    };

    let latest_command = Latest::new(security_type, look_back_days, tenor);

    let response = latest_command.get();

    let securities = match response {
        Ok(vec) => vec,
        Err(e) => exit(handle_error(e)),
    };

    (if args.vertical {
        security_vprint
    } else {
        security_print
    })(&securities)
}

#[cfg(feature = "quality")]
/// Handle the quality command.
pub fn handle_quality(args: &AuctionResultParser) {
    use auctionresult::treasury::print::auction_quality_print;

    #[cfg(feature = "quality")]
    let AuctionResultCommands::Quality { cusip, lookback } = &args.command
    else {
        exit(handle_error(AuctionResultError::ParseCusip));
    };

    let number_of_auctions = lookback.unwrap_or(0);
    let quality_command = quality::QualityCommand::new(cusip, number_of_auctions);
    let result = quality_command.calculate();

    let Ok(q) = result else {
        exit(handle_error(result.unwrap_err()))
    };

    auction_quality_print(&q);
}
