use crate::{
    cli_args::TemplateArgs,
    version_history::{validate_template_tag, LAST_TEMPLATE_VERSION},
};
use std::path::PathBuf;

use super::{
    template_source::{template_sources, TemplateSource},
    RepoSource, RepoVersion, TemplateAdjuster,
};

pub async fn template_download(args: &TemplateArgs) {
    let version = get_repo_version(&args.tag);
    let repo_temp_download = RepoSource::download_from_github(version, std::env::temp_dir()).await;
    let downloader = TemplateDownloader::new(
        &repo_temp_download,
        args.template.clone(),
        args.name.clone(),
    );
    downloader.copy_template(&downloader.template_source.metadata.files_include);
    downloader.update_dependencies();
    downloader.rename_template_to(args.template.clone());
}

pub(crate) fn get_repo_version(args_tag: &Option<String>) -> RepoVersion {
    if let Some(tag) = args_tag {
        assert!(validate_template_tag(tag), "invalid template tag");
        RepoVersion::Tag(tag.clone())
    } else {
        RepoVersion::Tag(LAST_TEMPLATE_VERSION.to_string())
    }
}

pub struct TemplateDownloader<'a> {
    pub repo_source: &'a RepoSource,
    pub template_source: TemplateSource<'a>,
    pub target_path: PathBuf,
    pub adjuster: TemplateAdjuster,
}

impl<'a> TemplateDownloader<'a> {
    pub fn new(repo_source: &'a RepoSource, template_name: String, target_path: PathBuf) -> Self {
        let template_sources = template_sources(repo_source);
        let template_source = template_sources
            .into_iter()
            .find(|source| source.metadata.name == template_name)
            .unwrap_or_else(|| panic!("Unknown template {template_name}"));

        let metadata = template_source.metadata.clone();
        TemplateDownloader {
            repo_source,
            template_source,
            target_path: target_path.clone(),
            adjuster: TemplateAdjuster::new(target_path, metadata),
        }
    }

    pub fn copy_template(&self, files_to_copy: &[String]) {
        self.template_source
            .copy_template(&self.target_path, files_to_copy);
    }
    pub fn update_dependencies(&self) {
        self.adjuster.update_dependencies();
    }
    pub fn rename_template_to(&self, new_template_name: String) {
        self.adjuster.rename_template_to(new_template_name);
    }
}
