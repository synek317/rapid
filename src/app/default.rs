use std::env;
use std::io::{self, Stdout, Stderr};
use super::App;

const DEFAULT_USAGE: &'static str = "
Does nothing. Rapidly.

Usage:
  program (-h | --help)
  program (-v | --version)

Options:
  -h --help          Show this screen
  -v --version       Show version
";

impl Default for App<Stdout, Stderr> {
    fn default() -> Self {
        Self {
            name:    env::var("CARGO_PKG_NAME").unwrap_or_else(|_| env!("CARGO_PKG_NAME").to_string()),
            usage:   DEFAULT_USAGE.to_string(),
            version: env::var("CARGO_PKG_VERSION").unwrap_or_else(|_| env!("CARGO_PKG_VERSION").to_string()),
            stdout:  io::stdout(),
            stderr:  io::stderr(),
            exit:    true
        }
    }
}
