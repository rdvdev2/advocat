use std::{env, path};
use crate::ux;

#[derive(Clone)]
pub struct Config {
    pub log_level: ux::LogLevel,
    pub config_dir: path::PathBuf,
    pub cache_dir: path::PathBuf,
    pub tmp_dir: path::PathBuf
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
            tmp_dir: env::temp_dir().join("advocat")
        };

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
}