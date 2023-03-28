use std::{fs, path::PathBuf};

use crate::folder_structure::RelevantDirectories;

use super::{repo_temp_download::RepoSource, template_metadata::TemplateMetadata};

const TEMPLATES_PATH_IN_REPO: &str = "contracts/examples";

pub struct TemplateSource<'a> {
    pub repo_temp_dir: &'a RepoSource,
    pub source_path: PathBuf,
    pub metadata: TemplateMetadata,
}

pub fn template_sources<'a>(repo_temp_dir: &'a RepoSource) -> Vec<TemplateSource<'a>> {
    let templates_path = repo_temp_dir.repo_path().join(TEMPLATES_PATH_IN_REPO);
    let dirs = RelevantDirectories::find_all(&templates_path, &[]);
    let mut sources = Vec::new();
    for dir in dirs.iter_contract_crates() {
        println!("{}", dir.path.display());
        let template_metadata_path = dir.path.join("template.toml");
        if template_metadata_path.is_file() {
            if let Ok(s) = fs::read_to_string(&template_metadata_path) {
                let metadata: TemplateMetadata = toml::from_str(s.as_str())
                    .unwrap_or_else(|error| panic!("error parsing template.toml: {error}"));
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
