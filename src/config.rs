use std::fs;

use anyhow::{Context, Result};
use serde::Deserialize;

pub const DEFAULT_CONFIG: &[u8] = br#"[
  {
    "name": "baidu",
    "key": ["baidu", "bd"],
    "url": "https://www.baidu.com/s?wd={}"
  },
  {
    "name": "google",
    "key": ["google", "gl"],
    "url": "https://www.google.com/search?q={}"
  },
  {
    "name": "crates.io",
    "key": ["crates.io", "cio"],
    "url": "https://crates.io/crates/{}"
  },
  {
    "name": "docs.rs",
    "key": ["docs.rs", "doc"],
    "url": "https://docs.rs/{}"
  },
  {
    "name": "rust-std",
    "key": ["rust-std", "std"],
    "url": "https://doc.rust-lang.org/std/"
  }
]
"#;

#[derive(Debug, Deserialize)]
pub struct Sites(Vec<Site>);

#[derive(Debug, Clone, Deserialize)]
pub struct Site {
    // name: String,
    key: Vec<String>,
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
        self.0.iter().find(|x| x.match_from_key(key))
    }
}

impl Site {
    pub fn join(&self, key_word: Option<String>) -> String {
        if let Some(key_word) = key_word {
            self.url.replace("{}", &key_word)
        } else {
            self.url.clone()
        }
    }

    fn match_from_key(&self, key: &str) -> bool {
        self.key.iter().any(|x| x == key)
    }
}
