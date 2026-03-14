#[derive(Debug)]
pub struct ParsedArgData {
    pub config_path: std::path::PathBuf,
    pub next_step: PostArgParseStep
}

#[derive(Debug)]
pub enum PostArgParseStep {
    Fail(ArgParseError),
    ShowHelpScreen(HelpScreen),
    GenerateTemplateConfig,
    VerifyConfigFile,
    ProceedAsUsual
}

#[derive(Debug)]
pub enum ArgParseError {
    MalformedInput(std::string::String),
    MissingConfigPath
}

#[derive(Debug)]
pub enum HelpScreen {
    General,
    Config
}

