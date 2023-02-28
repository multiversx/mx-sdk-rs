use clap::{Parser, Subcommand};

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
#[allow(clippy::enum_variant_names)]
pub enum InteractCliCommand {
    #[command(name = "deploy-vault", about = "Deploy Vault contract")]
    DeployVault,
    #[command(name = "deploy-forwarder-raw", about = "Deploy ForwarderRaw contract")]
    DeployForwarderRaw,
    #[command(name = "deploy-promises", about = "Deploy Promises contract")]
    DeployPromises,
}
