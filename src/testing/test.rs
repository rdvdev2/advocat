use std::{fs, io, path, process};
use std::io::Write;
use termion::color;
use crate::{debug, ux};
use crate::testing::diff_display;

pub struct Test {
    inputs: String,
    outputs: String
}

pub struct TestResult {
    pub status: ux::TaskStatus,
    pub error: Option<io::Error>,
    pub diff: String
}

impl Test {
    pub fn from_files(input_file: &path::Path, output_file: &path::Path) -> Option<Test> {
        if let Ok(inputs) = fs::read_to_string(input_file) {
            if let Ok(outputs) = fs::read_to_string(output_file) {
                debug!("Found test: {} => {}", input_file.to_string_lossy(), output_file.to_string_lossy());
                return Some(Test { inputs, outputs });
            }
        }
        None
    }

    pub fn run(&self, binary: &path::Path) -> TestResult {
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
    let mut dd =
        diff_display::DiffDisplay::new("Expected output", "Your output", &color::Green, &color::Red);

    for line in diff {
        match line {
            diff::Result::Left(l) => {
                pass = false;
                dd.write_left(l);
            }
            diff::Result::Both(l, r) => {
                dd.write_both(l, r);
            }
            diff::Result::Right(r) => {
                pass = false;
                dd.write_right(r);
            }
        }
    }

    dd.end();
    (pass, dd.build())
}