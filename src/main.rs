use std::collections::HashMap;

use anyhow::{bail, Ok, Result};
use clap::Parser;
use human_panic::setup_panic;

use crate::{
    ffs::scanner::Scanner,
    types::{args::Args, flag::Flag},
};
mod ffs;
mod types;

#[tokio::main]
async fn main() -> Result<()> {
    setup_panic!();

    let args = Args::parse();

    let mut ffs = Scanner::new(args.language, args.dir);

    let found_flags = ffs.scan()?;

    let found_flags_set: HashMap<String, Vec<Flag>> = found_flags
        .iter()
        .cloned()
        .filter(|f| f.namespace_key == args.namespace.clone().unwrap_or("default".to_string()))
        .fold(HashMap::new(), |mut acc, f| {
            acc.entry(f.flag_key.clone())
                .or_insert_with(Vec::new)
                .push(f);
            acc
        });

    let flipt_config = flipt::Config::default();

    // checks if the flipt server is up and running
    flipt::meta::MetaClient::new(flipt_config.clone())?
        .info()
        .get()
        .await?;

    let flipt_client = flipt::api::ApiClient::new(flipt_config)?;

    // TODO: paginate
    let existing_flags = flipt_client
        .flags()
        .list(&flipt::api::flag::FlagListRequest {
            namespace_key: args.namespace,
            ..Default::default()
        })
        .await?;

    let existing_flags_set: HashMap<_, _> = existing_flags
        .flags
        .iter()
        .map(|f| (f.key.clone(), f))
        .collect();

    let mut out_writer: Box<dyn std::io::Write> = match args.output {
        Some(s) => Box::new(std::fs::File::create(s)?),
        None => Box::new(std::io::stdout()),
    };

    // get collection of found flags and their locations in code that do not existing in flipt
    let missing_flags: Vec<_> = found_flags_set
        .iter()
        .filter(|(k, _)| !existing_flags_set.contains_key(k.as_str()))
        .flat_map(|(_, v)| v)
        .collect();

    // ensure all found flags exist in flipt, write to output if not
    for flag in &missing_flags {
        let json = serde_json::to_string(&flag)?;
        writeln!(out_writer, "{json}")?;
    }

    if !missing_flags.is_empty() {
        bail!("Found {} missing flags", missing_flags.len());
    }

    Ok(())
}
