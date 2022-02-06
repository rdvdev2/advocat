use std::{fmt, fs, io, path};
use std::io::Write;
use curl::easy;
use crate::{config, debug};

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

        if let Some(credentials) = &config.credentials {
            debug!("Credentials were provided, authenticating");
            cm.try_to_authenticate(credentials)?;
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

    fn try_to_authenticate(&mut self, credentials: &Credentials) -> Result<(), Error> {
        if let Some(form) = credentials.build_form() {
            debug!("Attempting to authenticate");
            self.handle.url("https://jutge.org/")?;
            self.handle.nobody(true)?;
            self.handle.httppost(form)?;
            self.handle.perform()?;
            self.handle.nobody(false)?;
            debug!("Authentication finished");
            Ok(())
        } else {
            debug!("Unable to generate the authentication form");
            Ok(())
        }

    }
}

#[derive(Clone)]
pub struct Credentials {
    email: Vec<u8>,
    password: Vec<u8>
}

impl Credentials {
    pub fn new(email: &[u8], password: &[u8]) -> Credentials {
        Credentials {
            email: Vec::from(email),
            password: Vec::from(password)
        }
    }

    fn build_form(&self) -> Option<easy::Form> {
        let mut form = easy::Form::new();

        form.part("email").contents(self.email.as_slice()).add().ok()?;
        form.part("password").contents(self.password.as_slice()).add().ok()?;
        form.part("submit").contents(b"").add().ok()?;

        Some(form)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn credentials_build_form_test() {
        let credentials = Credentials::new(b"me@example.com", b"1234");
        assert!(credentials.build_form().is_some());
    }
}