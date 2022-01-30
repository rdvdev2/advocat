use core::fmt;
use std::path;
use std::fs;
use std::io;

use regex::Regex;

pub struct Problem {
    pub id: String,
    pub source: path::PathBuf,
    pub output: path::PathBuf,
    pub work_dir: path::PathBuf,
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

        let source = path.join(path::Path::new("main.cc"));
        let output = path.join(path::Path::new("main.x"));
        let proj_dirs = directories::ProjectDirs::from("com", "rdvdev2", "advocat");
        let work_dir = if let Some(proj_dirs) = proj_dirs {
            proj_dirs.cache_dir().join(path::Path::new(&id))
        } else {
            path.join(path::Path::new(".advocat"))
        };

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
    if !path.exists() {
        Err(SourceError::NonExistingPath)
    } else if !path.is_file() {
        Err(SourceError::NonFilePath)
    } else {
        let contents = fs::read_to_string(path)
            .map_err(SourceError::CantRead)?;
        let re = Regex::new(r"int\s+main\s*(\s*)").unwrap();
        Ok(re.is_match(&contents))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_test_path(filename: &str) -> path::PathBuf {
        path::PathBuf::from(env!("CARGO_MANIFEST_DIR").to_owned() + "/tests/" + filename)
    }

    fn generate_problem(folder: &str) -> Result<Problem, CreationError> {
        let path = get_test_path(folder);
        Problem::try_from(path.as_path())
    }

    #[test]
    fn generate_public_problem() {
        let p = generate_problem("problems/P00000_xx").unwrap();
        assert_eq!(p.id, "P00000_xx");
        assert_eq!(p.source, get_test_path("problems/P00000_xx/main.cc"));
        assert_eq!(p.output, get_test_path("problems/P00000_xx/main.x"));
        assert!(!p.work_dir.to_string_lossy().is_empty());
        assert!(!p.is_private);
        assert!(p.has_main);
        assert_eq!(p.zip_url, "https://jutge.org/problems/P00000_xx/zip");
        assert_eq!(p.main_cc_url, "https://jutge.org/problems/P00000_xx/main/cc"); // Irrelevant, but still tested
    }

    #[test]
    fn generate_public_nomain_problem() {
        let p = generate_problem("problems/P00001_xx").unwrap();
        assert_eq!(p.id, "P00001_xx");
        assert_eq!(p.source, get_test_path("problems/P00001_xx/main.cc"));
        assert_eq!(p.output, get_test_path("problems/P00001_xx/main.x"));
        assert!(!p.work_dir.to_string_lossy().is_empty());
        assert!(!p.is_private);
        assert!(!p.has_main);
        assert_eq!(p.zip_url, "https://jutge.org/problems/P00001_xx/zip");
        assert_eq!(p.main_cc_url, "https://jutge.org/problems/P00001_xx/main/cc");
    }

    #[test]
    fn generate_private_nomain_problem() {
        let p = generate_problem("problems/X00000_xx").unwrap();
        assert_eq!(p.id, "X00000_xx");
        assert_eq!(p.source, get_test_path("problems/X00000_xx/main.cc"));
        assert_eq!(p.output, get_test_path("problems/X00000_xx/main.x"));
        assert!(!p.work_dir.to_string_lossy().is_empty());
        assert!(p.is_private);
        assert!(!p.has_main);
        assert_eq!(p.zip_url, "https://jutge.org/problems/X00000_xx/zip");
        assert_eq!(p.main_cc_url, "https://jutge.org/problems/X00000_xx/main/cc");
    }

    #[test]
    fn generate_problem_non_existing() {
        match generate_problem("foobar") {
            Err(CreationError::NonExistingPath) => {},
            _ => panic!()
        }
    }

    #[test]
    fn generate_problem_non_directory() {
        match generate_problem("problems/P00000_xx/main.cc") {
            Err(CreationError::NonDirectoryPath) => {},
            _ => panic!()
        }
    }
    #[test]
    fn generate_problem_bad_format() {
        match generate_problem("problems/..") {
            Err(CreationError::BadPathFormat) => {},
            _ => panic!()
        }
    }

    #[test]
    fn generate_problem_bad_id() {
        match generate_problem("problems") {
            Err(CreationError::BadId(_)) => {},
            _ => panic!()
        }
    }

    #[test]
    fn generate_problem_bad_main() {
        match generate_problem("problems/P99999_xx") {
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
        let path = get_test_path(test_file);
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