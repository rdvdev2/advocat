use std::path;
use crate::fetch::connection_manager::ConnectionManager;
use crate::fetch::unzip;

pub trait Resource {
    fn acquire(&self) -> Result<path::PathBuf, String>;
}

pub struct UserResource {
    path: path::PathBuf
}

impl UserResource {
    pub fn new(path: path::PathBuf) -> UserResource {
        UserResource { path }
    }
}

impl Resource for UserResource {
    fn acquire(&self) -> Result<path::PathBuf, String> {
        if self.path.exists() {
            Ok(self.path.to_path_buf())
        } else {
            Err(String::from("The path doesn't exist"))
        }
    }
}

pub struct OnlineResource<'a> {
    download_path: path::PathBuf,
    connection_manager: &'a ConnectionManager,
    url: String
}

impl<'a> OnlineResource<'a> {
    pub fn new(download_path: path::PathBuf, url: String, connection_manager: &ConnectionManager) -> OnlineResource {
        OnlineResource {
            download_path,
            url,
            connection_manager
        }
    }
}

impl<'a> Resource for OnlineResource<'a> {
    fn acquire(&self) -> Result<path::PathBuf, String> {
        if self.download_path.is_file() {
            Ok(self.download_path.to_path_buf())
        } else if self.download_path.is_dir() {
            Err(String::from("The download path is a directory"))
        } else {
            self.connection_manager.get_file(&self.url, &self.download_path)
                .map_err(|e| e.to_string())?;
            Ok(self.download_path.to_path_buf())
        }
    }
}

pub struct UnzipResource<T: Resource> {
    output_path: path::PathBuf,
    source_resource: T
}

impl<T: Resource> UnzipResource<T> {
    pub fn new(source_resource: T, output_path: path::PathBuf) -> UnzipResource<T> {
        UnzipResource {
            source_resource,
            output_path
        }
    }
}

impl<T: Resource> Resource for UnzipResource<T> {
    fn acquire(&self) -> Result<path::PathBuf, String> {
        if self.output_path.is_dir() {
            Ok(self.output_path.to_path_buf())
        } else if self.output_path.is_file() {
            Err(String::from("The extraction path is a file"))
        } else {
            let source = self.source_resource.acquire()?;
            unzip::unzip_samples(source.as_path(), self.output_path.as_path())
                .map_err(|e| e.to_string())?;
            Ok(self.output_path.to_path_buf())
        }

    }
}