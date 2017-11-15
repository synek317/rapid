use std::env;
use std::io::Write;
use std::process::exit;
use std::result::Result as StdResult;
use error_chain::ChainedError;
use serde::de::Deserialize;
use logger::init_logging;
use consts::{ERROR_EXIT_CODE, SUCCESS_EXIT_CODE};
use utils::parse_cmd_args;
use errors::ChainedErrorMethods;
use super::App;

impl<TStdout: Write + Send + 'static, TStderr: Write + Send + 'static> App<TStdout, TStderr> {
    pub fn run<F, TError>(self, app: F) -> i32
        where F: FnOnce() -> StdResult<(), TError>,
              TError: ChainedError
    {
        let exit_code = {
            let name = self.name;
            let _guard = init_logging(self.stdout);

            log_process!("running {} with command line: {}", name, get_cmd_args() => { app() })
                .map(|_| SUCCESS_EXIT_CODE)
                .unwrap_or_else(|e| {
                    error!("One or more errors occurred while running {}:", name);
                    e.log_error();
                    ERROR_EXIT_CODE
                })
        };

        if self.exit {
            exit(exit_code)
        }

        exit_code
    }

    pub fn run_with_args<'a, F, TCmdArgs, TError>(mut self, app: F) -> i32
        where F: FnOnce(TCmdArgs) -> StdResult<(), TError>,
              TCmdArgs: Deserialize<'a>,
              TError: ChainedError
    {
        use utils::ParsingResult::*;

        match parse_cmd_args(&self.usage) {
            Args(args) => self.run(|| app(args)),
            Version => {
                let _ = writeln!(self.stdout, "{} v.{}", self.name, self.version);
                exit_if_requested(self.exit, SUCCESS_EXIT_CODE)
            },
            Help => {
                let _ = writeln!(self.stdout, "{}", self.usage);
                exit_if_requested(self.exit, SUCCESS_EXIT_CODE)
            },
            Error(e) => {
                let _ = writeln!(self.stderr, "{}", e);
                exit_if_requested(self.exit, ERROR_EXIT_CODE)
            }
        }
    }
}

fn exit_if_requested(is_exit_requested: bool, code: i32) -> i32 {
    if is_exit_requested {
        exit(code);
    }

    code
}

fn get_cmd_args() -> String {
    env::args().collect::<Vec<_>>().join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_utils::*;

    mod run {
        use super::*;
        use super::super::super::super::errors::*;

        #[test]
        fn runs_given_function() {
//            let _g = super::super::super::super::test_utils::LOCK.lock().unwrap();

            let mut executed = false;

            super::super::super::super::test_utils::OutputStream::capture(|stdout| {
                super::super::super::super::test_utils::OutputStream::capture(|stderr| {
                    App::new(stdout, stderr)
                        .run(|| -> Result<()> {
                            executed = true;
                            Ok(())
                        });
                });
            });

            assert!(executed);
        }
    }
}
