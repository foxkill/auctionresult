//! # The Auction Result App
use std::str::FromStr;

use auctionresult::treasury::print::horizontally_print_out_treasury;
use auctionresult::vertically_print_out_treasury;
//
#[allow(unused)]
use auctionresult::{Get, Latest, SecurityType, Treasury, TreasuryAccess};
use clap::Parser;
use clap::Subcommand;

// The Parser.
#[derive(Debug, Parser)]
#[command(name = "ars")]
#[command(bin_name = "ars")]
#[command(author = "Stefan M. <foxkill@gmx.de>")]
#[command(about = "Retrieves information about US Treasuries und US Treasury auctions of them.")]
#[command(version)]
struct AuctionResultParser {
    #[arg(short = 'E', long, value_name = "vertical")]
    /// Display result not as a table.
    vertical: bool,
    #[clap(subcommand)]
    command: AuctionResultCommands,
}

// The Sub Commands.
#[derive(Debug, Subcommand)]
#[clap(author, version, about)]
enum AuctionResultCommands {
    #[command(arg_required_else_help = true)]
    /// Retrieve informations about a specific security.
    Get {
        #[arg(value_name = "cusip")]
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

fn main() {
    let args = AuctionResultParser::parse();
    match &args.command {
        AuctionResultCommands::Get { cusip } => {
            let get_command = Get::new(cusip);

            let response: Vec<Treasury> = get_command.get();

            if args.vertical {
                vertically_print_out_treasury(&response);
            } else {
                horizontally_print_out_treasury(&response);
            }

        }
        AuctionResultCommands::Latest { sectype, days } => {
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
    }
}
