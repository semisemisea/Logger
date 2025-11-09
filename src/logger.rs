#![allow(unused)]

const ESC: char = '\x1b';
const RESET: &str = "\x1b[0m";
const RED_FG: &str = "\x1b[31m";
const GREEN_FG: &str = "\x1b[32m";
const YELLOW_FG: &str = "\x1b[33m";
const BLUE_FG: &str = "\x1b[34m";
const PURPLE_FG: &str = "\x1b[35m";
const CYAN_FG: &str = "\x1b[36m";
const WHITE_FG: &str = "\x1b[37m";
const RED_BG: &str = "\x1b[41m";
const GREEN_BG: &str = "\x1b[42m";
const YELLOW_BG: &str = "\x1b[43m";
const BLUE_BG: &str = "\x1b[44m";
const PURPLE_BG: &str = "\x1b[45m";

use std::{default, fmt::Debug};

#[derive(Debug, PartialEq, PartialOrd)]
enum LogLevel {
    Debug = 0,
    Info,
    Warning,
    Error,
    Panic,
}

pub struct LoggerConfig {
    file_name: bool,
    lines: bool,
    log_level: LogLevel,
}

impl LoggerConfig {
    pub fn build(self) -> Logger {
        Logger::from_config(self)
    }

    pub fn enable_file_name(mut self, file_name: bool) -> Self {
        self.file_name = file_name;
        self
    }

    pub fn enable_lines(mut self, lines: bool) -> Self {
        self.lines = lines;
        self
    }
    pub fn set_log_level(mut self, level: u8) -> Self {
        self.log_level = match level {
            0 => LogLevel::Debug,
            1 => LogLevel::Info,
            2 => LogLevel::Warning,
            3 => LogLevel::Error,
            4 => LogLevel::Panic,
            _ => LogLevel::Warning,
        };
        self
    }
}

impl Default for LoggerConfig {
    fn default() -> Self {
        LoggerConfig {
            file_name: true,
            lines: true,
            log_level: LogLevel::Warning,
        }
    }
}

#[derive(Default)]
pub struct Logger(LoggerConfig);

impl Logger {
    fn from_config(config: LoggerConfig) -> Self {
        Logger(config)
    }

    pub fn config() -> LoggerConfig {
        LoggerConfig::default()
    }

    pub fn enable_file_name(&mut self, file_name: bool) {
        self.0.file_name = file_name
    }

    pub fn debug(&self, msg: &str) {
        if self.0.log_level <= LogLevel::Debug {
            eprint!("{PURPLE_FG}DEBUG: ");
            if self.0.file_name {
                eprint!("{BLUE_FG}[{} ", file!());
            }
            if self.0.lines {
                eprint!("{BLUE_FG}lines: {}] ", line!());
            }
            eprintln!("{RESET}{msg}");
        }
    }

    pub fn info(&self, msg: &str) {
        if self.0.log_level <= LogLevel::Info {
            eprint!("{CYAN_FG}LOG: ");
            if self.0.file_name {
                eprint!("{BLUE_FG}[{} ", file!());
            }
            if self.0.lines {
                eprint!("{BLUE_FG}lines: {}] ", line!());
            }
            eprintln!("{RESET}{msg}");
        }
    }

    pub fn warning(&self, msg: &str) {
        if self.0.log_level <= LogLevel::Warning {
            eprint!("{YELLOW_FG}WARNING: ");
            if self.0.file_name {
                eprint!("{BLUE_FG}[{} ", file!());
            }
            if self.0.lines {
                eprint!("{BLUE_FG}lines: {}] ", line!());
            }
            eprintln!("{RESET}{msg}");
        }
    }

    pub fn error(&self, msg: &str) {
        if self.0.log_level <= LogLevel::Error {
            eprint!("{RED_FG}ERROR: ");
            if self.0.file_name {
                eprint!("{BLUE_FG}[{} ", file!());
            }
            if self.0.lines {
                eprint!("{BLUE_FG}lines: {}] ", line!());
            }
            eprintln!("{RESET}{msg}");
        }
    }

    pub fn panic(&self, msg: &str) {
        if self.0.log_level <= LogLevel::Panic {
            eprint!("{RED_FG}{RESET}{RED_BG}PANIC{RESET}: ");
            if self.0.file_name {
                eprint!("{BLUE_FG}[{} ", file!());
            }
            if self.0.lines {
                eprint!("{BLUE_FG}lines: {}] ", line!());
            }
            eprintln!("{RESET}{msg}");
        }
    }

    pub fn enable_lines(&mut self, lines: bool) {
        self.0.lines = lines
    }

    pub fn level(&mut self, level: u8) {
        self.0.log_level = match level {
            0 => LogLevel::Debug,
            1 => LogLevel::Info,
            2 => LogLevel::Warning,
            3 => LogLevel::Error,
            4 => LogLevel::Panic,
            _ => LogLevel::Warning,
        }
    }
}

pub trait CaptureExt {
    fn capture(self, logger: Logger) -> Self;
}

impl<T, E> CaptureExt for ::std::result::Result<T, E>
where
    E: ::std::fmt::Display + ::std::fmt::Debug,
{
    #[inline]
    fn capture(self, logger: Logger) -> Result<T, E> {
        self.inspect_err(|e| logger.info(format!("{e}").as_str()))
    }
}
