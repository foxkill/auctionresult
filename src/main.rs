//! # The Auction Result App
mod cli;
use clap::Parser;
use cli::handle_get;
use cli::handle_latest;
use cli::AuctionResultCommands;
use cli::AuctionResultParser;

#[allow(unused)]
use auctionresult::{Get, Latest, SecurityType, Treasury, TreasuryAccess};

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
        AuctionResultCommands::Quality { cusip: _ } => {
            todo!()
        }
    }
}
