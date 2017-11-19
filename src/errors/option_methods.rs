use std::fmt::{Display, Debug};
use failure::{Error, err_msg};

pub trait OptionMethods {
    type TOk;

    fn ok_or_error<D>(self, context: D) -> Result<Self::TOk, Error> where
        D: Into<Error>;

    fn ok_or_else_error<F, D>(self, f: F) -> Result<Self::TOk, Error> where
        F: FnOnce() -> D,
        D: Into<Error>;

    fn ok_or_error_msg<D>(self, msg: D) -> Result<Self::TOk, Error> where
        D: Display + Debug + Send + Sync + 'static;

    fn ok_or_else_error_msg<F, D>(self, f: F) -> Result<Self::TOk, Error> where
        F: FnOnce() -> D,
        D: Display + Debug + Send + Sync + 'static;
}

impl<T> OptionMethods for Option<T> {
    type TOk = T;

    fn ok_or_error<D>(self, context: D) -> Result<Self::TOk, Error> where
        D: Into<Error>
    {
        self.ok_or(context).map_err(Into::into)
    }

    fn ok_or_else_error<F, D>(self, f: F) -> Result<Self::TOk, Error> where
        F: FnOnce() -> D,
        D: Into<Error>
    {
        self.ok_or_else(|| f().into())
    }

    fn ok_or_error_msg<D>(self, msg: D) -> Result<Self::TOk, Error> where
        D: Display + Debug + Send + Sync + 'static
    {
        self.ok_or_error(err_msg(msg))
    }

    fn ok_or_else_error_msg<F, D>(self, f: F) -> Result<Self::TOk, Error> where
        F: FnOnce() -> D,
        D: Display + Debug + Send + Sync + 'static
    {
        self.ok_or_else_error(|| err_msg(f()))
    }
}
