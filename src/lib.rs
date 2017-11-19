pub extern crate time;
#[macro_use(slog_o, slog_kv)]
extern crate slog;
extern crate slog_stdlog;
extern crate slog_scope;
extern crate slog_term;
extern crate slog_async;
extern crate serde;
#[macro_use]
extern crate log;
extern crate docopt;
#[allow(unused_imports)] //it is only re-exported
#[macro_use]
extern crate serde_derive;
#[allow(unused_imports)] //it is only re-exported in prod use
#[macro_use]
extern crate lazy_static;
#[allow(unused_imports)]
#[macro_use]
extern crate failure;
#[allow(unused_imports)]
#[macro_use]
extern crate failure_derive;

#[macro_use]
pub mod logger;
#[macro_use]
mod errors;
mod app;
mod utils;
mod consts;
#[cfg(test)]
mod test_utils;

pub use errors::*;
pub use std::result::Result as StdResult;
pub use app::App;

//re-export useful macros
pub use log::*;
pub use serde_derive::*;
pub use lazy_static::*;
pub use failure::*;
