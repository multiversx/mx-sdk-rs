use crate::{cli::TemplateArgs, version::FrameworkVersion, version_history::LAST_TEMPLATE_VERSION};

use super::{
    ContractCreatorTarget, RepoSource, RepoVersion, TemplateAdjuster,
    template_source::{TemplateSource, template_sources},
};

/// Creates a new contract on disk, from a template, given a name.
pub async fn create_contract(args: &TemplateArgs) {
    let target = target_from_args(args);
    let contract_dir = target.contract_dir();

    if contract_dir.exists() && !args.overwrite {
        eprintln!(
            "Error: destination `{}` already exists. Use --overwrite to overwrite.",
            contract_dir.display()
        );
        std::process::exit(1);
    }

    let version = get_repo_version(&args.tag);
    let version_tag: FrameworkVersion = version.get_tag();
    let repo_temp_download = RepoSource::download_from_github(version, std::env::temp_dir()).await;

    let creator = ContractCreator::new(
        &repo_temp_download,
        args.template.clone(),
        target,
        false,
        args.author.clone(),
    );

    // Remove only after the download succeeded and the template name is validated,
    // so a failure in those steps does not cause unnecessary data loss.
    if args.overwrite && contract_dir.exists() {
        std::fs::remove_dir_all(&contract_dir)
            .unwrap_or_else(|e| panic!("failed to remove existing directory: {e}"));
    }

    creator.create_contract(version_tag);
}

fn target_from_args(args: &TemplateArgs) -> ContractCreatorTarget {
    let target_path = args.path.clone().unwrap_or_default();
    let new_name = args.name.as_deref().unwrap_or(&args.template);
    ContractCreatorTarget::new(target_path, new_name)
}

pub(crate) fn get_repo_version(args_tag: &Option<String>) -> RepoVersion {
    if let Some(tag) = args_tag {
        RepoVersion::Tag(tag.clone())
    } else {
        RepoVersion::Tag(LAST_TEMPLATE_VERSION.to_string())
    }
}

/// Coordinates the creation of a new contract from a template.
pub struct ContractCreator<'a> {
    pub repo_source: &'a RepoSource,
    pub template_source: TemplateSource<'a>,
    pub target: ContractCreatorTarget,
    pub adjuster: TemplateAdjuster,
}

impl<'a> ContractCreator<'a> {
    pub fn new(
        repo_source: &'a RepoSource,
        template_name: String,
        target: ContractCreatorTarget,
        keep_paths: bool,
        new_author: Option<String>,
    ) -> Self {
        let template_sources = template_sources(repo_source);
        let template_source = template_sources
            .into_iter()
            .find(|source| source.metadata.name == template_name)
            .unwrap_or_else(|| panic!("Unknown template {template_name}"));

        let metadata = template_source.metadata.clone();
        ContractCreator {
            repo_source,
            template_source,
            target: target.clone(),
            adjuster: TemplateAdjuster {
                metadata,
                target,
                keep_paths,
                new_author,
            },
        }
    }

    pub fn create_contract(&self, args_tag: FrameworkVersion) {
        self.copy_template(args_tag);
        self.update_dependencies();
        self.rename_template();
    }

    pub fn copy_template(&self, args_tag: FrameworkVersion) {
        self.template_source
            .copy_template(self.target.contract_dir(), args_tag);
    }

    pub fn update_dependencies(&self) {
        self.adjuster.update_cargo_toml_files();
    }

    pub fn rename_template(&self) {
        self.adjuster.rename_template_to();
    }
}
