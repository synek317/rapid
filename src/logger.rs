use std::io::Write;
use slog::{Drain, Logger};
use slog_term::{CompactFormat, PlainSyncDecorator};
use slog_async::Async;
use slog_scope::{GlobalLoggerGuard, set_global_logger};
use slog_stdlog;

pub fn create_logger<O: Write + Send + 'static>(output: O) -> Logger  {
    let decorator = PlainSyncDecorator::new(output);
    let drain     = CompactFormat::new(decorator).build().fuse();
    let drain     = Async::new(drain).build().fuse();

    Logger::root(drain, slog_o!())
}

pub fn init_logging<O: Write + Send + 'static>(output: O) -> GlobalLoggerGuard  {
    let logger      = create_logger(output);
    let scope_guard = set_global_logger(logger);
    let _log_guard  = slog_stdlog::init();

    scope_guard
}

#[macro_export]
macro_rules! log_process {
    ($msg:expr, $($params:expr),* => $block:block) => {{
        info!(concat!("Start - ", $msg), $($params),*);
        let (result, taken) = run_and_measure!($block);
        info!(concat!("End - ", $msg, ". It took {} ms"), $($params),*, taken);
        result
    }};

    ($msg:expr, $($params:expr),* => $block:block) => {{
        info!(concat!("Start - ", $msg), $($params),*);
        let (result, taken) = run_and_measure!($block);
        info!(concat!("End - ", $msg, ". It took {} ms"), $($params),*, taken);
        result
    }};

    ($msg:expr => $block:block) => {{
        info!(concat!("Start - ", $msg));
        let (result, taken) = run_and_measure!($block);
        info!(concat!("End - ", $msg, ". It took {} ms"), taken);
        result
    }}
}

#[macro_export]
macro_rules! run_and_measure {
    ($block:block) => {{
        use $crate::time;

        let start = time::precise_time_ns();
        let result = (|| $block)();
        let taken = ((time::precise_time_ns() - start) as f64)/1000000f64;

        (result, taken)
    }}
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_utils::OutputStream;

    #[test]
    fn work_in_progress() {
        //check no panic
        let _output = OutputStream::capture(|output_stream| {
            let _guard = init_logging(output_stream);
            info!("x");
        });

        assert_eq!("dummy", "dummy")
    }
}
