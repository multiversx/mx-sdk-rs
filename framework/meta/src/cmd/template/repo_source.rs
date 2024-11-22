use std::{
    fs,
    path::{Path, PathBuf},
};

use super::{RepoTempDownload, RepoVersion};

pub enum RepoSource {
    Downloaded(RepoTempDownload),
    LocalPath(PathBuf),
}

impl RepoSource {
    pub async fn download_from_github(version: RepoVersion, temp_dir_path: PathBuf) -> Self {
        fs::create_dir_all(&temp_dir_path).unwrap();
        RepoSource::Downloaded(RepoTempDownload::download_from_github(version, temp_dir_path).await)
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
}
