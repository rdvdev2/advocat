use core::fmt;
use std::{env, path};
use std::fs;
use std::io;

use regex::Regex;
use crate::debug;

#[derive(Debug)]
pub struct Problem {
    pub id: String,
    pub source: path::PathBuf,
    pub output: path::PathBuf,
    pub work_dir: path::PathBuf,
    pub tmp_dir: path::PathBuf,
    pub is_private: bool,
    pub has_main: bool,
    pub zip_url: String,
    pub main_cc_url: String
}

#[derive(Debug)]
pub enum CreationError {
    NonExistingPath,
    NonDirectoryPath,
    BadPathFormat,
    BadId(IdError),
    BadSource(SourceError)
}

impl fmt::Display for CreationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CreationError::NonExistingPath => write!(f, "The path doesn't exist!"),
            CreationError::NonDirectoryPath => write!(f, "The path isn't a directory!"),
            CreationError::BadPathFormat => write!(f, "The path ends in \"..\"!"),
            CreationError::BadId(e) => write!(f, "Problem id is wrong: {}", e),
            CreationError::BadSource(e) => write!(f, "Problem with main.cc: {}", e)
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum IdError {
    InvalidId,
    UnsupportedType(char)
}

impl fmt::Display for IdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IdError::InvalidId => write!(f, "The problem id is invalid!"),
            IdError::UnsupportedType(_) => write!(f, "The problem type is unsupported!")
        }
    }
}

#[derive(Debug)]
pub enum SourceError {
    NonExistingPath,
    NonFilePath,
    CantRead(io::Error)
}

impl fmt::Display for SourceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SourceError::NonExistingPath => write!(f, "File doesn't exist!"),
            SourceError::NonFilePath => write!(f, "Not a file!"),
            SourceError::CantRead(e) => write!(f, "Error reading contents: {}", e)
        }
    }
}

impl TryFrom<&path::Path> for Problem {
    type Error = CreationError;

    fn try_from(path: &path::Path) -> Result<Self, Self::Error> {
        if !path.exists() {
            return Err(CreationError::NonExistingPath);
        } else if !path.is_dir() {
            return Err(CreationError::NonDirectoryPath);
        }

        let id: String = path.file_name()
            .ok_or(CreationError::BadPathFormat)?
            .to_string_lossy().into();
        let id = verify_id(id)
            .map_err(CreationError::BadId)?;

        let source = path.join("main.cc");
        let output = path.join("main.x");
        let proj_dirs = directories::ProjectDirs::from("com", "rdvdev2", "advocat");
        let work_dir = if let Some(proj_dirs) = proj_dirs {
            proj_dirs.cache_dir().join(&id)
        } else {
            path.join(".advocat")
        };
        let tmp_dir = env::temp_dir().join("advocat").join(&id);

        let is_private = id.starts_with('X');
        let has_main = file_has_main(&source)
            .map_err(CreationError::BadSource)?;

        let problem_url = String::from("https://jutge.org/problems/") + &id;
        let zip_url = problem_url.clone() + "/zip";
        let main_cc_url = problem_url + "/main/cc";

        Ok(Problem {
            id,
            source,
            output,
            work_dir,
            tmp_dir,
            is_private,
            has_main,
            zip_url,
            main_cc_url
        })
    }
}

fn verify_id(id: String) -> Result<String, IdError> {
    let re = Regex::new(r"[A-Z]\d{5}_[a-z]{2}").unwrap();
    if !re.is_match(&id) {
        return Err(IdError::InvalidId);
    }

    if id.starts_with('P') || id.starts_with('X') {
        Ok(id)
    } else {
        Err(IdError::UnsupportedType(id.as_bytes()[0].into()))
    }
}

