use anyhow::{Ok, Result};
use clap::Parser;
use ffs::types::args::Args;
use ffs::FFS;
use human_panic::setup_panic;
mod types;

fn main() -> Result<()> {
    setup_panic!();

    let args = Args::parse();

    FFS::new(args).execute();

    Ok(())
}
