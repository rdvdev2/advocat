use std::env;
use crate::problem::{CreationError, Problem};

mod problem;
mod ux;

pub fn run() -> i32 {
    let name = env!("CARGO_PKG_NAME");
    let version = env!("CARGO_PKG_VERSION");
    let authors = env!("CARGO_PKG_AUTHORS");

    info!("{} v{} by {}", name, version, authors);
    debug!("Debug mode ON: Consider decreasing the log level to get quiter output.");

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

    warning!("After detecting the problem, nothing happens! (unimplemented)");
    exitcode::OK
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