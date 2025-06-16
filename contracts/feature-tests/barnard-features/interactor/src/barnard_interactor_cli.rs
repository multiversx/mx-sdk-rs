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
    #[command(name = "token-type", about = "Token type test")]
    GetESDTTokenType(ESDTTokenTypeArgs),
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct CodeHashArgs {
    #[arg(short = 'a', long = "address")]
    pub address: String,
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct ESDTTokenTypeArgs {
    #[arg(short = 'a', long = "address")]
    pub address: String,

    #[arg(short = 'i', long = "token-id")]
    pub token_id: String,

    #[arg(short = 'n', long = "nonce")]
    pub nonce: u64,
}
