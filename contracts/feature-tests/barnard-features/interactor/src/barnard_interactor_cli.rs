use clap::{Args, Parser, Subcommand};

/// Adder Interact CLI
#[derive(Default, PartialEq, Eq, Debug, Parser)]
#[command(version, about)]
#[command(propagate_version = true)]
pub struct InteractCli {
    #[command(subcommand)]
    pub command: Option<InteractCliCommand>,
}

/// Adder Interact CLI Commands
#[derive(Clone, PartialEq, Eq, Debug, Subcommand)]
pub enum InteractCliCommand {
    #[command(name = "deploy", about = "Deploy contract")]
    Deploy,
    #[command(name = "epoch", about = "Epoch info")]
    EpochInfo,
    #[command(name = "codehash", about = "Code hash test")]
    CodeHash(CodeHashArgs),
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct CodeHashArgs {
    #[arg(short = 'a', long = "address")]
    pub address: String,
}
