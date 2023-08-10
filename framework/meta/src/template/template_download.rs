use crate::cli_args::TemplateArgs;
use convert_case::{Case, Casing};
use std::path::PathBuf;

use super::{
    repo_temp_download::RepoSource,
    template_source::{template_sources, TemplateSource},
    TemplateAdjuster,
};

pub async fn template_download(args: &TemplateArgs) {
    let repo_temp_download = RepoSource::download_from_github(std::env::temp_dir()).await;
    let downloader = TemplateDownloader::new(
        &repo_temp_download,
        args.template.clone(),
        args.name.clone(),
    );
    downloader.template_download();
    downloader.update_dependencies();
    downloader.rename_trait_to(args.template.to_case(Case::UpperCamel));
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

        TemplateDownloader {
            repo_source,
            template_source,
            target_path: target_path.clone(),
            adjuster: TemplateAdjuster::new(target_path, template_name),
        }
    }

    pub fn template_download(&self) {
        self.template_source.copy_template(&self.target_path);
    }
    pub fn update_dependencies(&self) {
        self.adjuster.update_dependencies();
    }
    pub fn rename_trait_to(&self, new_template_name: String) {
        self.adjuster.rename_trait_to(new_template_name);
    }
}
