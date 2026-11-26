use clap::{ArgAction, Args, Subcommand, ValueEnum};
use multiversx_sc_snippets::imports::RustBigUint;

use super::cli_args_tx::GatewayArgs;
use super::cli_args_tx_system::SystemTxCommonArgs;

#[derive(Clone, PartialEq, Eq, Debug, Args)]
pub struct EsdtCliArgs {
    #[command(subcommand)]
    pub command: EsdtCliAction,
}

#[derive(Clone, PartialEq, Eq, Debug, Subcommand)]
pub enum EsdtCliAction {
    // --- issue ---
    #[command(about = "Issues a new fungible token. The initial supply is sent to the sender.")]
    IssueFungible(EsdtIssueFungibleArgs),

    #[command(about = "Issues a new non-fungible (NFT) collection.")]
    IssueNonFungible(EsdtIssueCollectionArgs),

    #[command(about = "Issues a new semi-fungible (SFT) collection.")]
    IssueSemiFungible(EsdtIssueCollectionArgs),

    #[command(about = "Registers a new Meta-ESDT collection.")]
    RegisterMetaEsdt(EsdtRegisterMetaArgs),

    #[command(about = "Registers a new token and grants all roles to the sender.")]
    RegisterAndSetAllRoles(EsdtRegisterAndSetAllRolesArgs),

    #[command(about = "Registers a new dynamic NFT, SFT or Meta-ESDT collection.")]
    RegisterDynamic(EsdtRegisterDynamicArgs),

    #[command(
        about = "Registers a new dynamic NFT, SFT or Meta-ESDT collection and grants all roles to the sender."
    )]
    RegisterAndSetAllRolesDynamic(EsdtRegisterDynamicArgs),

    // --- roles ---
    #[command(about = "Grants special roles to an address.")]
    SetSpecialRole(EsdtRolesArgs),

    #[command(about = "Revokes special roles from an address.")]
    UnsetSpecialRole(EsdtRolesArgs),

    #[command(about = "Transfers the NFT create role from one address to another.")]
    TransferNftCreateRole(EsdtTransferNftCreateRoleArgs),

    #[command(about = "Allows any address to burn the token (ESDTRoleBurnForAll).")]
    SetBurnRoleGlobally(EsdtTokenTxArgs),

    #[command(about = "Reverts set-burn-role-globally.")]
    UnsetBurnRoleGlobally(EsdtTokenTxArgs),

    #[command(about = "Re-sends the addresses holding ESDTTransferRole to all shards.")]
    SendAllTransferRoleAddresses(EsdtTokenTxArgs),

    // --- management ---
    #[command(about = "Pauses all transfers of a token.")]
    Pause(EsdtTokenTxArgs),

    #[command(about = "Resumes the transfers of a paused token.")]
    Unpause(EsdtTokenTxArgs),

    #[command(about = "Freezes a fungible token for an address.")]
    Freeze(EsdtTokenAddressArgs),

    #[command(about = "Unfreezes a fungible token for an address.")]
    Unfreeze(EsdtTokenAddressArgs),

    #[command(about = "Wipes a fungible token from a frozen address.")]
    Wipe(EsdtTokenAddressArgs),

    #[command(about = "Freezes a single NFT/SFT/Meta-ESDT for an address.")]
    FreezeNft(EsdtNftAddressArgs),

    #[command(about = "Unfreezes a single NFT/SFT/Meta-ESDT for an address.")]
    UnfreezeNft(EsdtNftAddressArgs),

    #[command(about = "Wipes a single NFT/SFT/Meta-ESDT from a frozen address.")]
    WipeNft(EsdtNftAddressArgs),

    #[command(about = "Transfers the ownership (management rights) of a token.")]
    TransferOwnership(EsdtTransferOwnershipArgs),

    #[command(about = "Changes token properties (e.g. --can-freeze false).")]
    ControlChanges(EsdtControlChangesArgs),

    #[command(about = "Permanently stops the creation of new NFTs/SFTs in a collection.")]
    StopNftCreate(EsdtTokenTxArgs),

    #[command(about = "Converts an SFT collection to a Meta-ESDT collection.")]
    ChangeSftToMetaEsdt(EsdtChangeSftToMetaArgs),

    #[command(about = "Converts a collection to dynamic.")]
    ChangeToDynamic(EsdtTokenTxArgs),

    #[command(about = "Updates a token to the latest token type version.")]
    UpdateTokenId(EsdtTokenTxArgs),

    #[command(about = "Allows the NFT create role to be held by addresses in multiple shards.")]
    ChangeToMultiShardCreate(EsdtTokenTxArgs),

    // --- queries ---
    #[command(about = "Queries the properties of a token (no transaction, no wallet required).")]
    TokenProperties(EsdtTokenQueryArgs),

    #[command(
        about = "Queries the special roles of a token, per address (no transaction, no wallet required)."
    )]
    SpecialRoles(EsdtTokenQueryArgs),

    #[command(about = "Queries the ESDT system SC configuration, including the issue cost.")]
    ContractConfig(EsdtQueryArgs),
}

