use std::path;
use curl::easy;
use std::fs;
use std::io;
use std::io::Write;
use crate::debug;

enum DownloadError {
    CantCreateFile(io::Error),
    CurlError(curl::Error)
}

fn download_file(url: &str, path: &path::Path) -> Result<(), DownloadError> {
    debug!("Downloading {} to {}", url, path.to_string_lossy());

    let mut file = fs::File::create(path)
        .map_err(DownloadError::CantCreateFile)?;

    let mut easy = easy::Easy::new();
    easy.url(url).map_err(DownloadError::CurlError)?;
    easy.write_function(move |data| {
        file.write(data).or(Ok(0))
    }).map_err(DownloadError::CurlError)?;
    easy.perform().map_err(DownloadError::CurlError)
}

enum UnzipError {
    CantReadFile(io::Error),
    CantCreateFile(io::Error),
    CantInflateFile(io::Error),
    ZipError(zip::result::ZipError)
}

fn unzip_file(zip_path: &path::Path, output_folder: &path::Path) -> Result<(), UnzipError> {
    debug!("Unzipping {} to {}", zip_path.to_string_lossy(), output_folder.to_string_lossy());

    let zip_file = fs::File::open(zip_path)
        .map_err(UnzipError::CantReadFile)?;
    let mut archive = zip::ZipArchive::new(zip_file)
        .map_err(UnzipError::ZipError)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let outpath = match file.enclosed_name() {
            Some(path) => output_folder.join(path),
            None => continue
        };

        if file.is_dir() {
            debug!("Inflating folder {}", output_folder.to_string_lossy());
            fs::create_dir_all(outpath).map_err(UnzipError::CantCreateFile)?;
        } else {
            if let Some(parent) = outpath.parent() {
                debug!("Inflating folder {}", parent.to_string_lossy());
                fs::create_dir_all(parent).map_err(UnzipError::CantCreateFile)?;
            }

            debug!("Inflating file {}", outpath.to_string_lossy());
            let mut outfile = fs::File::create(&outpath).map_err(UnzipError::CantCreateFile)?;
            io::copy(&mut file, &mut outfile).map_err(UnzipError::CantInflateFile)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use std::env;
    use std::fs;

    #[test]
    fn download_file_works() {
        let file = env::temp_dir().join("advocat-test").join("problem.zip");
        fs::create_dir_all(file.parent().unwrap()).unwrap();
        let url = "https://jutge.org/problems/P10051_en/zip";

        assert!(download_file(url, &file).is_ok());
        assert!(file.is_file());
        // TODO: Check the contents of the file

        fs::remove_file(file).unwrap();
    }


    #[test]
    fn unzip_file_works() {
        let file = env::temp_dir().join("advocat-test").join("problem-2.zip");
        fs::create_dir_all(file.parent().unwrap()).unwrap();
        let url = "https://jutge.org/problems/P10051_en/zip";

        assert!(download_file(url, &file).is_ok(),
            "The test file can't be downloaded, test ABORTED");

        let output_folder = env::temp_dir().join("advocat-test").join("unzip");
        assert!(unzip_file(&file, &output_folder).is_ok());
        assert!(output_folder.is_dir());
        assert_ne!(output_folder.read_dir().unwrap().count(), 0);

        fs::remove_dir_all(output_folder).unwrap();
        fs::remove_file(file).unwrap();
    }
}