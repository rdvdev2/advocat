extern crate core;

use std::env;
use crate::problem::{CreationError, Problem};

mod problem;

pub fn run() -> i32 {
    let cwd = if let Ok(dir) = env::current_dir() {
        dir
    } else {
        eprintln!("FATAL ERROR! Couldn't get the working directory!");
        return exitcode::OSERR;
    };

    let _problem: Problem = match Problem::try_from(cwd.as_path()) {
        Ok(p) => p,
        Err(e) => {
            return handle_problem_creation_error(e)
        }
    };

    exitcode::OK
}

fn handle_problem_creation_error(e: problem::CreationError) -> i32 {
    eprintln!("ERROR! {}", e);
    match e {
        CreationError::NonExistingPath |
        CreationError::NonDirectoryPath |
        CreationError::BadPathFormat => exitcode::OSERR,
        CreationError::BadId(_) |
        CreationError::BadSource(_) => exitcode::DATAERR
    }
}