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
    #[command(name = "full", about = "Full scenario, whatever that means")]
    Full(FullArgs),
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct FullArgs {
    /// Endpoint name for Vault.
    #[arg(long = "endpoint", verbatim_doc_comment)]
    pub endpoint_name: String,

    /// Endpoint args.
    #[arg(long = "endpoint-args", verbatim_doc_comment)]
    pub endpoint_args: Option<Vec<String>>,
}
