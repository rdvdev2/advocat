use crate::testing::test;
use crate::{error, ux};
use std::fmt;
use std::fs;
use std::io;
use std::path;
use termion::style;

pub enum Error {
    PathDoesntExist,
    PathIsNotADir,
    CantReadDir(io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::PathDoesntExist => write!(f, "The tests path doesn't exist!"),
            Error::PathIsNotADir => write!(f, "The tests path isn't a directory!"),
            Error::CantReadDir(e) => write!(f, "Couldn't read the tests directory: {}", e),
        }
    }
}

pub struct TestSuite {
    name: String,
    tests: Vec<test::Test>,
}

impl TestSuite {
    pub fn from_dir(name: &str, dir: &path::Path) -> Result<TestSuite, Error> {
        if !dir.exists() {
            Err(Error::PathDoesntExist)
        } else if !dir.is_dir() {
            Err(Error::PathIsNotADir)
        } else {
            let mut suite = TestSuite {
                name: name.to_owned(),
                tests: Vec::new(),
            };
            let mut input_files: Vec<fs::DirEntry> = fs::read_dir(dir)
                .map_err(Error::CantReadDir)?
                .flatten()
                .filter(|f| f.path().extension().unwrap_or_else(|| "".as_ref()) == "inp")
                .collect();
            input_files.sort_by_key(|a| a.file_name());
            input_files
                .iter()
                .map(|inp| (inp.path(), inp.path().with_extension("cor")))
                .for_each(|(inp, out)| {
                    if let Some(test) = test::Test::from_files(inp.as_path(), out.as_path()) {
                        suite.tests.push(test);
                    }
                });

            Ok(suite)
        }
    }

    pub fn run(&self, binary: &path::Path, should_skip: bool) -> usize {
        let mut pass_count: usize = 0;
        for (i, test) in self.tests.iter().enumerate() {
            let test_name = format!("{} test {}", self.name, i + 1);
            if should_skip {
                ux::show_task_status(&test_name, ux::TaskType::Test, &ux::TaskStatus::SkipBad);
            } else {
                ux::show_task_status(&test_name, ux::TaskType::Test, &ux::TaskStatus::InProgress);
                let result = test.run(binary);
                ux::show_task_status(&test_name, ux::TaskType::Test, &result.status);
                if let Some(e) = result.error {
                    error!("Error running test: {}", e);
                } else if result.status.is_ok() {
                    pass_count += 1;
                } else {
                    ux::show_task_output(
                        "Test diff",
                        format!("{}{}", style::Reset, result.diff).as_str(),
                    );
                }
            }
        }

        pass_count
    }

    pub fn count(&self) -> usize {
        self.tests.len()
    }
}
