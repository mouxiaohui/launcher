use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    pub site_key: String,

    pub key_word: String,

    /// Configuration to use.
    #[arg(short, long, value_parser)]
    pub config: Option<String>,
}
