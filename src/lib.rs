pub extern crate time;
#[macro_use(slog_o, slog_kv)]
extern crate slog;
extern crate slog_stdlog;
extern crate slog_scope;
extern crate slog_term;
extern crate slog_async;
#[macro_use]
extern crate error_chain;
extern crate serde;
#[macro_use]
extern crate log;
extern crate docopt;
#[allow(unused_imports)] //it is only re-exported
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;

#[macro_use]
pub mod logger;
#[macro_use]
mod errors;
mod app;
mod utils;
mod consts;
#[cfg(test)]
mod test_utils;

pub use error_chain::ChainedError;
pub use errors::*;
pub use std::result::Result as StdResult;
pub use app::App;

//re-export useful macros
pub use log::*;
pub use error_chain::*;
pub use serde_derive::*;
