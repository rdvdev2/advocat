use std::{env, fs, ops};
use std::path;
use crate::{config, problem};
use crate::config::Config;

static mut CONFIG: Option<Config> = None;

pub struct SelfCleaningTmp {
    dir: path::PathBuf
}

impl SelfCleaningTmp {
    pub fn new(module: &str, test_name: &str) -> SelfCleaningTmp {
        let dir = env::temp_dir().join("advocat-test").join(module).join(test_name);
        fs::create_dir_all(&dir).expect("Unable to get a temporal directory");
        SelfCleaningTmp { dir }
    }
}

impl ops::Deref for SelfCleaningTmp {
    type Target = path::Path;

    fn deref(&self) -> &Self::Target {
        self.dir.as_path()
    }
}

impl Drop for SelfCleaningTmp {
    fn drop(&mut self) {
        fs::remove_dir_all(self.dir.as_path()).expect("Couldn't remove the temporal directory");
    }
}

pub fn get_tests_folder() -> path::PathBuf {
    path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests")
}

pub fn get_config() -> config::Config {
    unsafe {
        if CONFIG.is_none() {
            CONFIG = Some(config::Config::generate());
        }
        CONFIG.clone().unwrap()
    }
}

pub fn try_get_problem(id: &str) -> Result<problem::Problem, problem::CreationError> {
    problem::Problem::new(get_tests_folder().join("problems").join(id).as_path(), &get_config())
}

pub fn get_problem(id: &str) -> problem::Problem {
    try_get_problem(id).expect("Couldn't generate a problem struct for the test")
}