pub mod log_colours;
pub use log_colours::LogColours;


use colored::Colorize;



#[derive(Debug)]
pub enum TimestampTimezoneType {
    Local,
    Utc,
}


#[derive(Debug)]
pub struct Logger {
    timestap_timezone_type: TimestampTimezoneType
}
impl Default for Logger {
    fn default() -> Self {
        Logger::new()
    }
}

impl Logger {
    pub fn new() -> Logger {
        Logger {
            timestap_timezone_type: TimestampTimezoneType::Local
        }
    }
    pub fn set_timezone_type(
        mut self,
        timezone_type: TimestampTimezoneType
    ) -> Self {
        self.timestap_timezone_type = timezone_type;
        self
    }

    pub fn log(&self, with_level: LogColours, message: impl std::fmt::Display) {
        let current_timestamp = time_format::now_ms()
            .ok()
            .unwrap_or(time_format::TimeStampMs {
                seconds: -1,
                milliseconds: 0
            });

        let formated_timestamp = match self.timestap_timezone_type {
            TimestampTimezoneType::Local => time_format::strftime_ms_local(
                "%Y-%m-%d %H:%M:%S.{ms}",
                current_timestamp
            ),
            TimestampTimezoneType::Utc => time_format::strftime_ms_utc(
                "%Y-%m-%d %H:%M:%S.{ms} %Z",
                current_timestamp
            ),
        }.unwrap_or(String::from("TIMESTAMP ERROR"));

        let formated_level = match with_level {
            LogColours::Error   => "  ERROR".custom_color(LogColours::Error),
            LogColours::Warn    => "   WARN".custom_color(LogColours::Warn),
            LogColours::Success => "SUCCESS".custom_color(LogColours::Success),
            LogColours::Info    => "   INFO".custom_color(LogColours::Info),
            LogColours::None    => "   NONE".custom_color(LogColours::None),
        };

        println!("[{formated_timestamp}] {formated_level} => {message}");
    }
} 

