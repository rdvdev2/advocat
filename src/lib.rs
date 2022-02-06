use std::{env, fs, path};
use std::fmt::Display;
use termion::{color, style};
use crate::compiler::CompilationError;
use crate::problem::{CreationError, Problem};
use crate::testsuite::TestSuite;
use crate::ux::set_global_log_level;

mod problem;
mod ux;
mod download;
mod testsuite;
mod template;
mod compiler;
mod config;
mod connection_manager;

#[cfg(test)]
mod test_utils;

pub fn run() -> i32 {
    let config = config::Config::generate();
    set_global_log_level(config.log_level);

    let name = env!("CARGO_PKG_NAME");
    let version = env!("CARGO_PKG_VERSION");
    let authors = env!("CARGO_PKG_AUTHORS");

    info!("{} v{} by {}", name, version, authors);
    debug!("Debug mode ON: Consider decreasing the log level to get quieter output.");

    debug!("Getting the working directory...");
    let cwd = if let Ok(dir) = env::current_dir() {
        dir
    } else {
        error!("Couldn't get the working directory!");
        return exitcode::OSERR;
    };
    debug!("Done! Working directory is {}", cwd.to_string_lossy());

    debug!("Generating problem details...");
    let problem: Problem = match Problem::new(cwd.as_path(), &config) {
        Ok(p) => p,
        Err(e) => {
            return handle_problem_creation_error(e)
        }
    };
    debug!("Done! Problem details: {:?}", problem);

    if !problem.work_dir.exists() {
        debug!("Creating the problem directory: {}", problem.work_dir.to_string_lossy());
        if fs::create_dir_all(&problem.work_dir).is_err() {
            error!("Couldn't create a workdir for the program!");
            return exitcode::IOERR;
        }
    }

    if problem.work_dir.is_file() {
        error!("The file {} has the same path as the workdir of the program, move it or delete it!",
            problem.work_dir.to_string_lossy());
        return exitcode::DATAERR;
    }

    let mut connection = match connection_manager::ConnectionManager::new(&config) {
        Ok(c) => c,
        Err(e) => {
            error!("Couldn't start the connection manager: {}", e);
            return exitcode::IOERR;
        }
    };

    let zip = execute_task("Downloading problem zip", || download::download_problem_zip(&problem, &mut connection));
    let main_cc = execute_task("Downloading problem main.cc", || download::download_problem_main(&problem, &mut connection));
    let tests = execute_task("Extracting tests", || download::unzip_problem_tests(&problem));

    if !zip {
        warning!("Unable to retrieve tests!");
    }

    if !main_cc {
        error!("Unable to retrieve the main.cc file!");
        return exitcode::IOERR;
    }

    if !tests {
        warning!("Unable to unzip tests!");
    }

    let jutge_tests = load_tests("jutge.org", problem.work_dir.join("samples").as_path(), !tests);
    let user_tests = load_tests("user", problem.source.parent().unwrap(), false);

    debug!("Generating sources...");
    let generated_sources = match template::generate_main(&problem) {
        Ok(s) => s,
        Err(e) => {
            error!("Couldn't generate a main.cc file to compile: {}", e);
            return exitcode::IOERR;
        }
    };

    println!();
    let binary = execute_compiler(&problem, generated_sources.as_path());

    let mut passed_tests: usize = 0;
    let mut total_tests: usize = 0;
    if let Some(tests) = jutge_tests {
        passed_tests += tests.run(problem.output.as_path(), !binary);
        total_tests += tests.count();
    }
    if let Some(tests) = user_tests {
        passed_tests += tests.run(problem.output.as_path(), !binary);
        total_tests += tests.count();
    }

    show_veredict(binary, passed_tests, total_tests)
}

fn execute_task<T, E: Display + Sized>(name: &str, mut task: T) -> bool
where
    T: FnMut() -> (ux::TaskStatus, Option<E>)
{
    ux::show_task_status(name, ux::TaskType::Fetch, &ux::TaskStatus::InProgress);
    let (status, err) = task();

    ux::show_task_status(name, ux::TaskType::Fetch, &status);
    if let Some(err) = err {
        error!("The task [{}] returned the following error: {}", name, err);
    }
    status.is_ok()
}

fn load_tests(name: &str, dir: &path::Path, ignore_missing_dir: bool) -> Option<TestSuite> {
    debug!("Loading {} tests...", name);
    match TestSuite::from_dir(name, dir) {
        Err(testsuite::TestSuiteCreationError::PathDoesntExist) if ignore_missing_dir => None,
        Err(e) => { error!("Error loading {} tests: {}", name, e); None },
        Ok(testsuite) => Some(testsuite)
    }
}

fn execute_compiler(problem: &Problem, generated_sources: &path::Path) -> bool {
    const TASK: &str = "Compilation";

    ux::show_task_status(TASK, ux::TaskType::Test, &ux::TaskStatus::InProgress);
    match compiler::P1XX.compile_problem(problem, generated_sources) {
        Ok(()) => {
            ux::show_task_status(TASK, ux::TaskType::Test, &ux::TaskStatus::Pass);
            true
        },
        Err(e) => {
            ux::show_task_status(TASK, ux::TaskType::Test, &ux::TaskStatus::Fail);
            match e.error {
                CompilationError::CompilerError(stderr) => {
                    ux::show_task_output(format!("Compilation output (pass {})", e.pass).as_str(), &stderr);
                }
                _ => error!("Compilation failed unexpectedly: {}", e)
            }
            false
        }
    }
}

fn handle_problem_creation_error(e: problem::CreationError) -> i32 {
    error!("{}", e);
    match e {
        CreationError::NonExistingPath |
        CreationError::NonDirectoryPath |
        CreationError::BadPathFormat => exitcode::OSERR,
        CreationError::BadId(_) |
        CreationError::BadSource(_) => exitcode::DATAERR
    }
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
    println!(" ({passed} out of {total} tests passed){}", style::Reset);

    code
}

#[cfg(test)]
mod test {
    use crate::problem::{IdError, SourceError};
    use super::*;

    #[test]
    fn handle_problem_creation_error_test() {
        assert_eq!(handle_problem_creation_error(CreationError::NonExistingPath), exitcode::OSERR);
        assert_eq!(handle_problem_creation_error(CreationError::NonDirectoryPath), exitcode::OSERR);
        assert_eq!(handle_problem_creation_error(CreationError::BadPathFormat), exitcode::OSERR);
        assert_eq!(handle_problem_creation_error(CreationError::BadId(IdError::InvalidId)), exitcode::DATAERR);
        assert_eq!(handle_problem_creation_error(CreationError::BadSource(SourceError::NonFilePath)), exitcode::DATAERR);
    }

    #[test]
    fn show_veredict_test() {
        assert_eq!(show_veredict(false, 0, 0), exitcode::DATAERR);
        assert_eq!(show_veredict(true, 0, 0), exitcode::OK);
        assert_eq!(show_veredict(true, 0, 1), exitcode::DATAERR);
        assert_eq!(show_veredict(true, 1, 1), exitcode::OK);
    }
}