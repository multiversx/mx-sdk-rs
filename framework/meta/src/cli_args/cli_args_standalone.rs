use std::path::PathBuf;

use clap::{ArgAction, Args, Parser, Subcommand};

use super::{CliArgsToRaw, ContractCliAction};

/// Parsed arguments of the meta crate CLI.
#[derive(Default, PartialEq, Eq, Debug, Parser)]
#[command(
    version,
    about,
    after_help = "
The MultiversX smart contract Meta crate can be used in two ways:
    A. Import it into a contract's specific meta-crate. 
        There it will receive access to the contract ABI generator. 
        Based on that it is able to build the contract and apply various tools.
        This part also contains the multi-contract config infrastructure.

    B. Use it as a standalone tool.
        It can be used to automatically upgrade contracts from one version to the next.

You are currently using the standalone tool (B).
"
)]
#[command(propagate_version = true)]
pub struct StandaloneCliArgs {
    #[command(subcommand)]
    pub command: Option<StandaloneCliAction>,
}

#[derive(Clone, PartialEq, Eq, Debug, Subcommand)]
pub enum StandaloneCliAction {
    #[command(
        about = "General info about the contract an libraries residing in the targetted directory.."
    )]
    Info(InfoArgs),

    #[command(
        about = "Calls the meta crates for all contracts under given path with the given arguments."
    )]
    All(AllArgs),

    #[command(
        about = "Upgrades a contract to the latest version. Multiple contract crates are allowed."
    )]
    Upgrade(UpgradeArgs),

    #[command(
        name = "local-deps",
        about = "Generates a report on the local depedencies of contract crates. Will explore indirect depdencies too."
    )]
    LocalDeps(LocalDepsArgs),

    #[command(name = "new", about = "Creates a contract by a pre-existing template")]
    Template(TemplateArgs),

    #[command(name = "templates", about = "Lists all pre-existing templates")]
    TemplateList(TemplateListArgs),
    #[command(
        name = "test-gen",
        about = "Generates Rust integration tests based on scenarios provided in the scenarios folder of each contract."
    )]
    TestGen(TestGenArgs),
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct InfoArgs {
    /// Target directory to retrieve info from.
    /// Will be current directory if not specified.
    #[arg(long, verbatim_doc_comment)]
    pub path: Option<String>,

    /// Ignore all directories with these names.
    #[arg(long, verbatim_doc_comment)]
    #[clap(global = true, default_value = "target")]
    pub ignore: Vec<String>,
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct AllArgs {
    #[command(subcommand)]
    pub command: ContractCliAction,

    /// Target directory where to call all contract meta crates.
    /// Will be current directory if not specified.
    #[arg(long, verbatim_doc_comment)]
    #[clap(global = true)]
    pub path: Option<String>,

    /// Ignore all directories with these names.
    #[arg(long, verbatim_doc_comment)]
    #[clap(global = true, default_value = "target")]
    pub ignore: Vec<String>,

    #[arg(
        long = "no-abi-git-version",
        help = "Skips loading the Git version into the ABI",
        action = ArgAction::SetFalse
    )]
    #[clap(global = true)]
    pub load_abi_git_version: bool,

    /// For the meta crates, allows specifying the target directory where the Rust compiler will build the intermediary files.
    /// Sharing the same target directory can speed up building multiple contract crates at once.
    #[arg(long = "target-dir-meta", verbatim_doc_comment)]
    #[clap(global = true)]
    pub target_dir_meta: Option<String>,

    /// Overrides both the --target-dir-meta and the --target-dir-wasm args.
    #[arg(long = "target-dir-all", verbatim_doc_comment)]
    #[clap(global = true)]
    pub target_dir_all: Option<String>,
}

impl AllArgs {
    pub fn target_dir_all_override(&self) -> Self {
        let mut result = self.clone();
        if let Some(target_dir_all) = &self.target_dir_all {
            result.target_dir_meta = Some(target_dir_all.clone());
            match &mut result.command {
                ContractCliAction::Build(build_args) => {
                    build_args.target_dir_wasm = Some(target_dir_all.clone());
                },
                ContractCliAction::BuildDbg(build_args) => {
                    build_args.target_dir_wasm = Some(target_dir_all.clone());
                },
                ContractCliAction::Twiggy(build_args) => {
                    build_args.target_dir_wasm = Some(target_dir_all.clone());
                },
                _ => {},
            }
        }
        result
    }

    pub fn to_cargo_run_args(&self) -> Vec<String> {
        let processed = self.target_dir_all_override();
        let mut raw = vec!["run".to_string()];
        if let Some(target_dir_meta) = &processed.target_dir_meta {
            raw.push("--target-dir".to_string());
            raw.push(target_dir_meta.clone());
        }
        raw.append(&mut processed.command.to_raw());
        if !processed.load_abi_git_version {
            raw.push("--no-abi-git-version".to_string());
        }
        raw
    }
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct UpgradeArgs {
    /// Target directory where to upgrade contracts.
    /// Will be current directory if not specified.
    #[arg(long, verbatim_doc_comment)]
    pub path: Option<String>,

    /// Ignore all directories with these names.
    #[arg(long, verbatim_doc_comment)]
    #[clap(global = true, default_value = "target")]
    pub ignore: Vec<String>,

    /// Overrides the version to upgrade to.
    /// By default it will be the last version out.
    #[arg(long = "to", verbatim_doc_comment)]
    pub override_target_version: Option<String>,
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct LocalDepsArgs {
    /// Target directory where to generate local deps reports.
    /// Will be current directory if not specified.
    #[arg(long, verbatim_doc_comment)]
    pub path: Option<String>,

    /// Ignore all directories with these names.
    #[arg(long, verbatim_doc_comment)]
    #[clap(global = true, default_value = "target")]
    pub ignore: Vec<String>,
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct TemplateArgs {
    /// The new name the contract is to receive.
    /// If missing, the template name will be considered.
    #[arg(long, verbatim_doc_comment)]
    pub name: Option<String>,

    /// The contract template to clone.
    #[arg(long, verbatim_doc_comment)]
    pub template: String,

    /// The framework version on which the contracts should be created.
    #[arg(long, verbatim_doc_comment)]
    pub tag: Option<String>,

    /// Target directory where to create the new contract directory.
    /// Will be current directory if not specified.
    #[arg(long, verbatim_doc_comment)]
    pub path: Option<PathBuf>,
}

impl CliArgsToRaw for TemplateArgs {
    fn to_raw(&self) -> Vec<String> {
        Vec::new()
    }
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct TemplateListArgs {
    /// The framework version referred to.
    #[arg(long = "tag", verbatim_doc_comment)]
    pub tag: Option<String>,
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct TestGenArgs {
    /// Target directory where to generate contract integration tests.
    /// Will be current directory if not specified.
    #[arg(long, verbatim_doc_comment)]
    pub path: Option<String>,

    /// Ignore all directories with these names.
    #[arg(long, verbatim_doc_comment)]
    #[clap(global = true, default_value = "target")]
    pub ignore: Vec<String>,

    /// Creates test files if they don't exist.
    #[arg(long, verbatim_doc_comment)]
    pub create: bool,
}
