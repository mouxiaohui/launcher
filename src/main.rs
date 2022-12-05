use std::env;

use anyhow::{anyhow, Result};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Sites(Vec<Site>);

#[derive(Debug, Deserialize)]
pub struct Site {
    name: String,
    key: String,
    url: String,
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        return Err(anyhow!("Missing Parameters"));
    }

    let site_key: &str = args[1].as_ref();
    let keyword: &str = args[2].as_ref();

    let content = std::fs::read_to_string("./config.json")?;
    let config: Sites = serde_json::from_str(&content)?;
    println!("{:?}", config);

    Ok(())
}
