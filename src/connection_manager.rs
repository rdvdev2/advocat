use std::{fmt, fs, io, path};
use std::io::Write;
use curl::easy;
use crate::config;

pub enum Error {
    CurlError(curl::Error),
    IoError(io::Error)
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::CurlError(e) => write!(f, "Connection error: {}", e),
            Error::IoError(e) => write!(f, "IO error: {}", e)
        }
    }
}

impl From<curl::Error> for Error {
    fn from(e: curl::Error) -> Self {
        Self::CurlError(e)
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Self::IoError(e)
    }
}

pub struct ConnectionManager {
    handle: easy::Easy
}

impl ConnectionManager {
    pub fn new(config: &config::Config) -> Result<ConnectionManager, Error> {
        let cookie_store = config.cache_dir.join("cookies.jar");

        let mut handle = easy::Easy::new();
        handle.cookie_file(cookie_store.as_path())?;
        handle.cookie_jar(cookie_store.as_path())?;

        Ok(ConnectionManager { handle })
    }

    pub fn get_file(&mut self, url: &str, path: &path::Path) -> Result<(), Error> {
        let mut file = fs::File::create(path)?;

        self.handle.url(url)?;
        self.handle.write_function(move |data| file.write(data).or(Ok(0)))?;
        self.handle.perform()?;
        Ok(())
    }
}