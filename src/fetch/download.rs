use crate::fetch::{connection_manager, unzip};
use crate::{debug, problem, ux};

pub fn download_problem_zip(
    problem: &problem::Problem,
    connection: &mut connection_manager::ConnectionManager,
) -> (ux::TaskStatus, Option<connection_manager::Error>) {
    let path = problem.work_dir.join("problem.zip");

    if path.is_file() {
        debug!("Problem zip already downloaded");
        (ux::TaskStatus::SkipGood, None)
    } else if path.is_dir() {
        debug!("The download path is a folder");
        (ux::TaskStatus::SkipBad, None)
    } else {
        match connection.get_file(&problem.zip_url, &path) {
            Ok(()) => (ux::TaskStatus::Done, None),
            Err(e) => (ux::TaskStatus::Fail, Some(e)),
        }
    }
}

pub fn download_problem_main(
    problem: &problem::Problem,
    connection: &mut connection_manager::ConnectionManager,
) -> (ux::TaskStatus, Option<connection_manager::Error>) {
    let path = problem.work_dir.join("main.cc");

    if problem.has_main || path.is_file() {
        debug!("Problem main.cc already downloaded or unnecessary");
        (ux::TaskStatus::SkipGood, None)
    } else if path.is_dir() {
        debug!("The download path is a folder");
        (ux::TaskStatus::SkipBad, None)
    } else {
        match connection.get_file(&problem.main_cc_url, &path) {
            Ok(()) => (ux::TaskStatus::Done, None),
            Err(e) => (ux::TaskStatus::Fail, Some(e)),
        }
    }
}

pub fn unzip_problem_tests(problem: &problem::Problem) -> (ux::TaskStatus, Option<unzip::Error>) {
    let zip_path = problem.work_dir.join("problem.zip");
    let tests_path = problem.work_dir.join("samples");

    if tests_path.is_dir() {
        debug!("Problem tests already extracted");
        (ux::TaskStatus::SkipGood, None)
    } else if tests_path.is_file() || !zip_path.exists() {
        debug!("Unable to extract problem tests");
        (ux::TaskStatus::SkipBad, None)
    } else {
        match unzip::unzip_samples(&zip_path, &tests_path) {
            Ok(()) => (ux::TaskStatus::Done, None),
            Err(e) => (ux::TaskStatus::Fail, Some(e)),
        }
    }
}
