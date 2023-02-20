use clap::{ArgAction, Args, Parser, Subcommand};

use super::{BuildArgs, BuildDbgArgs, CliArgsToRaw, TwiggyArgs};

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
    BuildDbg(BuildDbgArgs),

    #[command(
        name = "twiggy",
        about = "Builds contract(s) and generate twiggy reports."
    )]
    Twiggy(TwiggyArgs),

    #[command(about = "Clean the Rust project and the output folder.")]
    Clean,

    #[command(about = "Update the Cargo.lock files in all wasm crates.")]
    Update,

    #[command(
        name = "snippets",
        about = "Generates a snippets project, based on the contract ABI."
    )]
    GenerateSnippets(GenerateSnippetsArgs),
}

impl CliArgsToRaw for ContractCliAction {
    fn to_raw(&self) -> Vec<String> {
        let mut raw = Vec::new();
        match self {
            ContractCliAction::Abi => {
                raw.push("abi".to_string());
            },
            ContractCliAction::Build(args) => {
                raw.push("build".to_string());
                raw.append(&mut args.to_raw());
            },
            ContractCliAction::BuildDbg(args) => {
                raw.push("build-dbg".to_string());
                raw.append(&mut args.to_raw());
            },
            ContractCliAction::Twiggy(args) => {
                raw.push("twiggy".to_string());
                raw.append(&mut args.to_raw());
            },
            ContractCliAction::Clean => {
                raw.push("clean".to_string());
            },
            ContractCliAction::Update => {
                raw.push("update".to_string());
            },
            ContractCliAction::GenerateSnippets(args) => {
                raw.push("snippets".to_string());
                raw.append(&mut args.to_raw());
            },
        }
        raw
    }
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct GenerateSnippetsArgs {
    /// Override snippets project if it already exists.
    #[arg(long, verbatim_doc_comment)]
    pub overwrite: bool,
}

impl CliArgsToRaw for GenerateSnippetsArgs {
    fn to_raw(&self) -> Vec<String> {
        let mut raw = Vec::new();
        if self.overwrite {
            raw.push("--overwrite".to_string());
        }
        raw
    }
}
