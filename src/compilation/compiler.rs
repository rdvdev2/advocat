use std::{fmt, io, path, process};
use crate::{debug, problem};

pub static P1XX: Compiler = Compiler {
    command: "g++",
    flags1: &["-D_JUDGE_", "-DNDEBUG", "-O2", "-Wall", "-Wextra", "-Werror", "-Wno-sign-compare", "-Wshadow"],
    flags2: &["-D_JUDGE_", "-DNDEBUG", "-O2"]
};

pub struct Compiler<'a> {
    command: &'a str,
    flags1: &'a[&'a str],
    flags2: &'a[&'a str]
}

pub enum CompilationError {
    SourceDoesNotExist,
    SourceIsADir,
    OutputIsADir,
    ExecutionError(io::Error),
    CompilerError(String),
    MissingOutput
}

impl fmt::Display for CompilationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CompilationError::SourceDoesNotExist => write!(f, "Source file doesn't exist!"),
            CompilationError::SourceIsADir => write!(f, "Source is a directory!"),
            CompilationError::OutputIsADir => write!(f, "Output is a directory!"),
            CompilationError::ExecutionError(e) => write!(f, "Command execution error: {}", e),
            CompilationError::CompilerError(stderr) => write!(f, "The compiler raised an error:\n{}", stderr),
            CompilationError::MissingOutput => write!(f, "Can't find the compiler output!")
        }
    }
}

pub struct CompileProcessError {
    pub pass: u8,
    pub error: CompilationError
}

impl fmt::Display for CompileProcessError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Pass {}: {}", self.pass, self.error)
    }
}

enum CompilationType {
    Object,
    Binary
}

impl Compiler<'_> {
    fn run(&self, source: &path::Path, output: &path::Path, compilation_type: CompilationType, flags: &[&str]) -> Result<(), CompilationError>{
        if !source.exists() {
            Err(CompilationError::SourceDoesNotExist)
        } else if source.is_dir() {
            Err(CompilationError::SourceIsADir)
        } else if output.is_dir() {
            Err(CompilationError::OutputIsADir)
        } else {
            let mut command = process::Command::new(&self.command);
            command.args(flags)
                .args(["-o", output.to_string_lossy().as_ref()]);

            match compilation_type {
                CompilationType::Object => command.args(["-c", source.to_string_lossy().as_ref()]),
                CompilationType::Binary => command.arg(source.to_string_lossy().to_string())
            };

            debug!("Running command: {:?}", command);
            let command = command.output().map_err(CompilationError::ExecutionError)?;

            if command.status.success() {
                if output.is_file() {
                    Ok(())
                } else {
                    Err(CompilationError::MissingOutput)
                }
            } else {
                Err(CompilationError::CompilerError(String::from_utf8_lossy(&command.stderr).to_string()))
            }
        }
    }

    fn compile_first_pass(&self, source: &path::Path, output: &path::Path) -> Result<(), CompilationError> {
        self.run(source, output, CompilationType::Object, self.flags1)
    }

    fn compile_and_link_first_pass(&self, source: &path::Path, output: &path::Path) -> Result<(), CompilationError> {
        self.run(source, output, CompilationType::Binary, self.flags1)
    }

    fn compile_and_link_second_pass(&self, source: &path::Path, output: &path::Path) -> Result<(), CompilationError> {
        self.run(source, output, CompilationType::Binary, self.flags2)
    }

    pub fn compile_problem(&self, problem: &problem::Problem, generated_source: &path::Path) -> Result<(), CompileProcessError>{
        debug!("Running the first pass compilation (P1++ checks)");
        if problem.has_main {
            let output = problem.tmp_dir.join("main.x");
            self.compile_and_link_first_pass(problem.source.as_path(), output.as_path())
        } else {
            let output = problem.tmp_dir.join("main.o");
            self.compile_first_pass(problem.source.as_path(), output.as_path())
        }.map_err(|error| CompileProcessError {pass: 1, error})?;

        debug!("Running the second pass compilation (G++ binary)");
        self.compile_and_link_second_pass(generated_source, problem.output.as_path())
            .map_err(|error| CompileProcessError {pass: 2, error})
    }
}