/// Token types accepted by `register-and-set-all-roles`.
#[derive(Clone, Copy, PartialEq, Eq, Debug, ValueEnum)]
pub enum EsdtTokenTypeArg {
    #[value(name = "FNG")]
    Fungible,
    #[value(name = "NFT")]
    NonFungible,
    #[value(name = "SFT")]
    SemiFungible,
    #[value(name = "META")]
    Meta,
}

/// Token types accepted by the dynamic register commands.
#[derive(Clone, Copy, PartialEq, Eq, Debug, ValueEnum)]
pub enum EsdtDynamicTokenTypeArg {
    #[value(name = "NFT")]
    NonFungible,
    #[value(name = "SFT")]
    SemiFungible,
    #[value(name = "META")]
    Meta,
}

/// Name and ticker of a new token.
#[derive(Clone, PartialEq, Eq, Debug, Args)]
pub struct EsdtTokenNameArgs {
    /// Token display name, alphanumeric. Length limits: see `contract-config`.
    #[arg(long)]
    pub name: String,

    /// Token ticker, uppercase alphanumeric (3-10 characters).
    #[arg(long)]
    pub ticker: String,
}

/// The issue cost paid to the ESDT system SC.
#[derive(Clone, PartialEq, Eq, Debug, Args)]
pub struct EsdtIssueCostArgs {
    /// Issue cost, in EGLD atto-units.
    /// If omitted, it is read from the ESDT system SC configuration (0.05 EGLD on mainnet).
    #[arg(long = "issue-cost")]
    pub issue_cost: Option<RustBigUint>,
}

/// Properties common to all newly issued tokens. All default to true.
#[derive(Clone, PartialEq, Eq, Debug, Args)]
pub struct EsdtIssuePropertiesArgs {
    /// The owner can freeze/unfreeze the token for an address.
    #[arg(long, default_value_t = true, action = ArgAction::Set)]
    pub can_freeze: bool,

    /// The owner can wipe the token from a frozen address.
    #[arg(long, default_value_t = true, action = ArgAction::Set)]
    pub can_wipe: bool,

    /// The owner can pause/unpause all transfers of the token.
    #[arg(long, default_value_t = true, action = ArgAction::Set)]
    pub can_pause: bool,

    /// The ownership of the token can be transferred.
    #[arg(long, default_value_t = true, action = ArgAction::Set)]
    pub can_change_owner: bool,

    /// The owner can change the token properties.
    #[arg(long, default_value_t = true, action = ArgAction::Set)]
    pub can_upgrade: bool,

    /// The owner can grant special roles.
    #[arg(long, default_value_t = true, action = ArgAction::Set)]
    pub can_add_special_roles: bool,
}

#[derive(Clone, PartialEq, Eq, Debug, Args)]
pub struct EsdtIssueFungibleArgs {
    #[command(flatten)]
    pub token: EsdtTokenNameArgs,

