use std::{fmt, process};
use std::path;
use std::fs;
use std::io;
use std::io::{Write};
use termion::{color, style};
use crate::{debug, error, ux};

pub enum Error {
    PathDoesntExist,
    PathIsNotADir,
    CantReadDir(io::Error)
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::PathDoesntExist => write!(f, "The tests path doesn't exist!"),
            Error::PathIsNotADir => write!(f, "The tests path isn't a directory!"),
            Error::CantReadDir(e) => write!(f, "Couldn't read the tests directory: {}", e)
        }
    }
}

pub struct TestSuite {
    name: String,
    tests: Vec<Test>
}

impl TestSuite {
    pub fn from_dir(name: &str, dir: &path::Path) -> Result<TestSuite, Error> {
        if !dir.exists() {
            Err(Error::PathDoesntExist)
        } else if !dir.is_dir() {
            Err(Error::PathIsNotADir)
        } else {
            let mut suite = TestSuite { name: name.to_owned(), tests: Vec::new() };
            let mut input_files: Vec<fs::DirEntry> = fs::read_dir(dir)
                .map_err(Error::CantReadDir)?
                .flatten()
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
                    ux::show_task_output("Test diff", format!("{}{}", style::Reset, result.diff).as_str());
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

// TODO: This is a mess, fix it
fn parse_diff(diff: Vec<diff::Result<&str>>) -> (bool, String) {
    debug!("Parsing diff");
    let mut pass = true;
    let mut dd =
        DiffDisplay::new("Expected output", "Your output", &color::Green, &color::Red);

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
    (pass, dd.text)
}



struct DiffDisplay {
    text: String,
    side_width: usize,
    left_color: &'static dyn color::Color,
    right_color: &'static dyn color::Color
}

impl DiffDisplay {
    fn new(left_title: &str, right_title: &str, left_color: &'static dyn color::Color, right_color: &'static dyn color::Color) -> Self {
        let side_width = ((ux::get_terminal_width() - 7) / 2) as usize;
        let mut dd = DiffDisplay { text: String::new(), side_width, left_color, right_color };
        dd.draw_horizontal_line('╭', '┬', '╮');
        dd.write_centered_row(left_title, right_title);
        dd.draw_horizontal_line('├', '┼', '┤');
        dd
    }

    fn draw_horizontal_line(&mut self, left: char, mid: char, right: char) {
        self.text.push_str(format!("{l}─{:─^w$}─{m}─{:─^w$}─{r}\n", "", "",
            l = left, m = mid, r = right, w = self.side_width).as_str());
    }

    fn end(&mut self) {
        self.draw_horizontal_line('╰', '┴', '╯');
    }

    fn write_centered_row(&mut self, left: &str, right: &str) {
        let left = self.trim_line(left);
        let right = self.trim_line(right);
        self.text.push_str(format!("│ {l:^w$} │ {r:^w$} │\n",
            l = left, r = right, w = self.side_width).as_str());
    }

    fn write_row(&mut self, left: &str, mid: char, right: &str, left_color: &dyn color::Color, right_color: &dyn color::Color) {
        let left = self.trim_line(left);
        let right = self.trim_line(right);
        self.text.push_str(format!("│ {lc}{l:w$}{nc} {m} {rc}{r:w$}{nc} │\n",
            l = left, m = mid, r = right,
            lc = color::Fg(left_color),
            rc = color::Fg(right_color),
            nc = color::Fg(color::Reset),
            w = self.side_width).as_str());
    }

    fn write_left(&mut self, left: &str) {
        self.write_row(left, '<', "", self.left_color, self.right_color);
    }

    fn write_right(&mut self, right: &str) {
        self.write_row("", '>', right, self.left_color, self.right_color);
    }

    fn write_both(&mut self, left: &str, right: &str) {
        self.write_row(left, '│', right, &color::Reset, &color::Reset);
    }

    fn trim_line(&self, line: &str) -> String {
        if line.len() <= self.side_width {
            line.to_owned()
        } else {
            let line = &line[0..self.side_width-3];
            line.to_owned() + "..."
        }
    }
}