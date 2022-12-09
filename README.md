# Launcher

Quickly open the website on the command line

## Usage

```sh
$ lc std

# Carry keyword
$ lc doc tokio
```

```sh
$ lc --help
Usage: lc.exe [OPTIONS] <SITE_KEY> <KEY_WORD>

Arguments:
  <SITE_KEY>
  <KEY_WORD>

Options:
  -c, --config <CONFIG>  Configuration to use
  -h, --help             Print help information
  -V, --version          Print version information
```

configuration example

Lin: ~/.config/launcher/config.json

Win: ~\AppData\Roaming\launcher\config.json

Mac: ~/Library/Application Suppor/launcher/config.json

```json
[
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
```
