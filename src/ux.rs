use std::io;
use std::io::Write;
use termion::{color, style};

#[derive(PartialEq, PartialOrd, Clone, Copy)]
pub enum LogLevel { Debug, Info, Warning, Error }

static mut GLOBAL_LOG_LEVEL: LogLevel = LogLevel::Info;

pub fn set_global_log_level(level: LogLevel) {
    unsafe {
        GLOBAL_LOG_LEVEL = level
    }
}

pub fn get_global_log_level() -> LogLevel {
    unsafe {
        GLOBAL_LOG_LEVEL
    }
}

pub fn print_message(level: LogLevel, msg: String) {
    match level {
        x if x < get_global_log_level() => (),
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

pub enum TaskType { Fetch, Test }

#[derive(PartialEq, Debug)]
pub enum TaskStatus { Done, Pass, SkipGood, SkipBad, Fail, InProgress }

impl TaskStatus {
    pub fn is_ok(&self) -> bool {
        matches!(self, TaskStatus::Done | TaskStatus::Pass | TaskStatus::SkipGood)
    }
}

pub fn show_task_status(name: &str, task_type: TaskType, task_status: &TaskStatus) {
    let name = match task_type {
        TaskType::Fetch => name.to_owned() + "... ",
        TaskType::Test => name.to_uppercase() + ": "
    };
    print!("{}{}", color::Fg(color::Yellow), name);

    match task_status {
        TaskStatus::Done => println!("{}DONE ✓{}", color::Fg(color::Green), style::Reset),
        TaskStatus::Pass => println!("{}PASS ✓{}", color::Fg(color::Green), style::Reset),
        TaskStatus::SkipGood => println!("{}SKIP ✓{}", color::Fg(color::Cyan), style::Reset),
        TaskStatus::SkipBad => println!("{}SKIP ✘{}", color::Fg(color::Cyan), style::Reset),
        TaskStatus::Fail => println!("{}FAIL ✘{}", color::Fg(color::Red), style::Reset),
        TaskStatus::InProgress => {
            print!("{}...\r", style::Reset);
            if let LogLevel::Debug = get_global_log_level() {
                println!();
            } else {
                io::stdout().flush().unwrap();
            }
        }
    }
}

pub fn show_task_output(title: &str, contents: &str) {
    println!("==> {}:", title);
    println!("{}{}{}", color::Fg(color::Magenta), contents, style::Reset);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[ignore] // Rationale: This test is purely visual and must be manually checked
    fn fetch_status_test() {
        let task_name = "Testing fetch status";

        show_task_status(task_name, TaskType::Fetch, &TaskStatus::Done);
        show_task_status(task_name, TaskType::Fetch, &TaskStatus::SkipGood);
        show_task_status(task_name, TaskType::Fetch, &TaskStatus::SkipBad);
        show_task_status(task_name, TaskType::Fetch, &TaskStatus::Fail);
        show_task_status(task_name, TaskType::Fetch, &TaskStatus::InProgress);
        println!();
    }

    #[test]
    #[ignore] // Rationale: This test is purely visual and must be manually checked
    fn testing_status_test() {
        let task_name = "dummy test";

        show_task_status(task_name, TaskType::Test, &TaskStatus::Pass);
        show_task_status(task_name, TaskType::Test, &TaskStatus::SkipBad);
        show_task_status(task_name, TaskType::Test, &TaskStatus::Fail);
        show_task_status(task_name, TaskType::Test, &TaskStatus::InProgress);
        println!();
    }
}