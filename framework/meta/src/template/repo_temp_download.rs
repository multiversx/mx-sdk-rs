use std::{
    fs::{self, File},
    io::{ErrorKind, Write},
    path::{Path, PathBuf},
};

const REPOSITORY: &str = "https://github.com/multiversx/mx-sdk-rs/archive/refs/heads/master.zip";
const ZIP_NAME: &str = "./master.zip";
const REPOSITORY_TEMP_DIR_NAME: &str = "mx-sdk-rs-master";

pub enum RepoSource {
    Downloaded(RepoTempDownload),
    LocalPath(PathBuf),
}

impl RepoSource {
    pub async fn download_from_github(temp_dir_path: PathBuf) -> Self {
        RepoSource::Downloaded(RepoTempDownload::download_from_github(temp_dir_path).await)
    }

    pub fn from_local_path(repo_local_path: impl AsRef<Path>) -> Self {
        RepoSource::LocalPath(repo_local_path.as_ref().to_path_buf())
    }

    pub fn repo_path(&self) -> PathBuf {
        match self {
            RepoSource::Downloaded(repo_temp_download) => {
                repo_temp_download.repository_temp_dir_path()
            },
            RepoSource::LocalPath(local_path) => local_path.clone(),
        }
    }

    pub fn copy_repo_dir(&self, path_in_repo: impl AsRef<Path>, target_path: impl AsRef<Path>) {
        let from_path = self.repo_path().join(path_in_repo);
        copy_dir::copy_dir(from_path, target_path).unwrap();
    }
}

pub struct RepoTempDownload {
    pub temp_dir_path: PathBuf,
    pub repo_temp_dir_name: String,
}

impl RepoTempDownload {
    pub async fn download_from_github(temp_dir_path: PathBuf) -> Self {
        let tt_download = RepoTempDownload {
            temp_dir_path,
            repo_temp_dir_name: REPOSITORY_TEMP_DIR_NAME.to_string(),
        };
        tt_download.download_binaries().await.unwrap();
        tt_download.delete_temp_folder();
        tt_download.unzip_binaries();
        tt_download.delete_zip();
        tt_download
    }

    pub fn from_local_copy(repo_local_path: &Path, temp_dir_path: PathBuf) -> Self {
        let tt_download = RepoTempDownload {
            temp_dir_path,
            repo_temp_dir_name: REPOSITORY_TEMP_DIR_NAME.to_string(),
        };
        tt_download.delete_temp_folder();
        copy_dir::copy_dir(
            repo_local_path,
            tt_download
                .temp_dir_path
                .join(&tt_download.repo_temp_dir_name),
        )
        .unwrap();
        tt_download
    }

    fn zip_path(&self) -> PathBuf {
        self.temp_dir_path.join(ZIP_NAME)
    }

    pub fn repository_temp_dir_path(&self) -> PathBuf {
        self.temp_dir_path.join(REPOSITORY_TEMP_DIR_NAME)
    }

    async fn download_binaries(&self) -> Result<(), reqwest::Error> {
        let response = reqwest::get(REPOSITORY).await?.bytes().await?;

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
        zip.extract(Path::new(&self.temp_dir_path)).unwrap();
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