fn file_has_main(path: &path::Path) -> Result<bool, SourceError> {
    debug!("Attempting to read {}", path.to_string_lossy());
    if !path.exists() {
        Err(SourceError::NonExistingPath)
    } else if !path.is_file() {
        Err(SourceError::NonFilePath)
    } else {
        let contents = fs::read_to_string(path)
            .map_err(SourceError::CantRead)?;
        debug!("Done reading {}", path.to_string_lossy());
        let re = Regex::new(r"int\s+main\s*(\s*)").unwrap();
        Ok(re.is_match(&contents))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test_utils;

    #[test]
    fn generate_public_problem() {
        let p = test_utils::get_problem("P00000_xx");
        assert_eq!(p.id, "P00000_xx");
        assert_eq!(p.source, test_utils::get_tests_folder().join("problems/P00000_xx/main.cc"));
        assert_eq!(p.output, test_utils::get_tests_folder().join("problems/P00000_xx/main.x"));
        assert!(!p.work_dir.to_string_lossy().is_empty());
        assert!(!p.is_private);
        assert!(p.has_main);
        assert_eq!(p.zip_url, "https://jutge.org/problems/P00000_xx/zip");
        assert_eq!(p.main_cc_url, "https://jutge.org/problems/P00000_xx/main/cc"); // Irrelevant, but still tested
    }

    #[test]
    fn generate_public_nomain_problem() {
        let p = test_utils::get_problem("P00001_xx");
        assert_eq!(p.id, "P00001_xx");
        assert_eq!(p.source, test_utils::get_tests_folder().join("problems/P00001_xx/main.cc"));
        assert_eq!(p.output, test_utils::get_tests_folder().join("problems/P00001_xx/main.x"));
        assert!(!p.work_dir.to_string_lossy().is_empty());
        assert!(!p.is_private);
        assert!(!p.has_main);
        assert_eq!(p.zip_url, "https://jutge.org/problems/P00001_xx/zip");
        assert_eq!(p.main_cc_url, "https://jutge.org/problems/P00001_xx/main/cc");
    }

    #[test]
    fn generate_private_nomain_problem() {
        let p = test_utils::get_problem("X00000_xx");
        assert_eq!(p.id, "X00000_xx");
        assert_eq!(p.source, test_utils::get_tests_folder().join("problems/X00000_xx/main.cc"));
        assert_eq!(p.output, test_utils::get_tests_folder().join("problems/X00000_xx/main.x"));
        assert!(!p.work_dir.to_string_lossy().is_empty());
        assert!(p.is_private);
        assert!(!p.has_main);
        assert_eq!(p.zip_url, "https://jutge.org/problems/X00000_xx/zip");
        assert_eq!(p.main_cc_url, "https://jutge.org/problems/X00000_xx/main/cc");
    }

    #[test]
    fn generate_problem_non_existing() {
        match test_utils::try_get_problem("foobar") {
            Err(CreationError::NonExistingPath) => {},
            _ => panic!()
        }
    }

    #[test]
    fn generate_problem_non_directory() {
        match test_utils::try_get_problem("P00000_xx/main.cc") {
            Err(CreationError::NonDirectoryPath) => {},
            _ => panic!()
        }
    }
    #[test]
    fn generate_problem_bad_format() {
        match test_utils::try_get_problem("..") {
            Err(CreationError::BadPathFormat) => {},
            _ => panic!()
        }
    }

    #[test]
    fn generate_problem_bad_id() {
        match test_utils::try_get_problem("") {
            Err(CreationError::BadId(_)) => {},
            _ => panic!()
        }
    }

    #[test]
    fn generate_problem_bad_main() {
        match test_utils::try_get_problem("P99999_xx") {
            Err(CreationError::BadSource(_)) => {},
            _ => panic!()
        }
    }

    #[test]
    fn verify_public_id() {
        let id = String::from("P00000_xx");
        assert_eq!(verify_id(id.clone()), Ok(id));
    }

    #[test]
    fn verify_private_id() {
        let id = String::from("X00000_xx");
        assert_eq!(verify_id(id.clone()), Ok(id));
    }

    #[test]
    fn verify_game_id() {
        let id = String::from("G00000_xx");
        assert_eq!(verify_id(id), Err(IdError::UnsupportedType('G')));
    }

    #[test]
    fn verify_invalid_id() {
        let id = String::from("FooBar");
        assert_eq!(verify_id(id), Err(IdError::InvalidId));
    }

    fn test_has_main(test_file: &str) -> Result<bool, SourceError> {
        let path = test_utils::get_tests_folder().join(test_file);
        file_has_main(path.as_path())
    }

    #[test]
    fn has_main_true() {
        assert!(test_has_main("problems/P00000_xx/main.cc").unwrap());
    }

    #[test]
    fn has_main_false() {
        assert!(!test_has_main("problems/P00001_xx/main.cc").unwrap());
    }

    #[test]
    fn has_main_non_existent() {
        match test_has_main("foobar") {
            Err(SourceError::NonExistingPath) => {}
            _ => panic!()
        }
    }

    #[test]
    fn has_main_non_file() {
        match test_has_main("problems") {
            Err(SourceError::NonFilePath) => {}
            _ => panic!()
        }
    }
}