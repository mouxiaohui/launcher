use std::fs;

use anyhow::{Context, Result};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Sites(Vec<Site>);

#[derive(Debug, Clone, Deserialize)]
pub struct Site {
    name: String,
    key: String,
    url: String,
}

impl Sites {
    pub fn load_json(path: &str) -> Result<Self> {
        let content =
            fs::read_to_string(path).context(format!("Failed to read file: \"{}\"", &path))?;
        Self::from_json(&content)
    }

    pub fn from_json(content: &str) -> anyhow::Result<Self> {
        Ok(serde_json::from_str(content).context("Failed to parse json file")?)
    }

    pub fn get_site(&self, key: &str) -> Option<&Site> {
        self.0.iter().find(|v| v.key == key)
    }
}

impl Site {
    pub fn join(&self, key_word: &str) -> String {
        self.url.replace("{}", key_word)
    }
}
