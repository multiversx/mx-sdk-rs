use clap::{Args, Parser, Subcommand};
use multiversx_sc_snippets::imports::RustBigUint;

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
    #[command(name = "issue-token", about = "Issue a token")]
    IssueToken(IssueTokenArgs),
    #[command(name = "mint", about = "Mints fungible tokens")]
    Mint(MintArgs),
    #[command(name = "set-roles", about = "Set roles")]
    SetRoles(SetRolesArgs),
    #[command(name = "burn", about = "Burns fungible tokens")]
    Burn(BurnArgs),
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

#[derive(Clone, Debug, PartialEq, Eq, Args)]
pub struct MintArgs {
    #[arg(long = "amount")]
    pub amount: RustBigUint,
}

#[derive(Clone, Debug, PartialEq, Eq, Parser)]
pub struct SetRolesArgs {
    #[arg(long = "roles", value_delimiter = ',')]
    pub roles: Vec<u16>,
}

#[derive(Clone, Debug, PartialEq, Eq, Args)]
pub struct BurnArgs {
    #[arg(long = "amount")]
    pub amount: RustBigUint,
}
