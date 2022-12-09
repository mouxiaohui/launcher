use std::io::Write;

use anyhow::{anyhow, Context, Result};
use clap::Parser;

use crate::args::Args;
use crate::config::Sites;

mod args;
mod config;

fn main() -> Result<()> {
    let url = get_url(Args::parse())?;
    open::that(url)?;

    Ok(())
}

fn get_url(args: Args) -> Result<String> {
    let config_file = match args.config {
        Some(v) => v,
        None => get_config_dir()?,
    };
    match Sites::load_json(&config_file)?.get_site(&args.site_key) {
        Some(site) => Ok(site.join(args.key_word)),
        None => Err(
            anyhow!("The specified website was not found from the configuration file").context(
                format!(
                    "Can't found `{}` in \"{}\"",
                    args.site_key,
                    config_file
                ),
            ),
        ),
    }
}

fn get_config_dir() -> Result<String> {
    let mut path = dirs::config_dir()
        .ok_or_else(|| anyhow!("The configuration file directory could not be found"))?;

    path.push("launcher");
    if !path.exists() {
        std::fs::create_dir(&path).context(format!("Faild to create file: \"{:?}\"", &path))?;
    }

    path.push("config.json");
    if !path.exists() {
        let mut file = std::fs::File::create(&path)
            .context(format!("Faild to create file: \"{:?}\"", &path))?;
        file.write_all(&config::DEFAULT_CONFIG)
            .context(format!("Faild to write file: \"{:?}\"", &path))?;
        file.flush()?;
    }

    Ok(path
        .to_str()
        .ok_or_else(|| anyhow!("Incorrect directory path"))?
        .to_owned())
}
