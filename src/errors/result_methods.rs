use std::fmt::{Display, Debug};
use failure::{Fail, Error, ResultExt, err_msg};
use errors::FailMethods;

pub trait ResultMethods {
    type TOk;
    type TError;

    fn if_error_log_error(&self) -> &Self;

    fn if_error_log_error_and_ignore(&self) {
        let _ = self.if_error_log_error();
    }

    fn if_error_log_warning(&self) -> &Self;

    fn if_error_log_warning_and_ignore(&self) {
        let _ = self.if_error_log_warning();
    }

    fn context_error<D>(self, context: D) -> Result<Self::TOk, Error> where
        D: Display + Send + Sync + 'static;

    fn with_context_error<F, D>(self, f: F) -> Result<Self::TOk, Error> where
        F: FnOnce(&Self::TError) -> D,
        D: Display + Send + Sync + 'static;

    fn context_error_msg<D>(self, msg: D) -> Result<Self::TOk, Error> where
        D: Display + Debug + Send + Sync + 'static;

    fn with_context_error_msg<F, D>(self, f: F) -> Result<Self::TOk, Error> where
        F: FnOnce() -> D,
        D: Display + Debug + Send + Sync + 'static;
}

impl<T, E> ResultMethods for Result<T, E> where E: Fail {
    type TOk = T;
    type TError = E;

    fn if_error_log_error(&self) -> &Self {
        if let Err(ref e) = *self {
            e.log_error()
        }

        self
    }

    fn if_error_log_warning(&self) -> &Self {
        if let Err(ref e) = *self {
            e.log_warning()
        }

        self
    }

    fn context_error<D>(self, context: D) -> Result<T, Error> where
        D: Display + Send + Sync + 'static
    {
        self.context(context).map_err(Into::into)
    }

    fn with_context_error<F, D>(self, f: F) -> Result<T, Error> where
        F: FnOnce(&E) -> D,
        D: Display + Send + Sync + 'static
    {
        self.with_context(f).map_err(Into::into)
    }

    fn context_error_msg<D>(self, msg: D) -> Result<T, Error> where
        D: Display + Debug + Send + Sync + 'static
    {
        self.context_error(err_msg(msg))
    }

    fn with_context_error_msg<F, D>(self, f: F) -> Result<T, Error> where
        F: FnOnce() -> D,
        D: Display + Debug + Send + Sync + 'static
    {
        self.with_context_error(|_| err_msg(f()))
    }
}
