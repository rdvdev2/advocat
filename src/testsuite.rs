use std::fmt;
use std::path;
use std::fs;
use std::io;
use crate::debug;

pub enum TestSuiteCreationError {
    PathDoesntExist,
    PathIsNotADir,
    CantReadDir(io::Error)
}

impl fmt::Display for TestSuiteCreationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TestSuiteCreationError::PathDoesntExist => write!(f, "The tests path doesn't exist!"),
            TestSuiteCreationError::PathIsNotADir => write!(f, "The tests path isn't a directory!"),
            TestSuiteCreationError::CantReadDir(e) => write!(f, "Couldn't read the tests directory: {}", e)
        }
    }
}

pub struct TestSuite {
    name: String,
    tests: Vec<Test>
}

impl TestSuite {
    // TODO: Tests
    pub fn from_dir(name: &str, dir: &path::Path) -> Result<TestSuite, TestSuiteCreationError> {
        if !dir.exists() {
            Err(TestSuiteCreationError::PathDoesntExist)
        } else if !dir.is_dir() {
            Err(TestSuiteCreationError::PathIsNotADir)
        } else {
            let mut suite = TestSuite { name: name.to_owned(), tests: Vec::new() };
            let mut input_files: Vec<fs::DirEntry> = fs::read_dir(dir)
                .map_err(TestSuiteCreationError::CantReadDir)?
                .filter_map(|r| r.ok())
                .filter(|f| f.path().extension().unwrap_or_else(|| "".as_ref()) == "inp")
                .collect();
            input_files.sort_by_key(|a| a.file_name());
            input_files.iter()
                .map(|inp| (inp.path(), inp.path().with_extension("cor")))
                .for_each(|(inp, out)|
                    if let Some(test) = Test::from_files(inp.as_path(), out.as_path()) {
                        suite.tests.push(test);
                });

            Ok(suite)
        }
    }
}

struct Test {
    inputs: String,
    outputs: String
}

impl Test {
    // TODO: Tests
    fn from_files(input_file: &path::Path, output_file: &path::Path) -> Option<Test> {
        if let Ok(inputs) = fs::read_to_string(input_file) {
            if let Ok(outputs) = fs::read_to_string(output_file) {
                debug!("Found test: {} => {}", input_file.to_string_lossy(), output_file.to_string_lossy());
                return Some(Test { inputs, outputs });
            }
        }
        None
    }
}