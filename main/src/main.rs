#![allow(unused_imports)]


use std::process::ExitCode;

use argument_parsing::{
    parse_arguments,
    ArgumentsParseError,
    ArgumentsParseResult,
};

fn main() -> ExitCode {
    let env_args = std::env::args()
        .skip(1)
        .collect();

    let arguments_parsing_result = match parse_arguments(env_args) {
        Err( ArgumentsParseError::MalformedArguments(message) ) => {
            eprintln!("Failed to parse arguments: {message}");
            return ExitCode::from(101);
        },
        Err( ArgumentsParseError::EmptyConfigName ) => {
            eprintln!("Failed to parse arguments: Provided config name is empty");
            return ExitCode::from(102);
        },
        Err( ArgumentsParseError::MissingConfigPath ) => {
            eprintln!("Config path not provided!");
            return ExitCode::from(103);
        }
        Ok(parse_result) => parse_result
    };


    println!("Hello, world!");
    println!("{arguments_parsing_result:?}");

    ExitCode::SUCCESS
}
