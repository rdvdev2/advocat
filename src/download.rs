use std::fmt::{Display, Formatter};
use std::path;
use curl::easy;
use std::fs;
use std::io;
use std::io::Write;
use crate::{debug, problem, ux};

pub enum DownloadError {
    CantCreateFile(io::Error),
    CurlError(curl::Error)
}

impl Display for DownloadError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DownloadError::CantCreateFile(e) => write!(f, "Couldn't create the file: {}", e),
            DownloadError::CurlError(e) => write!(f, "Curl raised an error: {}", e)
        }
    }
}

fn download_file(url: &str, path: &path::Path) -> Result<(), DownloadError> {
    debug!("Downloading {} to {}", url, path.to_string_lossy());

    let mut file = fs::File::create(path)
        .map_err(DownloadError::CantCreateFile)?;

    let mut easy = easy::Easy::new();
    easy.url(url).map_err(DownloadError::CurlError)?;
    easy.write_function(move |data| {
        file.write(data).or(Ok(0))
    }).map_err(DownloadError::CurlError)?;
    easy.perform().map_err(DownloadError::CurlError)
}

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

// TODO: Tests
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

// TODO: Tests
pub fn download_problem_zip(problem: &problem::Problem) -> (ux::TaskStatus, Option<DownloadError>) {
    let path = problem.work_dir.join("problem.zip");

    if path.is_file() {
        (ux::TaskStatus::SkipGood, None)
    } else if path.is_dir() || problem.is_private {
        (ux::TaskStatus::SkipBad, None)
    } else {
        match download_file(&problem.zip_url, &path) {
            Ok(()) => (ux::TaskStatus::Done, None),
            Err(e) => (ux::TaskStatus::Fail, Some(e)),
        }
    }
}

// TODO: Tests
pub fn download_problem_main(problem: &problem::Problem) -> (ux::TaskStatus, Option<DownloadError>) {
    let path = problem.work_dir.join("main.cc");

    if problem.has_main || path.is_file() {
        (ux::TaskStatus::SkipGood, None)
    } else if path.is_dir() || problem.is_private {
        (ux::TaskStatus::SkipBad, None)
    } else {
        match download_file(&problem.main_cc_url, &path) {
            Ok(()) => (ux::TaskStatus::Done, None),
            Err(e) => (ux::TaskStatus::Fail, Some(e))
        }
    }
}

// TODO: Tests
pub fn unzip_problem_tests(problem: &problem::Problem) -> (ux::TaskStatus, Option<UnzipError>) {
    let zip_path = problem.work_dir.join("problem.zip");
    let tests_path = problem.work_dir.join("samples");

    if tests_path.is_dir() {
        (ux::TaskStatus::SkipGood, None)
    } else if tests_path.is_file() || !zip_path.exists() {
        (ux::TaskStatus::SkipBad, None)
    } else {
        match unzip_samples(&zip_path, &tests_path) {
            Ok(()) => (ux::TaskStatus::Done, None),
            Err(e) => (ux::TaskStatus::Fail, Some(e))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::env;
    use std::fs;

    #[test]
    fn download_file_works() {
        let file = env::temp_dir().join("advocat-test").join("problem.zip");
        fs::create_dir_all(file.parent().unwrap()).unwrap();
        let url = "https://jutge.org/problems/P10051_en/zip";

        assert!(download_file(url, &file).is_ok());
        assert!(file.is_file());
        // TODO: Check the contents of the file

        fs::remove_file(file).unwrap();
    }

    #[test]
    fn unzip_file_works() {
        let file = env::temp_dir().join("advocat-test").join("problem-2.zip");
        fs::create_dir_all(file.parent().unwrap()).unwrap();
        let url = "https://jutge.org/problems/P10051_en/zip";

        assert!(download_file(url, &file).is_ok(),
            "The test file can't be downloaded, test ABORTED");

        let output_folder = env::temp_dir().join("advocat-test").join("samples");
        assert!(unzip_samples(&file, &output_folder).is_ok());
        assert!(output_folder.is_dir());
        output_folder.read_dir().unwrap().for_each(|x| {
            assert!(x.unwrap().file_name().to_string_lossy().starts_with("sample"));
        });
        assert_ne!(output_folder.read_dir().unwrap().count(), 0);

        fs::remove_dir_all(output_folder).unwrap();
        fs::remove_file(file).unwrap();
    }
}