    /// Initial supply, in the smallest denomination (i.e. including decimals).
    #[arg(long)]
    pub initial_supply: RustBigUint,

    /// Number of decimals (0-18).
    #[arg(long)]
    pub num_decimals: usize,

    #[command(flatten)]
    pub properties: EsdtIssuePropertiesArgs,

    #[command(flatten)]
    pub cost: EsdtIssueCostArgs,

    #[command(flatten)]
    pub common: SystemTxCommonArgs,
}

#[derive(Clone, PartialEq, Eq, Debug, Args)]
pub struct EsdtIssueCollectionArgs {
    #[command(flatten)]
    pub token: EsdtTokenNameArgs,

    #[command(flatten)]
    pub properties: EsdtIssuePropertiesArgs,

    /// The NFT create role can be transferred to another address.
    #[arg(long, default_value_t = true, action = ArgAction::Set)]
    pub can_transfer_nft_create_role: bool,

    #[command(flatten)]
    pub cost: EsdtIssueCostArgs,

    #[command(flatten)]
    pub common: SystemTxCommonArgs,
}

#[derive(Clone, PartialEq, Eq, Debug, Args)]
pub struct EsdtRegisterMetaArgs {
    #[command(flatten)]
    pub token: EsdtTokenNameArgs,

    /// Number of decimals (0-18).
    #[arg(long)]
    pub num_decimals: usize,

    #[command(flatten)]
    pub properties: EsdtIssuePropertiesArgs,

    /// The NFT create role can be transferred to another address.
    #[arg(long, default_value_t = true, action = ArgAction::Set)]
    pub can_transfer_nft_create_role: bool,

    #[command(flatten)]
    pub cost: EsdtIssueCostArgs,

    #[command(flatten)]
    pub common: SystemTxCommonArgs,
}

#[derive(Clone, PartialEq, Eq, Debug, Args)]
pub struct EsdtRegisterAndSetAllRolesArgs {
    #[command(flatten)]
    pub token: EsdtTokenNameArgs,

    /// Token type.
    #[arg(long, value_enum)]
    pub token_type: EsdtTokenTypeArg,

    /// Number of decimals (0-18). Only relevant for FNG and META.
    #[arg(long, default_value_t = 0)]
    pub num_decimals: usize,

    #[command(flatten)]
    pub cost: EsdtIssueCostArgs,

    #[command(flatten)]
    pub common: SystemTxCommonArgs,
}

#[derive(Clone, PartialEq, Eq, Debug, Args)]
pub struct EsdtRegisterDynamicArgs {
    #[command(flatten)]
    pub token: EsdtTokenNameArgs,

    /// Token type.
    #[arg(long, value_enum)]
    pub token_type: EsdtDynamicTokenTypeArg,

    /// Number of decimals (0-18). Only relevant for META.
    #[arg(long, default_value_t = 0)]
    pub num_decimals: usize,

    #[command(flatten)]
    pub cost: EsdtIssueCostArgs,

    #[command(flatten)]
    pub common: SystemTxCommonArgs,
}

#[derive(Clone, PartialEq, Eq, Debug, Args)]
pub struct EsdtTokenTxArgs {
    /// Token identifier (e.g. TOKEN-abcdef).
    pub token: String,

    #[command(flatten)]
    pub common: SystemTxCommonArgs,
}

#[derive(Clone, PartialEq, Eq, Debug, Args)]
pub struct EsdtTokenAddressArgs {
    /// Token identifier (e.g. TOKEN-abcdef).
    pub token: String,

    /// Bech32 address of the account.
    #[arg(long)]
    pub address: String,

    #[command(flatten)]
    pub common: SystemTxCommonArgs,
}

#[derive(Clone, PartialEq, Eq, Debug, Args)]
pub struct EsdtNftAddressArgs {
    /// Collection identifier (e.g. NFT-abcdef).
    pub token: String,

