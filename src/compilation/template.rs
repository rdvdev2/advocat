use crate::{debug, problem};
use std::io::Write;
use std::{env, fmt, fs, io, path};

macro_rules! get_template {
    ($template:expr) => {
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/templates/",
            $template
        ))
    };
}

fn apply_normal_template(original: &str) -> String {
    format!(
        get_template!("normal.cc.in"),
        original = original,
        stub = get_template!("stub.cc.in")
    )
}

fn apply_nomain_template(original: &str, main: &str) -> String {
    format!(
        get_template!("nomain.cc.in"),
        original = original,
        stub = get_template!("stub.cc.in"),
        main = main
    )
}

pub enum Error {
    ErrorCreatingTmpFolder(io::Error),
    ErrorCreatingFile(io::Error),
    CantReadSources(io::Error),
    CantReadDownloadedMain(io::Error),
    ErrorWritingFile(io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::ErrorCreatingTmpFolder(e) => {
                write!(f, "Couldn't create a temporal folder: {}", e)
            }
            Error::ErrorCreatingFile(e) => write!(f, "Couldn't create the file: {}", e),
            Error::CantReadSources(e) => write!(f, "Couldn't read your sources: {}", e),
            Error::CantReadDownloadedMain(e) => {
                write!(f, "Couldn't read the downloaded sources: {}", e)
            }
            Error::ErrorWritingFile(e) => write!(f, "Unable to write the file: {}", e),
        }
    }
}

impl From<Error> for crate::Error {
    fn from(e: Error) -> Self {
        crate::Error {
            description: format!("Couldn't generate a main.cc file to compile: {}", e),
            exitcode: exitcode::DATAERR,
        }
    }
}

pub fn generate_main(problem: &problem::Problem) -> Result<path::PathBuf, Error> {
    let generated_main_path = problem.tmp_dir.join("main.cc");

    debug!("Creating {}...", generated_main_path.to_string_lossy());
    fs::create_dir_all(generated_main_path.parent().unwrap())
        .map_err(Error::ErrorCreatingTmpFolder)?;
    let mut generated_main =
        fs::File::create(&generated_main_path).map_err(Error::ErrorCreatingFile)?;

    debug!("Reading {}...", problem.source.to_string_lossy());
    let original = fs::read_to_string(problem.source.as_path()).map_err(Error::CantReadSources)?;

    debug!("Generating contents...");
    let generated_main_contents = if problem.has_main {
        apply_normal_template(original.as_str())
    } else {
        let main = fs::read_to_string(problem.work_dir.join("main.cc"))
            .map_err(Error::CantReadDownloadedMain)?;
        apply_nomain_template(original.as_str(), main.as_str())
    };

    debug!(
        "Writing contents to {}...",
        generated_main_path.to_string_lossy()
    );
    generated_main
        .write_all(generated_main_contents.as_ref())
        .map_err(Error::ErrorWritingFile)?;
    Ok(generated_main_path)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn apply_normal_template_test() {
        let original = "// I'M THE ORIGINAL ONE";
        let expected_output = env::current_dir()
            .unwrap()
            .join("tests")
            .join("resources")
            .join("normal.cc");

        assert_eq!(
            apply_normal_template(original),
            fs::read_to_string(expected_output).unwrap()
        );
    }

    #[test]
    fn apply_nomain_template_test() {
        let original = "// I'M THE ORIGINAL ONE";
        let main = "// I'M THE MAIN ONE";
        let expected_output = env::current_dir()
            .unwrap()
            .join("tests")
            .join("resources")
            .join("nomain.cc");

        assert_eq!(
            apply_nomain_template(original, main),
            fs::read_to_string(expected_output).unwrap()
        );
    }
}
