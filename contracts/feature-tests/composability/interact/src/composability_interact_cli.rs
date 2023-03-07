use clap::{Parser, Subcommand, Args};

/// Composability Interact CLI
#[derive(Default, PartialEq, Eq, Debug, Parser)]
#[command(version, about)]
#[command(propagate_version = true)]
pub struct InteractCli {
    #[command(subcommand)]
    pub command: Option<InteractCliCommand>,
}

/// Composability Interact CLI Commands
#[derive(Clone, PartialEq, Eq, Debug, Subcommand)]
pub enum InteractCliCommand {
    #[command(name = "deploy-vault", about = "Deploy Vault contract")]
    DeployVault,
    #[command(name = "deploy-promises", about = "Deploy Promises contract")]
    DeployPromises,
    #[command(name = "deploy-stresser", about = "Deploy Stresser")]
    DeployStresser(StresserArgs),
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct StresserArgs {
    /// The number of contracts deployed used by the stresser
    #[arg(short = 'n', long = "number", verbatim_doc_comment)]
    pub contracts_number: String,
}

