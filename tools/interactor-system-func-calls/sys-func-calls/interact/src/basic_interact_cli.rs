use clap::{Args, Parser, Subcommand};
use multiversx_sc_snippets::{imports::RustBigUint, multiversx_sc::proxy_imports::*};

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
    #[command(name = "issue-token", about = "Issues a token")]
    IssueToken(IssueTokenArgs),
    #[command(name = "mint", about = "Mints fungible tokens")]
    Mint(MintArgs),
    #[command(name = "set-roles", about = "Sets roles")]
    SetRoles(SetRolesArgs),
    #[command(name = "burn", about = "Burns fungible tokens")]
    Burn(BurnArgs),
    #[command(
        name = "issue-fungible",
        about = "Issues fungible tokens and sends them to your wallet"
    )]
    IssueFungible(IssueFungibleArgs),
    #[command(name = "issue-sft", about = "Issues a SFT")]
    IssueSft(IssueSftArgs),
    #[command(name = "mint-sft", about = "Mints a SFT")]
    MintSft(MintSFTArgs),
    #[command(name = "register-meta-esdt", about = "Registers a meta ESDT")]
    RegisterMetaEsdt(RegisterMetaEsdtArgs),
    #[command(name = "change-sft-meta-esdt", about = "Changes a SFT to a Meta ESDT")]
    ChangeSftMetaEsdt(ChangeSftMetaEsdtArgs),
    #[command(name = "unset-roles", about = "Unsets the roles of a token")]
    UnsetRoles(UnsetRolesArgs),
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
    #[arg(long = "token-id")]
    pub token_id: String,
    #[arg(long = "amount")]
    pub amount: RustBigUint,
}

#[derive(Clone, Debug, PartialEq, Eq, Parser)]
pub struct SetRolesArgs {
    #[arg(long = "token-id")]
    pub token_id: String,
    #[arg(long = "roles", value_delimiter = ',')]
    pub roles: Vec<u16>,
}

#[derive(Clone, Debug, PartialEq, Eq, Args)]
pub struct BurnArgs {
    #[arg(long = "token-id")]
    pub token_id: String,
    #[arg(long = "amount")]
    pub amount: RustBigUint,
}

#[derive(Clone, Debug, PartialEq, Eq, Args)]
pub struct IssueFungibleArgs {
    #[arg(short = 'c', long = "cost", default_value = "50000000000000000")]
    pub cost: RustBigUint,
    #[arg(long = "display-name")]
    pub display_name: String,
    #[arg(long = "token-ticker")]
    pub ticker: String,
    #[arg(long = "num-decimals")]
    pub num_decimals: usize,
    #[arg(short = 's', long = "supply")]
    pub supply: RustBigUint,
}

#[derive(Clone, Debug, PartialEq, Eq, Args)]
pub struct IssueSftArgs {
    #[arg(short = 'c', long = "cost", default_value = "50000000000000000")]
    pub cost: RustBigUint,
    #[arg(long = "display-name")]
    pub display_name: String,
    #[arg(long = "token-ticker")]
    pub ticker: String,
}

#[derive(TopEncode, TopDecode, Clone, Debug, PartialEq, Eq)]
pub struct NftDummyAttributes {
    pub creation_epoch: u64,
    pub cool_factor: u8,
}

#[derive(Clone, Debug, PartialEq, Eq, Args)]
pub struct MintSFTArgs {
    #[arg(long = "token-id")]
    pub token_id: String,
    #[arg(short = 'a', long = "amount")]
    pub amount: RustBigUint,
    #[arg(short = 'n', long = "name")]
    pub name: String,
    #[arg(short = 'r', long = "royalties")]
    pub royalties: RustBigUint,
    #[arg(long = "hash")]
    pub hash: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Args)]
pub struct RegisterMetaEsdtArgs {
    #[arg(short = 'c', long = "cost", default_value = "50000000000000000")]
    pub cost: RustBigUint,
    #[arg(long = "display-name")]
    pub display_name: String,
    #[arg(long = "token-ticker")]
    pub ticker: String,
    #[arg(long = "num-decimals")]
    pub num_decimals: usize,
}

#[derive(Clone, Debug, PartialEq, Eq, Args)]
pub struct ChangeSftMetaEsdtArgs {
    #[arg(long = "token-id")]
    pub token_id: String,
    #[arg(long = "num-decimals")]
    pub num_decimals: usize,
}

#[derive(Clone, Debug, PartialEq, Eq, Parser)]
pub struct UnsetRolesArgs {
    #[arg(short = 'a', long = "address")]
    pub address: String,
    #[arg(long = "token-id")]
    pub token_id: String,
    #[arg(long = "roles", value_delimiter = ',')]
    pub roles: Vec<u16>,
}
