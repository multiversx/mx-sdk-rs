use clap::{Args, Parser, Subcommand};
use multiversx_sc_snippets::imports::{EsdtTokenType, RustBigUint};

/// SysFuncCalls Interact CLI
#[derive(Default, PartialEq, Eq, Debug, Parser)]
#[command(version, about)]
#[command(propagate_version = true)]
pub struct InteractCli {
    #[command(subcommand)]
    pub command: Option<InteractCliCommand>,
}

/// SysFuncCalls Interact CLI Commands
#[derive(Clone, PartialEq, Eq, Debug, Subcommand)]
pub enum InteractCliCommand {
    #[command(name = "add", about = "Add value")]
    Add(AddArgs),
    #[command(name = "issue-token", about = "Issue a token")]
    IssueToken(IssueTokenArgs),
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct AddArgs {
    /// The value to add
    #[arg(short = 'v', long = "value")]
    pub value: u32,

    /// Repeat this number of times
    #[arg(short = 'c', long = "count", default_value = "1")]
    pub count: usize,
}

#[derive(Clone, Debug, PartialEq, Eq, Args)]
pub struct IssueTokenArgs {
    #[arg(short = 'c', long = "cost", default_value = "50000000000000000")]
    pub cost: RustBigUint,
    #[arg(short = 'd', long = "display-name")]
    pub display_name: String,
    #[arg(long = "token-ticker")]
    pub ticker: String,
    #[arg(long = "token-type")]
    pub token_type: u8,
    #[arg(long = "num-decimals")]
    pub num_decimals: usize,
}
