use crate::{
    cli_args::TemplateArgs,
    version_history::{validate_template_tag, LAST_TEMPLATE_VERSION},
};

use super::{
    template_source::{template_sources, TemplateSource},
    ContractCreatorTarget, RepoSource, RepoVersion, TemplateAdjuster,
};

/// Creates a new contract on disk, from a template, given a name.
pub async fn create_contract(args: &TemplateArgs) {
    let version = get_repo_version(&args.tag);
    let repo_temp_download = RepoSource::download_from_github(version, std::env::temp_dir()).await;
    let target = target_from_args(args);

    let creator = ContractCreator::new(&repo_temp_download, args.template.clone(), target, false);

    creator.create_contract();
}

fn target_from_args(args: &TemplateArgs) -> ContractCreatorTarget {
    let new_name = args.name.clone().unwrap_or_else(|| args.template.clone());
    let target_path = args.path.clone().unwrap_or_default();
    ContractCreatorTarget {
        target_path,
        new_name,
    }
}

pub(crate) fn get_repo_version(args_tag: &Option<String>) -> RepoVersion {
    if let Some(tag) = args_tag {
        assert!(validate_template_tag(tag), "invalid template tag");
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
            },
        }
    }

    pub fn create_contract(&self) {
        self.copy_template();
        self.update_dependencies();
        self.rename_template();
    }

    pub fn copy_template(&self) {
        self.template_source
            .copy_template(self.target.contract_dir());
    }

    pub fn update_dependencies(&self) {
        self.adjuster.update_dependencies();
    }

    pub fn rename_template(&self) {
        self.adjuster.rename_template_to();
    }
}
