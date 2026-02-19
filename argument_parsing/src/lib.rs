
#[derive(Debug, PartialEq)]
pub struct ArgumentsParseResult {
    pub overwritten_config_path: Option<String>,
    pub action: ArgumentsAction
}

#[derive(Debug, PartialEq)]
pub enum ArgumentsAction {
    ProceedAsUsual,
    HelpScreen(HelpScreenOptions),

    ConfigAction(ConfigAction),
}

#[derive(Debug, PartialEq)]
pub enum HelpScreenOptions {
    Default,
    Config
}

#[derive(Debug, PartialEq)]
pub enum ConfigAction {
    Verify,
    Generate,
}

#[derive(Debug, PartialEq)]
pub enum ArgumentsParseError {
    MalformedArguments(String)
}

pub fn parse_arguments(arguments: &str) -> Result<ArgumentsParseResult, ArgumentsParseError> {
    let mut output = ArgumentsParseResult {
        overwritten_config_path: None,
        action: ArgumentsAction::ProceedAsUsual
    };

    let mut args = arguments.split(" ").collect::<Vec<_>>();

    while !args.is_empty() {
        match args[..] {
            ["--config", argument, ..] => {
                match argument {
                    "+verify" => {
                        output.action = ArgumentsAction::ConfigAction(ConfigAction::Verify);
                    },
                    "+generate" => {
                        output.action = ArgumentsAction::ConfigAction(ConfigAction::Generate);
                    },
                    other if ["help", "+help"].contains(&other) => {
                        output.action = ArgumentsAction::HelpScreen(HelpScreenOptions::Config);
                    },
                    path => {
                        output.overwritten_config_path = Some(
                            path.to_string()
                                .replace("\"", "")
                                .replace("'", "")
                                .replace("`", "")
                        );
                    }
                }
                args.drain(0..2);
            }
            [unknown, ..] => {
                args.drain(0..1);
                return Err(ArgumentsParseError::MalformedArguments(
                    format!("Unkown argument: {unknown}")
                ));
            },
            _ => {
                args.drain(0..1);
            }
        }
    }
    Ok(output)
}





#[cfg(test)]
mod config_tests {
    #![allow(dead_code, unused_variables)]
    
    #[allow(unused_imports)]
    use crate::{
        parse_arguments,
        ArgumentsAction,
        ArgumentsParseError,
        ArgumentsParseResult,

        ConfigAction,
        HelpScreenOptions,
    };


    #[test]
    fn parse_file_path() {
        let example = "--config speecher_config.toml";
        let expected: Result<ArgumentsParseResult, ArgumentsParseError> = Ok(
            ArgumentsParseResult {
                overwritten_config_path: Some("speecher_config.toml".to_string()),
                action: ArgumentsAction::ProceedAsUsual
            }
        );

        assert_eq!(
            parse_arguments(example), expected
        );
    }

    #[test]
    fn parse_wierd_file_path() {
        let example = "--config \"+help\"";
        let expected: Result<ArgumentsParseResult, ArgumentsParseError> = Ok(
            ArgumentsParseResult {
                overwritten_config_path: Some("+help".to_string()),
                action: ArgumentsAction::ProceedAsUsual
            }
        );

        assert_eq!(
            parse_arguments(example), expected
        );
    }

    #[test]
    fn parse_generate() {
        let example = "--config +generate";
        let expected: Result<ArgumentsParseResult, ArgumentsParseError> = Ok(
            ArgumentsParseResult {
                overwritten_config_path: None,
                action: ArgumentsAction::ConfigAction(ConfigAction::Generate)
            }
        );

        assert_eq!(
            parse_arguments(example), expected
        );
    }

    #[test]
    fn parse_generate_at() {
        let example = "--config at_path.toml --config +generate";
        let expected: Result<ArgumentsParseResult, ArgumentsParseError> = Ok(
            ArgumentsParseResult {
                overwritten_config_path: Some("at_path.toml".to_string()),
                action: ArgumentsAction::ConfigAction(ConfigAction::Generate)
            }
        ); 

        assert_eq!(
            parse_arguments(example), expected
        );
    }

    #[test]
    fn parse_verify() {
        let example = "--config +verify";
        let expected: Result<ArgumentsParseResult, ArgumentsParseError> = Ok(
            ArgumentsParseResult {
                overwritten_config_path: None,
                action: ArgumentsAction::ConfigAction(ConfigAction::Verify)
            }
        ); 

        assert_eq!(
            parse_arguments(example), expected
        );
    }

    #[test]
    fn parse_verify_at() {
        let example = "--config +verify --config at_path.toml";
        let expected: Result<ArgumentsParseResult, ArgumentsParseError> = Ok(
            ArgumentsParseResult {
                overwritten_config_path: Some("at_path.toml".to_string()),
                action: ArgumentsAction::ConfigAction(ConfigAction::Verify)
            }
        );

        assert_eq!(
            parse_arguments(example), expected
        );
    }

    #[test]
    fn parse_help1() {
        let example = "--config +help";
        let expected: Result<ArgumentsParseResult, ArgumentsParseError> = Ok(
            ArgumentsParseResult {
                overwritten_config_path: None,
                action: ArgumentsAction::HelpScreen(HelpScreenOptions::Config)
            }
        );

        assert_eq!(
            parse_arguments(example), expected
        );
    }

    #[test]
    fn parse_help2() {
        let example = "--config help";
        let expected: Result<ArgumentsParseResult, ArgumentsParseError> = Ok(
            ArgumentsParseResult {
                overwritten_config_path: None,
                action: ArgumentsAction::HelpScreen(HelpScreenOptions::Config)
            }
        );

        assert_eq!(
            parse_arguments(example), expected
        );
    }

}

