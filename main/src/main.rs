#![allow(unused_imports)]


use std::{fmt::format, process::ExitCode};

use argument_parsing::{
    parse_arguments,
    ParsedArgData,
};
use logging::{
    LogColours,
    Logger,
};

fn main() -> ExitCode {
    let logger = Logger::new();

    let parsed_argument_data = parse_arguments(
        std::env::args()
            .skip(1)
            .collect()
    );

    logger.log(LogColours::Info, format!("{parsed_argument_data:?}"));
    println!();


    logger.log(LogColours::Error, "Error");
    logger.log(LogColours::Warn, "Warn");
    logger.log(LogColours::Success, "Success");
    logger.log(LogColours::Info, "Info");
    logger.log(LogColours::None, "None");

    ExitCode::SUCCESS
}

