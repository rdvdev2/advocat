use std::{env, fs, path};
use configparser::ini;
use crate::connection_manager::Credentials;
use crate::{connection_manager, debug, ux};

#[derive(Clone)]
pub struct Config {
    pub log_level: ux::LogLevel,
    pub config_dir: path::PathBuf,
    pub cache_dir: path::PathBuf,
    pub tmp_dir: path::PathBuf,
    pub credentials: Option<Credentials>
}

impl Config {
    pub fn generate() -> Config {
        let dirs = directories::ProjectDirs::from("com", "rdvdev2", "advocat");
        let (config_dir, cache_dir) = if let Some(dirs) = dirs {
            (dirs.config_dir().to_path_buf(), dirs.cache_dir().to_path_buf())
        } else {
            (env::temp_dir().join("advocat-config"), env::temp_dir().join("advocat-cache"))
        };

        let mut config = Config {
            log_level: ux::LogLevel::Info,
            config_dir,
            cache_dir,
            tmp_dir: env::temp_dir().join("advocat"),
            credentials: None
        };

        debug!("Creating directories");

        #[allow(unused_must_use)]
        {   // We don't immediately depend on this directories existing
            fs::create_dir_all(config.config_dir.as_path());
            fs::create_dir_all(config.cache_dir.as_path());
            fs::create_dir_all(config.tmp_dir.as_path());
        }

        debug!("Loading config file");
        config.load_config_file();

        let mut args = env::args();
        let _executable = args.next();
        for arg in args {
            match arg.as_str() {
                "-d" | "--debug" => config.log_level = ux::LogLevel::Debug,
                "-h" | "--help" => todo!("Add a help message"),
                _ => {}
            }
        }

        config
    }

    fn load_config_file(&mut self) {
        let config_file_path = self.config_dir.join("config.ini");

        // Attempt to create the file, but don't fail if the file can't be created
        #[allow(unused_must_use)]
        if !config_file_path.is_file() {
            fs::File::create(config_file_path.as_path());
        }

        let config_file = ini::Ini::new().load(config_file_path.as_path());

        if let Ok(config_file) = config_file {

            let email = &config_file["auth"]["email"];
            let password = &config_file["auth"]["password"];
            if let (Some(email), Some(password)) = (email, password) {
                self.credentials = Some(connection_manager::Credentials::new(email.as_bytes(), password.as_bytes()));
            }
        }
    }
}