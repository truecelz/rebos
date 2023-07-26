#![allow(dead_code)]

use colored::{Colorize, ColoredString};

pub enum LogMode {
    Info,
    Error,
    Warn,
    Debug,
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => ({
        log_core_print(format!($($arg)*), LogMode::Info);
    });
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => ({
        log_core_print(format!($($arg)*), LogMode::Error);
    });
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => ({
        log_core_print(format!($($arg)*), LogMode::Warn);
    });
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => ({
        log_core_print(format!($($arg)*), LogMode::Debug);
    });
}

pub fn log_core_print(msg: String, mode: LogMode) {
    let prefix_text: &str = match mode {
        LogMode::Info => "Info",
        LogMode::Error => "Error",
        LogMode::Warn => "Warning",
        LogMode::Debug => "Debug",
    };

    let prefix = apply_color(prefix_text.to_string(), &mode);

    println!("{left}{}{right} {}", prefix.bold(), msg, left = "[".black().bold(), right = "] :".black().bold());
}

fn apply_color(string: String, mode: &LogMode) -> String {
    let colored_string: ColoredString = match mode {
        LogMode::Info => string.bright_green(),
        LogMode::Error => string.bright_red(),
        LogMode::Warn => string.bright_yellow(),
        LogMode::Debug => string.bright_magenta(),
    };

    return colored_string.to_string();
}