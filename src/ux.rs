use termion::{color, style};

pub enum LogLevel { Debug, Info, Warning, Error }

pub fn print_message(level: LogLevel, msg: String) {
    match level {
        LogLevel::Debug => eprintln!(":: {}{}{}", style::Italic, msg, style::Reset),
        LogLevel::Info => println!("{}", msg),
        LogLevel::Warning => eprintln!("{}{}WARNING: {}{}{}", color::Fg(color::LightYellow), style::Bold, msg, style::NoBold, style::Reset),
        LogLevel::Error => eprintln!("{}{}ERROR: {}{}{}", color::Fg(color::Red), style::Bold, msg, style::NoBold, style::Reset)
    }
}

#[macro_export]
macro_rules! print_message {
    ($($msg:tt)*) => ($crate::ux::print_message($crate::ux::LogLevel::$level, format!($($msg)*)))
}

#[macro_export]
macro_rules! debug {
    ($($msg:tt)*) => ($crate::ux::print_message($crate::ux::LogLevel::Debug, format!($($msg)*)))
}

#[macro_export]
macro_rules! info {
    ($($msg:tt)*) => ($crate::ux::print_message($crate::ux::LogLevel::Info, format!($($msg)*)))
}

#[macro_export]
macro_rules! warning {
    ($($msg:tt)*) => ($crate::ux::print_message($crate::ux::LogLevel::Warning, format!($($msg)*)))
}

#[macro_export]
macro_rules! error {
    ($($msg:tt)*) => ($crate::ux::print_message($crate::ux::LogLevel::Error, format!($($msg)*)))
}