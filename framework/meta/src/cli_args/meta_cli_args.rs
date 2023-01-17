use clap::{ArgAction, Args, Parser, Subcommand};

use super::BuildArgs;

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
"
)]
#[command(propagate_version = true)]
pub struct CliArgs {
    #[command(subcommand)]
    pub command: Option<CliAction>,

    #[arg(
        long = "no-abi-git-version",
        help = "Skips loading the Git version into the ABI",
        action = ArgAction::SetFalse
    )]
    #[clap(global = true)]
    pub load_abi_git_version: bool,
}

#[derive(Clone, PartialEq, Eq, Debug, Subcommand)]
pub enum CliAction {
    #[command(
        name = "build",
        about = "Builds contract(s) for deploy on the blockchain."
    )]
    Build(BuildArgs),

    #[command(name = "build-dbg", about = "Builds contract(s) with symbols and WAT.")]
    BuildDbg(BuildArgs),

    #[command(
        name = "twiggy",
        about = "Builds contract(s) and generate twiggy reports."
    )]
    Twiggy(BuildArgs),

    #[command(about = "Clean the Rust project and the output folder.")]
    Clean,

    #[command(
        name = "snippets",
        about = "Generates a snippets project, based on the contract ABI."
    )]
    GenerateSnippets(GenerateSnippetsArgs),

    #[command(
        about = "Upgrades a contract to the latest version. Multiple contract crates are allowed."
    )]
    Upgrade(UpgradeArgs),
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct GenerateSnippetsArgs {
    /// Override snippets project if it already exists.
    #[arg(long, verbatim_doc_comment)]
    pub overwrite: bool,
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct UpgradeArgs {
    /// Target directory where to upgrade contracts. Will be current directory if not specified.
    #[arg(long, verbatim_doc_comment)]
    pub path: Option<String>,

    /// Overrides the version to upgrade to. By default it will be the last version out.
    #[arg(long = "to", verbatim_doc_comment)]
    pub override_target_version: Option<String>,
}
