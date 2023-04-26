use anyhow::{Ok, Result};
use clap::Parser;
use human_panic::setup_panic;

use crate::{
    ffs::{scanner::Scanner, writer::Writer},
    types::args::Args,
};
mod ffs;
mod types;

fn main() -> Result<()> {
    setup_panic!();

    let args = Args::parse();

    let mut ffs = Scanner::new(args.language, args.dir);
    let tokens = ffs.scan()?;
    let writer = Writer::new(args.output);

    writer.write(tokens)?;

    Ok(())
}
