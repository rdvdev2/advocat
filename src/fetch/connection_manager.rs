use std::{fmt, fs, io, path};
use std::io::Write;
use curl::easy;
use crate::{config, debug, warning};
use crate::fetch::credentials;

pub enum Error {
    CurlError(curl::Error),
    IoError(io::Error),
    AuthError
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::CurlError(e) => write!(f, "Connection error: {}", e),
            Error::IoError(e) => write!(f, "IO error: {}", e),
            Error::AuthError => write!(f, "The requested content isn't publicly available")
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
        debug!("Creating a connection manager");

        let cookie_store = config.cache_dir.join("cookies.jar");

        debug!("Creating the cURL Easy API handle");
        let mut handle = easy::Easy::new();
        handle.cookie_file(cookie_store.as_path())?;
        handle.cookie_jar(cookie_store.as_path())?;
        let mut cm = ConnectionManager { handle };

        if cm.check_is_authenticated()? {
            debug!("Client is authenticated, reusing previous session")
        } else if let Some(credentials) = &config.credentials {
            debug!("Credentials were provided, authenticating");
            if cm.try_to_authenticate(credentials)? {
                debug!("Authentication was successful");
            } else {
                warning!("The provided jutge.org credentials are invalid!");
            };
        } else {
            debug!("No credentials available, running in unauthenticated mode");
        }

        Ok(cm)
    }

    pub fn get_file(&mut self, url: &str, path: &path::Path) -> Result<(), Error> {
        debug!("Downloading {} to {}", url, path.to_string_lossy());
        let mut file = fs::File::create(path)?;

        self.handle.url(url)?;
        self.handle.write_function(move |data| file.write(data).or(Ok(0)))?;
        self.handle.perform()?;

        if let Some(content_type) = self.handle.content_type()? {
            if content_type.contains("html") {
                fs::remove_file(path)?;
                return Err(Error::AuthError);
            }
        }

        Ok(())
    }

    fn try_to_authenticate(&mut self, credentials: &credentials::Credentials) -> Result<bool, Error> {
        if let Some(form) = credentials.build_form() {
            debug!("Attempting to authenticate");
            self.handle.url("https://jutge.org/")?;
            self.handle.nobody(true)?;
            self.handle.httppost(form)?;
            self.handle.perform()?;
            self.handle.nobody(false)?;
            debug!("Authentication finished");
            self.check_is_authenticated()
        } else {
            debug!("Unable to generate the authentication form");
            Ok(false)
        }
    }

    fn check_is_authenticated(&mut self) -> Result<bool, Error> {
        let mut response = Vec::new();

        self.handle.url("https://jutge.org/dashboard")?;
        {
            let mut transfer = self.handle.transfer();
            transfer.write_function(|data| {
                response.extend_from_slice(data);
                Ok(data.len())
            })?;
            transfer.perform()?;
        }

        Ok(!String::from_utf8_lossy(&response).contains("Did you sign in?"))
    }
}