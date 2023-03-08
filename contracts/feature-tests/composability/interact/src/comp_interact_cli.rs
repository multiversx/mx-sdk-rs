use clap::{Args, Parser, Subcommand};

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
    // #[command(name = "deploy-vault", about = "Deploy Vault contract")]
    // DeployVault,
    // #[command(name = "deploy-forwarder-raw", about = "Deploy ForwarderRaw contract")]
    // DeployForwarderRaw,
    // #[command(name = "deploy-promises", about = "Deploy Promises contract")]
    // DeployPromises,
    #[command(name = "full", about = "Full scenario, whatever that means")]
    Full(FullArgs),
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct FullArgs {
    /// Call type: Sync, LegacyAsync, TransferExecute.
    #[arg(long = "call-type", verbatim_doc_comment)]
    pub call_type: String,

    /// Endpoint name for Vault.
    #[arg(long = "endpoint", verbatim_doc_comment)]
    pub endpoint_name: String,

    /// Payment token.
    #[arg(long = "payment-token", verbatim_doc_comment)]
    pub payment_token: String,

    /// Payment nonce.
    #[arg(long = "payment-nonce", verbatim_doc_comment)]
    pub payment_nonce: u64,

    /// Payment amount.
    #[arg(long = "payment-amount", verbatim_doc_comment)]
    pub payment_amount: u64,
}
