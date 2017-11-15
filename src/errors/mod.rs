mod chained_error_methods;
mod cumulative_error_collector;

pub use self::chained_error_methods::ChainedErrorMethods;
pub use self::cumulative_error_collector::*;

use std::fmt::Write;

error_chain! {
    errors {
        CumulativeError(errors: Vec<Error>) {
            cause("multiple errors occured")
            display("{}", collect_errors(errors))
        }
    }
}

fn collect_errors(errors: &Vec<Error>) -> String {
    let mut output = String::with_capacity(255);

    let _ = write!(output, "{} error(s) occured:", errors.len());

    for (i, error) in errors.iter().enumerate() {
        let _ = write!(output, "\nError no. {}: {}", i + 1, error);

        for inner in error.iter().skip(1) {
            let _ = write!(output, "\n  caused by: {}", inner);
        }
    }

    writeln!(output);

    output
}
