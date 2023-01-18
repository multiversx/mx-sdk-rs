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

You are currently using the contract tool (A).
"
)]
#[command(propagate_version = true)]
pub struct ContractCliArgs {
    #[command(subcommand)]
    pub command: ContractCliAction,

    #[arg(
        long = "no-abi-git-version",
        help = "Skips loading the Git version into the ABI",
        action = ArgAction::SetFalse
    )]
    #[clap(global = true)]
    pub load_abi_git_version: bool,
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Subcommand)]
pub enum ContractCliAction {
    #[default]
    #[command(name = "abi", about = "Generates the contract ABI and nothing else.")]
    Abi,

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
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct GenerateSnippetsArgs {
    /// Override snippets project if it already exists.
    #[arg(long, verbatim_doc_comment)]
    pub overwrite: bool,
}
