use anyhow::{anyhow, Result};
use clap::Parser;

use crate::args::Args;
use crate::config::Sites;

mod args;
mod config;

fn main() -> Result<()> {
    let url = get_url(Args::parse())?;
    run(&url)
}

fn get_url(args: Args) -> Result<String> {
    let config_file = &args.config.unwrap_or_else(|| String::from("./config.json"));
    match Sites::load_json(&config_file)?.get_site(&args.site_key) {
        Some(site) => Ok(site.join(&args.key_word)),
        None => Err(
            anyhow!("The specified website was not found from the configuration file").context(
                format!("Con't found `{}` in \"{}\"", args.key_word, config_file),
            ),
        ),
    }
}

fn run(url: &str) -> Result<()> {
    println!("{:?}", url);
    Ok(())
}
