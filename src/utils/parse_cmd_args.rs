use std::result::Result;
use docopt::{ArgvMap, Docopt, Error as DocoptError};
use serde::de::Deserialize;

type DocoptResult<T> = Result<T, DocoptError>;

pub enum ParsingResult<TCmdArgs> {
    Args(TCmdArgs),
    Version,
    Help,
    Error(DocoptError)
}

pub fn parse_cmd_args<'a, TCmdArgs>(usage: &str) -> ParsingResult<TCmdArgs>
    where TCmdArgs: Deserialize<'a>
{
    match parse_args(usage) {
        Ok(ref argv_map) if is_help(argv_map)    => ParsingResult::Help,
        Ok(ref argv_map) if is_version(argv_map) => ParsingResult::Version,
        Ok(argv_map)                             => deserialize_args(argv_map),
        Err(e)                                   => ParsingResult::Error(e),
    }
}

fn parse_args(usage: &str) -> DocoptResult<ArgvMap> {
    Docopt::new(usage)
        .map(|docopt| docopt
            .help(false)
            .version(None)
        )
        .and_then(|d| d.parse())
}

fn is_help(argv_map: &ArgvMap) -> bool {
    argv_map.get_bool("--help") || argv_map.get_bool("-h")
}

fn is_version(argv_map: &ArgvMap) -> bool {
    argv_map.get_bool("--version") || argv_map.get_bool("-v")
}

fn deserialize_args<'a, TCmdArgs>(argv_map: ArgvMap) -> ParsingResult<TCmdArgs>
    where TCmdArgs: Deserialize<'a>
{
    match argv_map.deserialize() {
        Ok(args) => ParsingResult::Args(args),
        Err(e)   => ParsingResult::Error(e)
    }
}
