use std::result::Result;
use errors::{Error, ErrorKind, ResultExt};

pub trait CumulativeErrorCollector<R> {
    fn collect_if_no_errors(self) -> Result<Vec<R>, Error>;
}

impl<T, R, E> CumulativeErrorCollector<R> for T
    where T: Iterator<Item=Result<R, E>>,
          E: Into<Error>
{
    fn collect_if_no_errors(self) -> Result<Vec<R>, Error> {
        let mut results = Vec::new();
        let mut errors = Vec::new();

        for item in self {
            match item {
                Ok(item) => results.push(item),
                Err(err) => errors.push(err.into())
            }
        }
        if errors.is_empty() {
            return Ok(results)
        }

        Err(ErrorKind::CumulativeError(errors).into())
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt::Debug;

    #[test]
    fn returns_ok_when_all_results_were_ok() {
        let results: Vec<Result<(), Error>> = vec![Ok(()), Ok(()), Ok(())];

        let actual = results.into_iter().collect_if_no_errors();

        assert!(actual.is_ok())
    }

    #[test]
    fn returns_error_if_there_is_one_error() {
        let results: Vec<Result<(), Error>> = vec![Ok(()), Err("dummy".into()), Ok(())];

        let actual = results.into_iter().collect_if_no_errors();

        assert!(actual.is_err());
    }

    #[test]
    fn returns_cumulative_error_if_there_is_one_error() {
        let results: Vec<Result<(), Error>> = vec![Ok(()), Err("dummy".into()), Ok(())];

        let actual = results.into_iter().collect_if_no_errors();

        assert_error(
            "1 error(s) occured:\n".to_owned() +
            "Error no. 1: dummy\n",
            actual
        );
    }

    #[test]
    fn returns_cumulative_errors_if_there_are_three_error() {
        let results: Vec<Result<(), Error>> = vec![Err("first".into()), Err("other".into()), Err("last".into())];

        let actual = results.into_iter().collect_if_no_errors();

        assert_error(
            "3 error(s) occured:\n".to_owned() +
            "Error no. 1: first\n" +
            "Error no. 2: other\n" +
            "Error no. 3: last\n",
            actual
        );
    }

    #[test]
    fn returns_cumulative_error_with_cause() {;
        let results = vec![
            (Err("root cause".into()) as Result<(), Error>)
            .chain_err(|| "intermediate cause")
            .chain_err(|| "error")
        ];

        let actual = results.into_iter().collect_if_no_errors();

        assert_error(
            "1 error(s) occured:\n".to_owned() +
            "Error no. 1: error\n" +
            "  caused by: intermediate cause\n" +
            "  caused by: root cause\n",
            actual
        );
    }

    #[test]
    fn returns_cause_for_every_inner_error() {;
        let results = vec![
            (Err("first cause".into()) as Result<(), Error>)
            .chain_err(|| "first error"),
            (Err("second cause".into()) as Result<(), Error>)
            .chain_err(|| "second error"),
        ];

        let actual = results.into_iter().collect_if_no_errors();

        assert_error(
            "2 error(s) occured:\n".to_owned() +
            "Error no. 1: first error\n" +
            "  caused by: first cause\n" +
            "Error no. 2: second error\n" +
            "  caused by: second cause\n",
            actual
        );
    }

    fn assert_error<T: Debug>(expected: String, result: Result<T, Error>) {
        let actual = format!("{}", result.unwrap_err());

        assert_eq!(expected, actual)
    }
}