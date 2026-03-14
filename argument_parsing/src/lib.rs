pub mod structs_and_enums;
pub use structs_and_enums::{
    ParsedArgData,
    PostArgParseStep,
    ArgParseError,
    HelpScreen,
};

use std::{
    path::PathBuf,
};



fn parse_config_argument(config_argument: &str, output: &mut ParsedArgData) {
    
    match config_argument {
        help if ["+help", "help"].contains(&help) => output.next_step
            = PostArgParseStep::ShowHelpScreen(HelpScreen::Config),

        "+generate" => output.next_step 
            = PostArgParseStep::GenerateTemplateConfig,

        "+verify" => output.next_step
            = PostArgParseStep::VerifyConfigFile,

        mut name => {
            // In case the user is supplying a funky config name, they'll be
            // wrapping the config name with escaped quotes (`"` characters),
            // therefore we have to remove them if they are at both the
            // begining, and end. I'm not gonna bother with more indepth
            // parsing, because at that point just rename the file nerd!
            if name.len() >= 2
                && name.starts_with("\"") && name.ends_with("\"") {
                name = &name[ 1 .. name.len() -1 ];
            }

            if name.is_empty() {
                output.next_step = PostArgParseStep::Fail(
                    ArgParseError::MissingConfigPath
                );
                return;
            }


            output.config_path = PathBuf::from(name)
        },
    }
}


pub fn parse_arguments(arguments: Vec<String>) -> ParsedArgData {
    let mut arguments: Vec<&str> = arguments
        .iter()
        .map(AsRef::as_ref)
        .collect();
    
    let mut output = ParsedArgData {
        config_path: PathBuf::from("speecher_config.toml"),
        next_step: PostArgParseStep::ProceedAsUsual
    };

    while !arguments.is_empty() {
        
        match arguments[..] {
            ["--config", config_argument, ..] => {
                parse_config_argument(config_argument, &mut output);
                arguments.drain(0..2);
            },
            ["--config"] => {
                output.next_step = PostArgParseStep::Fail(
                    ArgParseError::MissingConfigPath
                );
                return output;
            }

            [_, ..] => { arguments.drain(0..1); },
            
            [] => {}  // If empty, then we break out of the loop anyways
        }
    }

    output
}

