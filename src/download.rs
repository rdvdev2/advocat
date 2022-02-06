use std::fmt::Display;
use std::fmt::Formatter;
use std::path;
use std::fs;
use std::io;
use crate::{connection_manager, debug, problem, ux};

pub enum UnzipError {
    CantReadFile(io::Error),
    CantCreateFile(io::Error),
    CantInflateFile(io::Error),
    ZipError(zip::result::ZipError)
}

impl Display for UnzipError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            UnzipError::CantReadFile(e) => write!(f, "Couldn't read the file: {}", e),
            UnzipError::CantCreateFile(e) => write!(f, "Couldn't create a file: {}", e),
            UnzipError::CantInflateFile(e) => write!(f, "Couldn't inflate a file: {}", e),
            UnzipError::ZipError(e) => write!(f, "Zip raised an error: {:?}", e)
        }
    }
}

fn unzip_samples(zip_path: &path::Path, output_folder: &path::Path) -> Result<(), UnzipError> {
    debug!("Unzipping {} to {}", zip_path.to_string_lossy(), output_folder.to_string_lossy());

    let zip_file = fs::File::open(zip_path)
        .map_err(UnzipError::CantReadFile)?;
    let mut archive = zip::ZipArchive::new(zip_file)
        .map_err(UnzipError::ZipError)?;

    debug!("Creating output folder {}", output_folder.to_string_lossy());
    fs::create_dir_all(output_folder).map_err(UnzipError::CantCreateFile)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let outpath = match filter_samples(&file) {
            Some(path) => output_folder.join(path),
            None => continue
        };

        if file.is_dir() {
            debug!("Inflating folder {}", output_folder.to_string_lossy());
            fs::create_dir_all(outpath).map_err(UnzipError::CantCreateFile)?;
        } else {
            if let Some(parent) = outpath.parent() {
                if !parent.exists() {
                    debug!("Inflating folder {}", parent.to_string_lossy());
                    fs::create_dir_all(parent).map_err(UnzipError::CantCreateFile)?;
                }
            }

            debug!("Inflating file {}", outpath.to_string_lossy());
            let mut outfile = fs::File::create(&outpath).map_err(UnzipError::CantCreateFile)?;
            io::copy(&mut file, &mut outfile).map_err(UnzipError::CantInflateFile)?;
        }
    }
    Ok(())
}

fn filter_samples(zip_file: &zip::read::ZipFile) -> Option<path::PathBuf> {
    let path = zip_file.enclosed_name()?;
    if let Some(filename) = path.file_name() {
        if zip_file.is_file() && filename.to_string_lossy().starts_with("sample") {
            Some(filename.into())
        } else {
            None
        }
    } else {
        None
    }
}

pub fn download_problem_zip(problem: &problem::Problem, connection: &mut connection_manager::ConnectionManager) -> (ux::TaskStatus, Option<connection_manager::Error>) {
    let path = problem.work_dir.join("problem.zip");

    if path.is_file() {
        debug!("Problem zip already downloaded");
        (ux::TaskStatus::SkipGood, None)
    } else if path.is_dir() || problem.is_private {
        debug!("Can't download problem zip");
        (ux::TaskStatus::SkipBad, None)
    } else {
        match connection.get_file(&problem.zip_url, &path) {
            Ok(()) => (ux::TaskStatus::Done, None),
            Err(e) => (ux::TaskStatus::Fail, Some(e)),
        }
    }
}

pub fn download_problem_main(problem: &problem::Problem, connection: &mut connection_manager::ConnectionManager) -> (ux::TaskStatus, Option<connection_manager::Error>) {
    let path = problem.work_dir.join("main.cc");

    if problem.has_main || path.is_file() {
        debug!("Problem main.cc already downloaded or unnecessary");
        (ux::TaskStatus::SkipGood, None)
    } else if path.is_dir() || problem.is_private {
        debug!("Can't download problem main.cc");
        (ux::TaskStatus::SkipBad, None)
    } else {
        match connection.get_file(&problem.main_cc_url, &path) {
            Ok(()) => (ux::TaskStatus::Done, None),
            Err(e) => (ux::TaskStatus::Fail, Some(e))
        }
    }
}

pub fn unzip_problem_tests(problem: &problem::Problem) -> (ux::TaskStatus, Option<UnzipError>) {
    let zip_path = problem.work_dir.join("problem.zip");
    let tests_path = problem.work_dir.join("samples");

    if tests_path.is_dir() {
        debug!("Problem tests already extracted");
        (ux::TaskStatus::SkipGood, None)
    } else if tests_path.is_file() || !zip_path.exists() {
        debug!("Unable to extract problem tests");
        (ux::TaskStatus::SkipBad, None)
    } else {
        match unzip_samples(&zip_path, &tests_path) {
            Ok(()) => (ux::TaskStatus::Done, None),
            Err(e) => (ux::TaskStatus::Fail, Some(e))
        }
    }
}