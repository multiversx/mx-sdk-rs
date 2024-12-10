use std::{
    fs::{self, File},
    io::{ErrorKind, Write},
    path::{Path, PathBuf},
};

use super::RepoVersion;

const ZIP_NAME: &str = "mx-sdk-rs-download.zip";

pub struct RepoTempDownload {
    pub version: RepoVersion,
    pub temp_dir_path: PathBuf,
}

impl RepoTempDownload {
    pub async fn download_from_github(version: RepoVersion, temp_dir_path: PathBuf) -> Self {
        let tt_download = RepoTempDownload {
            version,
            temp_dir_path,
        };
        tt_download.download_binaries().await.unwrap();
        tt_download.delete_temp_folder();
        tt_download.unzip_binaries();
        tt_download.delete_zip();
        tt_download
    }

    fn zip_path(&self) -> PathBuf {
        self.temp_dir_path.join(ZIP_NAME)
    }

    pub fn repository_temp_dir_path(&self) -> PathBuf {
        self.temp_dir_path.join(self.version.temp_dir_name())
    }

    async fn download_binaries(&self) -> Result<(), reqwest::Error> {
        let response = reqwest::get(self.version.url()).await?.bytes().await?;
        if response.len() < 10000 {
            panic!(
                "Could not download artifact: {}",
                String::from_utf8_lossy(&response)
            );
        }

        let mut file = match File::create(self.zip_path()) {
            Err(why) => panic!("couldn't create {why}"),
            Ok(file) => file,
        };
        file.write_all(&response).unwrap();
        Ok(())
    }

    fn unzip_binaries(&self) {
        let file = File::open(self.zip_path()).unwrap();
        let mut zip = zip::ZipArchive::new(file).unwrap();
        zip.extract(Path::new(&self.temp_dir_path))
            .expect("Could not unzip artifact");
    }

    fn delete_zip(&self) {
        fs::remove_file(self.zip_path()).unwrap();
    }

    fn delete_temp_folder(&self) {
        fs::remove_dir_all(self.repository_temp_dir_path()).unwrap_or_else(|error| {
            // don't throw error if the temp repo doesn't exist
            if error.kind() != ErrorKind::NotFound {
                panic!("{:?}", error);
            }
        });
    }
}

impl Drop for RepoTempDownload {
    fn drop(&mut self) {
        self.delete_temp_folder();
    }
}
