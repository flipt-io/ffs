use anyhow::{Ok, Result};
use clap::Parser;
use human_panic::setup_panic;

use crate::{
    ffs::{query::Querier, scanner::Scanner, writer::Writer},
    types::args::Args,
};
mod ffs;
mod types;

#[tokio::main]
async fn main() -> Result<()> {
    setup_panic!();

    let args = Args::parse();

    let mut ffs = Scanner::new(args.language, args.dir);
    let tokens = ffs.scan()?;
    let writer = Writer::new(args.output);

    writer.write(&tokens)?;

    flipt::meta::MetaClient::new(flipt::Config::default())?
        .info()
        .get()
        .await?;

    let flipt_client = flipt::api::ApiClient::new(flipt::Config::default())?;

    let querier = Querier::new(flipt_client.flags());
    for k in tokens.keys() {
        querier.query(k).await?;
    }

    Ok(())
}
