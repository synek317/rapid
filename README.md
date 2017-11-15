# Warning

This is work in progress. The core functionality works and the crate may be used, but it is missing documentation and most of unit tests.

# Overview

Rapid allows rapid console application development. It bundles a bunch of common dependencies and provides functions to allow you to create application rapidly.

Included features and dependencies:

- parsing command-line arguments using [Docopt](https://github.com/docopt/docopt.rs) syntax
- handling errors with [error-chain](https://github.com/rust-lang-nursery/error-chain)
- setup logging with [slog](https://github.com/slog-rs/slog)
- setup global logger for [log](https://github.com/rust-lang-nursery/log) crate
- [lazy-static](https://github.com/rust-lang-nursery/lazy-static.rs)