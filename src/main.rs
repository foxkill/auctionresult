//! # The Auction Result App
mod cli;
use clap::Parser;
use cli::handle_get;
use cli::handle_latest;
#[cfg(feature = "quality")]
use cli::handle_quality;
use cli::AuctionResultCommands;
use cli::AuctionResultParser;

// TODO: Use directories crate for a directory for the cache.
// TODO: Create a command for upcoming (list the auctions for the next 7 days).
//#[clap(short = 'a', long = "print-all", conflicts_with = "report")]
// printall: bool,

fn main() {
    let args = AuctionResultParser::parse();

    match &args.command {
        AuctionResultCommands::Get { cusip: _ } => {
            handle_get(&args);
        }
        AuctionResultCommands::Latest {
            sectype: _,
            days: _,
            tenor: _,
        } => {
            handle_latest(&args);
        }
        #[cfg(feature = "quality")]
        AuctionResultCommands::Quality { cusip: _, lookback: _ } => {
            handle_quality(&args)
        }
    }
}
