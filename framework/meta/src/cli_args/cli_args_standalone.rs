use clap::{Args, Parser, Subcommand};

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
        about = "Upgrades a contract to the latest version. Multiple contract crates are allowed."
    )]
    Upgrade(UpgradeArgs),
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
