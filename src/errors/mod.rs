pub mod fail_methods;
mod result_methods;
mod option_methods;
mod cumulative_error_collector;

pub use self::fail_methods::*;
pub use self::result_methods::*;
pub use self::option_methods::*;
pub use self::cumulative_error_collector::*;

use std::result::Result as StdResult;
use std::fmt;

pub type Result<T> = StdResult<T, ::failure::Error>;

#[derive(Debug)]
pub struct CumulativeError<T> {
    errors: Vec<T>
}

impl<T> ::failure::Fail for CumulativeError<T> where T: fmt::Debug + fmt::Display + Send + Sync + 'static{}

impl<T> CumulativeError<T> {
    pub fn new(errors: Vec<T>) -> Self {
        Self { errors }
    }
}

impl<T> fmt::Display for CumulativeError<T> where T: fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let _ = write!(f, "{} error(s) occured:", self.errors.len())?;

        for (i, error) in self.errors.iter().enumerate() {
            let _ = write!(f, "\nError no. {}: {}", i + 1, error)?;
        }

        Ok(())
    }
}

#[macro_export]
macro_rules! bail {
    ($($arg:tt)*) => { return Err(format_err!($($arg)*)); }
}
