use std::{env, fs, path};
use std::fmt::Display;
use crate::problem::{CreationError, Problem};
use crate::testsuite::TestSuite;
use crate::ux::set_global_log_level;

mod problem;
mod ux;
mod download;
mod testsuite;

pub struct Config {
    log_level: ux::LogLevel
}

pub fn parse_args() -> Config {
    let mut args = env::args();
    let mut config = Config {
        log_level: ux::LogLevel::Info
    };

    let _executable = args.next();
    for arg in args {
        match arg.as_str() {
            "-d" | "--debug" => config.log_level = ux::LogLevel::Debug,
            "-h" | "--help" => todo!("Add a help message"),
            _ => {}
        }
    }

    config
}

pub fn run(config: Config) -> i32 {
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
    let problem: Problem = match Problem::try_from(cwd.as_path()) {
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

    let zip = execute_task("Downloading problem zip", || download::download_problem_zip(&problem));
    let main_cc = execute_task("Downloading problem main.cc", || download::download_problem_main(&problem));
    let tests = execute_task("Extracting tests", || download::unzip_problem_tests(&problem));

    if !zip && problem.is_private {
        eprintln!();
        warning!("Unable to retrieve tests!");
        eprintln!("You can manually download the problem zip from [{}] and save it as [{}]",
                  problem.zip_url,
                  problem.work_dir.join("problem.zip").to_string_lossy()
        );
    }

    if !main_cc {
        eprintln!();
        error!("Unable to retrieve the main.cc file!");
        eprintln!("You can manually download the main.cc file from [{}] and save it as [{}]",
            problem.main_cc_url,
            problem.work_dir.join("main.cc").to_string_lossy()
        );
        return exitcode::IOERR;
    }

    if !tests {
        eprintln!();
        warning!("Unable to unzip tests!");
    }

    let jutge_tests = load_tests("jutge.org", problem.work_dir.join("samples").as_path(), !tests);
    let user_tests = load_tests("user", problem.source.parent().unwrap(), false);

    exitcode::OK
}

fn execute_task<T, E: Display + Sized>(name: &str, task: T) -> bool
where
    T: Fn() -> (ux::TaskStatus, Option<E>)
{
    ux::show_task_status(name, ux::TaskType::Fetch, &ux::TaskStatus::InProgress);
    let (status, err) = task();

    ux::show_task_status(name, ux::TaskType::Fetch, &status);
    if let Some(err) = err {
        error!("The task [{}] returned the following error: {}", name, err);
    }
    return status.is_ok()
}

fn load_tests(name: &str, dir: &path::Path, ignore_missing_dir: bool) -> Option<TestSuite> {
    debug!("Loading {} tests...", name);
    match TestSuite::FromDir(name, dir) {
        Err(testsuite::TestSuiteCreationError::PathDoesntExist) if ignore_missing_dir => None,
        Err(e) => { error!("Error loading {} tests: {}", name, e); None },
        Ok(testsuite) => Some(testsuite)
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