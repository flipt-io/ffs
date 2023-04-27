use anyhow::{Ok, Result};
use clap::Parser;
use human_panic::setup_panic;
use types::flag::Flag;

use crate::{ffs::scanner::Scanner, types::args::Args};
mod ffs;
mod types;

fn main() -> Result<()> {
    setup_panic!();

    let args = Args::parse();

    let mut ffs = Scanner::new(args.language, args.dir);
    let flags = ffs.scan()?;
    write_output(flags, args.output)?;

    // flipt::meta::MetaClient::new(flipt::Config::default())?
    //     .info()
    //     .get()
    //     .await?;
    //
    // let flipt_client = flipt::api::ApiClient::new(flipt::Config::default())?;
    //
    // for k in tokens.keys() {
    //     flipt_client
    //         .flags()
    //         .get(&flipt::api::flag::FlagGetRequest {
    //             namespace_key: None,
    //             key: k.to_string(),
    //         })
    //         .await?;
    // }
    //
    Ok(())
}

fn write_output(flags: Vec<Flag>, to: Option<String>) -> Result<()> {
    let mut out_writer: Box<dyn std::io::Write> = match to {
        Some(s) => Box::new(std::fs::File::create(s)?),
        None => Box::new(std::io::stdout()),
    };

    for f in flags {
        let json = serde_json::to_string(&f)?;
        writeln!(out_writer, "{json}")?;
    }

    Ok(())
}
