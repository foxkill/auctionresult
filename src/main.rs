//! # The Auction Result App
//
#[allow(unused)]
use auctionresult::{Get, Latest, SecurityType, Treasury};
use clap::Parser;
use clap::Subcommand;

#[derive(Debug, Subcommand)]
enum Commands {
    /// Retrieve the details of a treasury with the given cusip number.
    #[command(arg_required_else_help = true)]
    Get { cusip: String },
    /// Retrieves the latest aution results.
    #[command(arg_required_else_help = true)]
    Latest {
        #[arg(value_name = "type", long, value_enum, require_equals = true)]
        /// The security type.
        sectype: Option<SecurityType>,
        #[arg(value_name = "days", long)]
        /// The number of days we want to look back.
        days: Option<usize>,
    },
}

#[derive(Debug, Parser)]
#[command(name = "ars")]
#[command(bin_name = "ars")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

fn main() {
    // let AuctionResult::Get(args): AuctionResult = AuctionResult::parse();

    println!("{:?}", Cli::parse());
    // println!("{:#?}", r);
}