    /// Nonce of the NFT/SFT/Meta-ESDT.
    #[arg(long = "token-nonce")]
    pub token_nonce: u64,

    /// Bech32 address of the account.
    #[arg(long)]
    pub address: String,

    #[command(flatten)]
    pub common: SystemTxCommonArgs,
}

#[derive(Clone, PartialEq, Eq, Debug, Args)]
pub struct EsdtRolesArgs {
    /// Token identifier (e.g. TOKEN-abcdef).
    pub token: String,

    /// Bech32 address of the account receiving (or losing) the roles.
    #[arg(long)]
    pub address: String,

    /// Comma-separated list of roles.
    /// Accepts the protocol names (e.g. ESDTRoleLocalMint, ESDTRoleNFTCreate, ESDTTransferRole)
    /// or the short names (e.g. Mint, Burn, NftCreate, NftAddQuantity, NftBurn, NftUpdateAttributes,
    /// NftAddUri, NftRecreate, NftUpdate, ModifyCreator, ModifyRoyalties, SetNewUri, Transfer).
    #[arg(long, value_delimiter = ',', required = true)]
    pub roles: Vec<String>,

    #[command(flatten)]
    pub common: SystemTxCommonArgs,
}

#[derive(Clone, PartialEq, Eq, Debug, Args)]
pub struct EsdtTransferNftCreateRoleArgs {
    /// Collection identifier (e.g. NFT-abcdef).
    pub token: String,

    /// Bech32 address currently holding the NFT create role.
    #[arg(long)]
    pub from_address: String,

    /// Bech32 address receiving the NFT create role.
    #[arg(long)]
    pub to_address: String,

    #[command(flatten)]
    pub common: SystemTxCommonArgs,
}

#[derive(Clone, PartialEq, Eq, Debug, Args)]
pub struct EsdtTransferOwnershipArgs {
    /// Token identifier (e.g. TOKEN-abcdef).
    pub token: String,

    /// Bech32 address of the new owner.
    #[arg(long)]
    pub new_owner: String,

    #[command(flatten)]
    pub common: SystemTxCommonArgs,
}

#[derive(Clone, PartialEq, Eq, Debug, Args)]
pub struct EsdtChangeSftToMetaArgs {
    /// SFT collection identifier (e.g. SFT-abcdef).
    pub token: String,

    /// Number of decimals (0-18).
    #[arg(long)]
    pub num_decimals: usize,

    #[command(flatten)]
    pub common: SystemTxCommonArgs,
}

/// Only the properties that are given are changed.
#[derive(Clone, PartialEq, Eq, Debug, Args)]
pub struct EsdtControlChangesArgs {
    /// Token identifier (e.g. TOKEN-abcdef).
    pub token: String,

    #[arg(long)]
    pub can_freeze: Option<bool>,

    #[arg(long)]
    pub can_wipe: Option<bool>,

    #[arg(long)]
    pub can_pause: Option<bool>,

    #[arg(long)]
    pub can_transfer_nft_create_role: Option<bool>,

    /// Only accepted for fungible tokens.
    #[arg(long)]
    pub can_mint: Option<bool>,

    /// Only accepted for fungible tokens.
    #[arg(long)]
    pub can_burn: Option<bool>,

    #[arg(long)]
    pub can_change_owner: Option<bool>,

    #[arg(long)]
    pub can_upgrade: Option<bool>,

    #[arg(long)]
    pub can_add_special_roles: Option<bool>,

    #[command(flatten)]
    pub common: SystemTxCommonArgs,
}

#[derive(Clone, PartialEq, Eq, Debug, Args)]
pub struct EsdtTokenQueryArgs {
    /// Token identifier (e.g. TOKEN-abcdef).
    pub token: String,

    #[command(flatten)]
    pub gateway: GatewayArgs,
}

#[derive(Clone, PartialEq, Eq, Debug, Args)]
pub struct EsdtQueryArgs {
    #[command(flatten)]
    pub gateway: GatewayArgs,
}
