use anyhow::{Ok, Result};
use clap::Parser;
use human_panic::setup_panic;
use types::token::{Token, TokenSet};

use crate::{ffs::scanner::Scanner, types::args::Args};
mod ffs;
mod types;

#[tokio::main]
async fn main() -> Result<()> {
    setup_panic!();

    let args = Args::parse();

    let mut ffs = Scanner::new(args.language, args.dir);
    let tokens = ffs.scan()?;

    flipt::meta::MetaClient::new(flipt::Config::default())?
        .info()
        .get()
        .await?;

    let flipt_client = flipt::api::ApiClient::new(flipt::Config::default())?;

    for k in tokens.keys() {
        flipt_client
            .flags()
            .get(&flipt::api::flag::FlagGetRequest {
                namespace_key: None,
                key: k.to_string(),
            })
            .await?;
    }

    Ok(())
}

#[allow(dead_code)]
fn write_output(tokens: &TokenSet, to: Option<String>) -> Result<()> {
    let mut out_writer: Box<dyn std::io::Write> = match to {
        Some(s) => Box::new(std::fs::File::create(s)?),
        None => Box::new(std::io::stdout()),
    };

    for (k, v) in tokens {
        for loc in v {
            let t = Token {
                key: k.to_string(),
                loc: loc.clone(),
            };

            let json = serde_json::to_string(&t)?;
            writeln!(out_writer, "{json}")?;
        }
    }

    Ok(())
}
