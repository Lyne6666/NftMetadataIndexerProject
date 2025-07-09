// src/main.rs
/*
 * Main executable for NftMetadataIndexerProject
 */

use clap::Parser;
use nftmetadataindexerproject::{Result, run};

#[derive(Parser)]
#[command(version, about = "NftMetadataIndexerProject - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
