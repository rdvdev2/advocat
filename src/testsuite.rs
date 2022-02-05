use std::{fmt, process};
use std::path;
use std::fs;
use std::io;
use std::io::{Write};
use termion::{color, style};
use crate::{debug, error, ux};

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

    pub fn run(&self, binary: &path::Path, should_skip: bool) -> usize {
        let mut pass_count: usize = 0;
        for (i, test) in self.tests.iter().enumerate() {
            let test_name = format!("{} test {}", self.name, i+1);
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
                    println!("==> Test diff:\n{}", result.diff);
                }
            }
        }

        pass_count
    }

    pub fn count(&self) -> usize {
        self.tests.len()
    }
}

struct Test {
    inputs: String,
    outputs: String
}

struct TestResult {
    status: ux::TaskStatus,
    error: Option<io::Error>,
    diff: String
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

    fn run(&self, binary: &path::Path) -> TestResult {
        debug!("Executing the binary");
        let process = process::Command::new(binary)
            .stdin(process::Stdio::piped())
            .stdout(process::Stdio::piped())
            .stderr(process::Stdio::piped())
            .spawn();

        let mut process = match process {
            Ok(p) => p,
            Err(e) => return TestResult { status: ux::TaskStatus::Fail, error: Some(e), diff: String::new() }
        };

        debug!("Sending inputs");
        match process.stdin.take() {
            Some(mut stdin) => if let Err(e) = stdin.write(self.inputs.as_bytes()) {
                return TestResult { status: ux::TaskStatus::Fail, error: Some(e), diff: String::new() }
            },
            None => debug!("The input pipe was closed by the program!")
        };

        debug!("Waiting for the program to end");
        let output = match process.wait_with_output() {
            Ok(o) => o,
            Err(e) => return TestResult { status: ux::TaskStatus::Fail, error: Some(e), diff: String::new() }
        };

        debug!("Capturing output");
        let binary_output = String::from_utf8_lossy(&output.stdout).to_string();

        debug!("Computing diff");
        let (pass, diff) = parse_diff(diff::lines(&self.outputs, &binary_output));
        let status = if pass {
            ux::TaskStatus::Pass
        } else {
            ux::TaskStatus::Fail
        };
        TestResult { status, error: None, diff }
    }
}

fn parse_diff(diff: Vec<diff::Result<&str>>) -> (bool, String) {
    debug!("Parsing diff");
    let mut pass = true;
    let mut diff_text = String::new();

    for line in diff {
        match line {
            diff::Result::Left(l) => {
                pass = false;
                diff_text.push_str(format!("TEST: {}{}{}\n", color::Fg(color::Green), l, style::Reset).as_str());
            }
            diff::Result::Both(l, _) => {
                diff_text.push_str(format!("      {}\n", l).as_str());
            }
            diff::Result::Right(r) => {
                pass = false;
                diff_text.push_str(format!("CODE: {}{}{}\n", color::Fg(color::Red), r, style::Reset).as_str());
            }
        }
    }

    (pass, diff_text)
}