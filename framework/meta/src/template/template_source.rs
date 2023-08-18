use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::folder_structure::{whitelisted_deep_copy, RelevantDirectories};

use super::{template_metadata::TemplateMetadata, RepoSource};

const TEMPLATES_PATH_IN_REPO: &str = "contracts/examples";
const TEMPLATE_TOML_FILE_NAME: &str = "mxsc-template.toml";

pub struct TemplateSource<'a> {
    pub repo_temp_dir: &'a RepoSource,
    pub source_path: PathBuf,
    pub metadata: TemplateMetadata,
}

impl<'a> TemplateSource<'a> {
    pub fn copy_template(&self, target_path: impl AsRef<Path>) {
        whitelisted_deep_copy(
            &self.source_path,
            target_path.as_ref(),
            &self.metadata.files_include,
        );
    }
}

pub fn template_sources(repo_temp_dir: &RepoSource) -> Vec<TemplateSource<'_>> {
    let templates_path = repo_temp_dir.repo_path().join(TEMPLATES_PATH_IN_REPO);
    let dirs = RelevantDirectories::find_all(&templates_path, &[]);
    let mut sources = Vec::new();
    for dir in dirs.iter_contract_crates() {
        let template_metadata_path = dir.path.join(TEMPLATE_TOML_FILE_NAME);
        if template_metadata_path.is_file() {
            if let Ok(s) = fs::read_to_string(&template_metadata_path) {
                let metadata: TemplateMetadata =
                    toml::from_str(s.as_str()).unwrap_or_else(|error| {
                        panic!("error parsing {TEMPLATE_TOML_FILE_NAME}: {error}")
                    });
                sources.push(TemplateSource {
                    repo_temp_dir,
                    source_path: dir.path.clone(),
                    metadata,
                })
            }
        }
    }
    sources
}
