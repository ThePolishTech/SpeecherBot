
#[derive(Debug)]
pub enum LogColours {
    Error,
    Warn,
    Success,
    Info,

    None
}
impl Into<colored::CustomColor> for LogColours {
    fn into(self) -> colored::CustomColor {
        
        let (r, g, b) = match self {
            Self::Error   => (255,   0,   0),
            Self::Warn    => (255, 255,   0),
            Self::Success => (  0, 255, 127),
            Self::Info    => (  0, 255, 255),

            Self::None    => (255, 255, 255)
        };

        colored::CustomColor { r, g, b }
    }
}


