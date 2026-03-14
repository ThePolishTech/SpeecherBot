#![allow(unused_imports)]


use std::process::ExitCode;

use argument_parsing::{
    parse_arguments,
    ParsedArgData,
};

fn main() -> ExitCode {
    let parsed_argument_data = parse_arguments(
        std::env::args()
            .skip(1)
            .collect()
    );

    println!("Formated data: {parsed_argument_data:?}");

    ExitCode::SUCCESS
}

