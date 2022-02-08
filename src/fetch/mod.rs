use std::fmt;
use crate::{config, error, problem, ux, warning};

mod download;
mod connection_manager;
mod credentials;
mod unzip;

pub use credentials::Credentials as Credentials;

pub fn fetch_resources(problem: &problem::Problem, config: &config::Config) -> Result<(bool, bool, bool), crate::Error> {
    let mut connection = connection_manager::ConnectionManager::new(config)
        .map_err(|e| crate::Error {
            description: format!("Couldn't start the connection manager: {}", e),
            exitcode: exitcode::IOERR
        })?;

    let zip = execute_task("Downloading problem zip", || download::download_problem_zip(problem, &mut connection));
    let main_cc = execute_task("Downloading problem main.cc", || download::download_problem_main(problem, &mut connection));
    let tests = execute_task("Extracting tests", || download::unzip_problem_tests(problem));

    if !zip {
        warning!("Unable to retrieve tests!");
    }

    if !main_cc {
        return Err( crate::Error {
            description: String::from("Unable to retrieve the main.cc file, which is required to compile your binary!"),
            exitcode: exitcode::IOERR
        });
    }

    if !tests {
        warning!("Unable to unzip tests!");
    }

    Ok((zip, main_cc, tests))
}

fn execute_task<T, E: fmt::Display + Sized>(name: &str, mut task: T) -> bool
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