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
}

impl CliArgsToRaw for AllArgs {
    fn to_raw(&self) -> Vec<String> {
        let mut raw = self.command.to_raw();
        if !self.load_abi_git_version {
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
