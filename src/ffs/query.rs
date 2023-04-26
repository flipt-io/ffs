use anyhow::Result;
use flipt::api::flag::{FlagClient, FlagGetRequest};

pub struct Querier<'a> {
    client: FlagClient<'a>,
}

impl Querier<'_> {
    pub fn new(client: FlagClient<'_>) -> Querier<'_> {
        Querier { client }
    }

    pub async fn query(&self, query: &str) -> Result<()> {
        self.client
            .get(&FlagGetRequest {
                namespace_key: None,
                key: query.to_string(),
            })
            .await?;
        Ok(())
    }
}
