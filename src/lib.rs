use std::{env, fmt, ops, path};
use termion::{color, style};
use crate::problem::Problem;

mod problem;
pub mod ux;
mod testing;
mod compilation;
mod fetch;
mod config;

#[cfg(test)]
mod test_utils;

pub struct Error {
    description: String,
    exitcode: exitcode::ExitCode
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.description)
    }
}

impl ops::Deref for Error {
    type Target = exitcode::ExitCode;

    fn deref(&self) -> &Self::Target {
        &self.exitcode
    }
}

pub fn run() -> Result<exitcode::ExitCode, Error> {
    let config = config::Config::generate()?;
    ux::set_global_log_level(config.log_level);

    info!("{} v{} by {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"), env!("CARGO_PKG_AUTHORS"));
    debug!("Debug mode ON: Consider decreasing the log level to get quieter output.");

    debug!("Generating problem details...");
    let problem = Problem::new(&config)?;
    debug!("Done! Problem details: {:?}", problem);

    let (_zip, _main_cc, tests) = fetch::fetch_resources(&problem, &config)?;
    
    let tests = [
        load_tests("jutge.org", problem.work_dir.join("samples").as_path(), !tests),
        load_tests("user", problem.source.parent().unwrap(), false)
    ];

    debug!("Generating sources...");
    let generated_sources = compilation::generate_main(&problem)?;

    println!();
    let binary = execute_compiler(&problem, generated_sources.as_path());
    let (passed_tests, total_tests) = run_tests(&tests, problem.output.as_path(), !binary);

    Ok(show_veredict(binary, passed_tests, total_tests))
}

fn load_tests(name: &str, dir: &path::Path, ignore_missing_dir: bool) -> Option<testing::TestSuite> {
    debug!("Loading {} tests...", name);
    match testing::TestSuite::from_dir(name, dir) {
        Err(testing::Error::PathDoesntExist) if ignore_missing_dir => None,
        Err(e) => { error!("Error loading {} tests: {}", name, e); None },
        Ok(testsuite) => Some(testsuite)
    }
}

fn execute_compiler(problem: &Problem, generated_sources: &path::Path) -> bool {
    const TASK: &str = "Compilation";

    ux::show_task_status(TASK, ux::TaskType::Test, &ux::TaskStatus::InProgress);
    match compilation::P1XX.compile_problem(problem, generated_sources) {
        Ok(()) => {
            ux::show_task_status(TASK, ux::TaskType::Test, &ux::TaskStatus::Pass);
            true
        },
        Err(e) => {
            ux::show_task_status(TASK, ux::TaskType::Test, &ux::TaskStatus::Fail);
            match e.error {
                compilation::Error::CompilerError(stderr) => {
                    ux::show_task_output(format!("Compilation output (pass {})", e.pass).as_str(), &stderr);
                }
                _ => error!("Compilation failed unexpectedly: {}", e)
            }
            false
        }
    }
}

fn run_tests(testsuites: &[Option<testing::TestSuite>], binary: &path::Path, skip_tests: bool) -> (usize, usize) {
    let mut passed: usize = 0;
    let mut total: usize = 0;

    for testsuite in testsuites.iter().flatten() {
        passed += testsuite.run(binary, skip_tests);
        total += testsuite.count();
    }

    (passed, total)
}

fn show_veredict(compiles: bool, passed: usize, total: usize) -> i32 {
    let code = if !compiles {
        print!("{}Your code doesn't compile!", color::Fg(color::Red));
        exitcode::DATAERR
    } else if total == 0 {
        print!("{}Your code compiles but you should test it before sumbitting. Try to add some tests to the folder.", color::Fg(color::LightYellow));
        exitcode::OK
    } else if passed != total {
        print!("{}DON'T submit your code to jutge.org!", color::Fg(color::Red));
        exitcode::DATAERR
    } else {
        print!("{}You're ready to submit your code to jutge.org!", color::Fg(color::Green));
        exitcode::OK
    };
    println!(" ({} out of {} tests passed){}", passed, total, style::Reset);

    code
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn show_veredict_test() {
        assert_eq!(show_veredict(false, 0, 0), exitcode::DATAERR);
        assert_eq!(show_veredict(true, 0, 0), exitcode::OK);
        assert_eq!(show_veredict(true, 0, 1), exitcode::DATAERR);
        assert_eq!(show_veredict(true, 1, 1), exitcode::OK);
    }
}
