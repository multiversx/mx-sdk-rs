use clap::{Args, Parser, Subcommand};
use multiversx_sc_snippets::{imports::*, multiversx_sc::proxy_imports::*};

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
    #[command(name = "pause-token", about = "Pauses a fungible token")]
    PauseToken(PauseTokenArgs),
    #[command(name = "unpause-token", about = "Unpauses a fungible token")]
    UnpauseToken(PauseTokenArgs),
    #[command(
        name = "freeze-token",
        about = "Freezes a fungible token for an address"
    )]
    FreezeToken(FreezeTokenArgs),
    #[command(
        name = "unfreeze-token",
        about = "Unfreezes a fungible token for an address"
    )]
    UnfreezeToken(FreezeTokenArgs),
    #[command(
        name = "freeze-nft",
        about = "Freezes a non-fungible token for an address"
    )]
    FreezeNFT(FreezeNFTArgs),
    #[command(
        name = "unfreeze-nft",
        about = "Unfreezes a non-fungible token for an address"
    )]
    UnfreezeNFT(FreezeNFTArgs),
    #[command(name = "wipe-token", about = "Wipes a fungible token for an address")]
    WipeToken(WipeTokenArgs),
    #[command(
        name = "wipe-nft",
        about = "Freezes a non-fungible token for an address"
    )]
    WipeNFT(WipeNFTArgs),
    #[command(name = "issue-nft-collection", about = "Create a NFT Collection")]
    IssueNFTCollection(IssueNftCollectionArgs),
    #[command(name = "create-nft", about = "Issue a NFT")]
    CreateNFT(CreateNFTArgs),
    #[command(
        name = "issue-fungible",
        about = "Issues fungible tokens and sends them to your wallet"
    )]
    IssueFungible(IssueFungibleArgs),
    #[command(name = "issue-sft-collection", about = "Issues a SFT")]
    IssueSftCollection(IssueSftArgs),
    #[command(name = "mint-sft", about = "Mints a SFT")]
    MintSft(MintSFTArgs),
    #[command(name = "register-meta-esdt", about = "Registers a meta ESDT")]
    RegisterMetaEsdt(RegisterMetaEsdtArgs),
    #[command(name = "change-sft-meta-esdt", about = "Changes a SFT to a Meta ESDT")]
    ChangeSftMetaEsdt(ChangeSftMetaEsdtArgs),
    #[command(name = "unset-roles", about = "Unsets the roles of a token")]
    UnsetRoles(UnsetRolesArgs),
    #[command(name = "transfer-ownership", about = "Transfers ownership of a token")]
    TransferOwnership(TransferOwnershipArgs),
    #[command(name = "transfer-nft-create-role", about = "Transfers NFT create role")]
    TransferNftCreateRole(TransferNftCreateRoleArgs),
    #[command(name = "control-changes", about = "Controls changes")]
    ControlChanges(ControlChangesArgs),
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
    #[arg(short = 'n', long = "nonce")]
    pub nonce: u64,
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
    #[arg(short = 'n', long = "nonce")]
    pub nonce: u64,
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
pub struct IssueNftCollectionArgs {
    #[arg(short = 'c', long = "cost", default_value = "50000000000000000")]
    pub cost: RustBigUint,
    #[arg(short = 'd', long = "display-name")]
    pub display_name: String,
    #[arg(long = "token-ticker")]
    pub ticker: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Args)]
pub struct IssueSftArgs {
    #[arg(short = 'c', long = "cost", default_value = "50000000000000000")]
    pub cost: RustBigUint,
    #[arg(short = 'd', long = "display-name")]
    pub display_name: String,
    #[arg(long = "token-ticker")]
    pub ticker: String,
}

#[derive(TopEncode, Clone, Debug, PartialEq, Eq)]
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
    pub royalties: u64,
    #[arg(long = "hash")]
    pub hash: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Args)]
pub struct RegisterMetaEsdtArgs {
    #[arg(short = 'c', long = "cost", default_value = "50000000000000000")]
    pub cost: RustBigUint,
    #[arg(short = 'd', long = "display-name")]
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

#[derive(Clone, Debug, PartialEq, Eq, Args)]
pub struct TransferOwnershipArgs {
    #[arg(long = "token-id")]
    pub token_id: String,
    #[arg(long = "new-owner")]
    pub new_owner: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Args)]
pub struct TransferNftCreateRoleArgs {
    #[arg(long = "token-id")]
    pub token_id: String,
    #[arg(long = "old-owner")]
    pub old_owner: String,
    #[arg(long = "new-owner")]
    pub new_owner: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Args)]
pub struct PauseTokenArgs {
    #[arg(long = "token-id", default_value = "")]
    pub token_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Args)]
pub struct FreezeTokenArgs {
    #[arg(long = "token-id", default_value = "")]
    pub token_id: String,
    #[arg(short = 'a', long = "address")]
    pub address: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Args)]
pub struct FreezeNFTArgs {
    #[arg(long = "token-id", default_value = "")]
    pub token_id: String,
    #[arg(long = "nonce")]
    pub nft_nonce: u64,
    #[arg(short = 'a', long = "address")]
    pub address: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Args)]
pub struct WipeTokenArgs {
    #[arg(long = "token-id", default_value = "")]
    pub token_id: String,
    #[arg(short = 'a', long = "address")]
    pub address: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Args)]
pub struct WipeNFTArgs {
    #[arg(long = "token-id", default_value = "")]
    pub token_id: String,
    #[arg(long = "nonce")]
    pub nft_nonce: u64,
    #[arg(short = 'a', long = "address")]
    pub address: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Args)]
pub struct CreateNFTArgs {
    #[arg(short = 't', long = "token-id")]
    pub token_id: String,
    #[arg(short = 'a', long = "amount")]
    pub amount: RustBigUint,
    #[arg(short = 'n', long = "name")]
    pub name: String,
    #[arg(short = 'h', long = "hash")]
    pub hash: String,
    #[arg(short = 'r', long = "royalties")]
    pub royalties: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Args)]
pub struct ControlChangesArgs {
    #[arg(long = "token-id")]
    pub token_id: String,
}
