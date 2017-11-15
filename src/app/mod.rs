mod run;
mod default;

use std::io::Write;

pub use self::run::*;
pub use self::default::*;

pub struct App<TStdout, TStderr>
    where TStdout: Write + Send + 'static,
          TStderr: Write + Send + 'static
{
    name:    String,
    usage:   String,
    version: String,
    stdout:  TStdout,
    stderr:  TStderr,
    exit:    bool
}

impl<TStdout: Write + Send + 'static, TStderr: Write + Send + 'static> App<TStdout, TStderr> {
    pub fn new(stdout: TStdout, stderr: TStderr) -> Self {
        Self {
            name:    String::default(),
            usage:   String::default(),
            version: String::default(),
            exit:    true,
            stdout,
            stderr
        }
    }

    pub fn name<S: Into<String>>(mut self, value: S) -> Self {
        self.name = value.into();
        self
    }

    pub fn usage<S: Into<String>>(mut self, value: S) -> Self {
        self.usage = value.into();
        self
    }

    pub fn version<S: Into<String>>(mut self, value: S) -> Self {
        self.version = value.into();
        self
    }

    pub fn stdout<S: Into<TStdout>>(mut self, value: S) -> Self {
        self.stdout= value.into();
        self
    }

    pub fn stderr<S: Into<TStderr>>(mut self, value: S) -> Self {
        self.stderr = value.into();
        self
    }

    pub fn exit<S: Into<bool>>(mut self, value: S) -> Self {
        self.exit= value.into();
        self
    }
}
