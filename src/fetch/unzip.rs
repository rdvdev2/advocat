use std::{fmt, fs, io, path};
use crate::debug;

pub enum Error {
    CantReadFile(io::Error),
    CantCreateFile(io::Error),
    CantInflateFile(io::Error),
    ZipError(zip::result::ZipError)
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::CantReadFile(e) => write!(f, "Couldn't read the file: {}", e),
            Error::CantCreateFile(e) => write!(f, "Couldn't create a file: {}", e),
            Error::CantInflateFile(e) => write!(f, "Couldn't inflate a file: {}", e),
            Error::ZipError(e) => write!(f, "Zip raised an error: {:?}", e)
        }
    }
}

pub fn unzip_samples(zip_path: &path::Path, output_folder: &path::Path) -> Result<(), Error> {
    debug!("Unzipping {} to {}", zip_path.to_string_lossy(), output_folder.to_string_lossy());

    let zip_file = fs::File::open(zip_path)
        .map_err(Error::CantReadFile)?;
    let mut archive = zip::ZipArchive::new(zip_file)
        .map_err(Error::ZipError)?;

    debug!("Creating output folder {}", output_folder.to_string_lossy());
    fs::create_dir_all(output_folder).map_err(Error::CantCreateFile)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let outpath = match filter_samples(&file) {
            Some(path) => output_folder.join(path),
            None => continue
        };

        if file.is_dir() {
            debug!("Inflating folder {}", output_folder.to_string_lossy());
            fs::create_dir_all(outpath).map_err(Error::CantCreateFile)?;
        } else {
            if let Some(parent) = outpath.parent() {
                if !parent.exists() {
                    debug!("Inflating folder {}", parent.to_string_lossy());
                    fs::create_dir_all(parent).map_err(Error::CantCreateFile)?;
                }
            }

            debug!("Inflating file {}", outpath.to_string_lossy());
            let mut outfile = fs::File::create(&outpath).map_err(Error::CantCreateFile)?;
            io::copy(&mut file, &mut outfile).map_err(Error::CantInflateFile)?;
        }
    }
    Ok(())
}

fn filter_samples(zip_file: &zip::read::ZipFile) -> Option<path::PathBuf> {
    let path = zip_file.enclosed_name()?;
    if let Some(filename) = path.file_name() {
        if zip_file.is_file() && filename.to_string_lossy().starts_with("sample") {
            Some(filename.into())
        } else {
            None
        }
    } else {
        None
    }
}