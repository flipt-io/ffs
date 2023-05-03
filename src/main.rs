use std::process::ExitCode;

use anyhow::Result;
use clap::Parser;
use human_panic::setup_panic;

use crate::{
    ffs::scanner::Scanner,
    types::{
        flag::Flag,
        args::Args,
    },
};

use futures::future::join_all;

mod ffs;
mod types;

#[tokio::main]
async fn main() -> Result<ExitCode> {
    setup_panic!();

    let args = Args::parse();

    let mut ffs = Scanner::new(args.language, args.dir);

    let found_flags = ffs.scan()?;

    let flipt_config = flipt::Config::new_from_env().unwrap_or_default();

    // checks if the flipt server is up and running
    flipt::meta::MetaClient::new(flipt_config.clone())?
        .info()
        .get()
        .await?;

    let flipt_client = &flipt::api::ApiClient::new(flipt_config)?;

    let flag_results : Vec<Flag> = join_all(found_flags
        .into_iter()
        .map(|f| async move {
            let resp = flipt_client.flags().get(&flipt::api::flag::FlagGetRequest{
                namespace_key: Some(f.namespace_key.clone()),
                key: f.flag_key.clone(),
            }).await;

            match resp {
                Ok(_) => None,
                Err(error) => {
                    match error.downcast_ref::<flipt::error::Error>() {
                        Some(flipt::error::Error::Upstream(e)) => {
                            if e.code == 5 {
                                Some(f)
                            } else {
                                None
                            }
                        },
                        _ => None,
                    }
                },
            }
        })).await.into_iter().flatten().collect();

    let mut out_writer: Box<dyn std::io::Write> = match args.output {
        Some(s) => Box::new(std::fs::File::create(s)?),
        None => Box::new(std::io::stdout()),
    };


    // ensure all found flags exist in flipt, write to output if not
    if !flag_results.is_empty() {
        let json = serde_json::to_string(&flag_results)?;
        writeln!(out_writer, "{json}")?;
    }

    Ok(ExitCode::SUCCESS)
}
