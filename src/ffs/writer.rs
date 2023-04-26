use crate::types::token::{Token, TokenSet};
use anyhow::Result;

pub struct Writer {
    output: Option<String>,
}

impl Writer {
    pub fn new(output: Option<String>) -> Self {
        Writer { output }
    }

    pub fn write(&self, tokens: &TokenSet) -> Result<()> {
        let mut out_writer: Box<dyn std::io::Write> = match &self.output {
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
}
