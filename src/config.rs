use core::fmt;
use std::{env, fs, io, path};
use configparser::ini;
use crate::connection_manager::Credentials;
use crate::{connection_manager, debug, ux};

#[derive(Debug)]
pub enum Error {
    CantCreateConfigDir(io::Error),
    CantCreateCacheDir(io::Error),
    CantCreateTmpDir(io::Error),
    UnknownProblemDir(io::Error),
    CantCreateConfigFile(io::Error)
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::CantCreateConfigDir(e) => write!(f, "Unable to create a directory for configuration: {}", e),
            Error::CantCreateCacheDir(e) => write!(f, "Unable to create a directory for program cache: {}", e),
            Error::CantCreateTmpDir(e) => write!(f, "Unable to create a directory for temporal files: {}", e),
            Error::UnknownProblemDir(e) => write!(f, "Can't determine the problem dir: {}", e),
            Error::CantCreateConfigFile(e) => write!(f, "Can't create the config file: {}", e)
        }
    }
}

impl From<Error> for crate::Error {
    fn from(e: Error) -> Self {
        crate::Error {
            description: format!("Error preparing the program: {}", e),
            exitcode: exitcode::IOERR
        }
    }
}

#[derive(Clone)]
pub struct Config {
    pub log_level: ux::LogLevel,
    pub problem_dir: path::PathBuf,
    pub config_dir: path::PathBuf,
    pub cache_dir: path::PathBuf,
    pub tmp_dir: path::PathBuf,
    pub credentials: Option<Credentials>
}

impl Config {
    pub fn generate() -> Result<Config, Error> {
        let dirs = directories::ProjectDirs::from("com", "rdvdev2", "advocat");
        let (config_dir, cache_dir) = if let Some(dirs) = dirs {
            (dirs.config_dir().to_path_buf(), dirs.cache_dir().to_path_buf())
        } else {
            (env::temp_dir().join("advocat-config"), env::temp_dir().join("advocat-cache"))
        };
        let problem_dir = env::current_dir()
            .map_err(Error::UnknownProblemDir)?;

        let mut config = Config {
            log_level: ux::LogLevel::Info,
            problem_dir,
            config_dir,
            cache_dir,
            tmp_dir: env::temp_dir().join("advocat"),
            credentials: None
        };

        debug!("Creating directories");
        fs::create_dir_all(config.config_dir.as_path())
            .map_err(Error::CantCreateConfigDir)?;
        fs::create_dir_all(config.cache_dir.as_path())
            .map_err(Error::CantCreateCacheDir)?;
        fs::create_dir_all(config.tmp_dir.as_path())
            .map_err(Error::CantCreateTmpDir)?;

        debug!("Loading config file");
        config.load_config_file()?;

        let mut args = env::args();
        let _executable = args.next();
        for arg in args {
            match arg.as_str() {
                "-d" | "--debug" => config.log_level = ux::LogLevel::Debug,
                "-h" | "--help" => todo!("Add a help message"),
                _ => {}
            }
        }

        Ok(config)
    }

    fn load_config_file(&mut self) -> Result<(), Error> {
        let config_file_path = self.config_dir.join("config.ini");

        if !config_file_path.is_file() {
            fs::File::create(config_file_path.as_path())
                .map_err(Error::CantCreateConfigFile)?;
        }

        let config_file = ini::Ini::new().load(config_file_path.as_path());

        if let Ok(config_file) = config_file {
            if let Some(auth) = config_file.get("auth") {
                let email = auth.get("email");
                let password = auth.get("password");

                if let (Some(Some(email)), Some(Some(password))) = (email, password) {
                    self.credentials = Some(connection_manager::Credentials::new(email.as_bytes(), password.as_bytes()));
                }
            }
        }

        Ok(())
    }
}