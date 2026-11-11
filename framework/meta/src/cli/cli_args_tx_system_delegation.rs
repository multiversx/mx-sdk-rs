use clap::{ArgAction, Args, Subcommand};
use multiversx_sc_snippets::imports::RustBigUint;

use super::cli_args_tx::GatewayArgs;
use super::cli_args_tx_system::{
    BlsKeysArgs, BlsKeysSignaturesArgs, SystemQueryArgs, SystemTxCommonArgs,
};

// ---------------------------------------------------------------------------
// Delegation manager
// ---------------------------------------------------------------------------

#[derive(Clone, PartialEq, Eq, Debug, Args)]
pub struct DelegationManagerCliArgs {
    #[command(subcommand)]
    pub command: DelegationManagerCliAction,
}

#[derive(Clone, PartialEq, Eq, Debug, Subcommand)]
pub enum DelegationManagerCliAction {
    #[command(about = "Creates a new delegation contract (staking provider).")]
    CreateNewDelegationContract(DelegationManagerCreateArgs),

    #[command(about = "Converts the sender's validator into a new delegation contract.")]
    MakeNewContractFromValidatorData(DelegationManagerFromValidatorArgs),

    #[command(
        about = "Merges the sender's validator into a delegation contract owned by the same address."
    )]
    MergeValidatorToDelegationSameOwner(DelegationManagerMergeArgs),

    #[command(
        about = "Merges the sender's validator into a delegation contract that whitelisted the sender."
    )]
    MergeValidatorToDelegationWithWhitelist(DelegationManagerMergeArgs),

    #[command(about = "Claims rewards from several delegation contracts at once.")]
    ClaimMulti(DelegationManagerMultiArgs),

    #[command(about = "Redelegates rewards in several delegation contracts at once.")]
    RedelegateMulti(DelegationManagerMultiArgs),

    // --- queries ---
    #[command(
        about = "Queries the delegation manager configuration, including the minimum deposit (no transaction, no wallet required)."
    )]
    ContractConfig(SystemQueryArgs),

    #[command(
        about = "Queries the addresses of all delegation contracts (no transaction, no wallet required)."
    )]
    ContractAddresses(SystemQueryArgs),
}

#[derive(Clone, PartialEq, Eq, Debug, Args)]
pub struct DelegationManagerCreateArgs {
    /// Total delegation cap, in EGLD atto-units. 0 means uncapped.
    #[arg(long, default_value = "0")]
    pub total_delegation_cap: RustBigUint,

    /// Service fee, in basis points (10000 = 100%).
    #[arg(long)]
    pub service_fee: u64,

    /// Initial stake of the owner, in EGLD atto-units. Must be at least the minimum deposit.
    /// If omitted, the minimum deposit is read from the delegation manager configuration.
    #[arg(long)]
    pub value: Option<RustBigUint>,

    #[command(flatten)]
    pub common: SystemTxCommonArgs,
}

#[derive(Clone, PartialEq, Eq, Debug, Args)]
pub struct DelegationManagerFromValidatorArgs {
    /// Maximum delegation cap, in EGLD atto-units. 0 means uncapped.
    #[arg(long, default_value = "0")]
    pub max_delegation_cap: RustBigUint,

    /// Service fee, in basis points (10000 = 100%).
    #[arg(long)]
    pub service_fee: u64,

    #[command(flatten)]
    pub common: SystemTxCommonArgs,
}

#[derive(Clone, PartialEq, Eq, Debug, Args)]
pub struct DelegationManagerMergeArgs {
    /// Bech32 address of the delegation contract.
    #[arg(long)]
    pub delegation_contract: String,

    #[command(flatten)]
    pub common: SystemTxCommonArgs,
}

#[derive(Clone, PartialEq, Eq, Debug, Args)]
pub struct DelegationManagerMultiArgs {
    /// Bech32 addresses of the delegation contracts.
    #[arg(long, num_args = 1.., required = true)]
    pub delegation_contracts: Vec<String>,

    #[command(flatten)]
    pub common: SystemTxCommonArgs,
}

// ---------------------------------------------------------------------------
// Delegation
// ---------------------------------------------------------------------------

#[derive(Clone, PartialEq, Eq, Debug, Args)]
pub struct DelegationCliArgs {
    /// Bech32 address of the delegation contract (staking provider).
    pub contract: String,

    #[command(subcommand)]
    pub command: DelegationCliAction,
}

#[derive(Clone, PartialEq, Eq, Debug, Subcommand)]
pub enum DelegationCliAction {
    // --- owner ---
    #[command(about = "Sets the staking provider metadata. Owner only.")]
    SetMetadata(DelegationMetadataArgs),

    #[command(about = "Changes the service fee. Owner only.")]
    ChangeServiceFee(DelegationServiceFeeArgs),

    #[command(about = "Changes the total delegation cap. Owner only.")]
    ModifyTotalDelegationCap(DelegationCapArgs),

    #[command(about = "Enables or disables the automatic activation of nodes. Owner only.")]
    SetAutomaticActivation(DelegationEnabledArgs),

    #[command(
        about = "Enables or disables the delegation cap check when redelegating rewards. Owner only."
    )]
    SetCheckCapOnRedelegateRewards(DelegationEnabledArgs),

    #[command(about = "Adds nodes (BLS keys) to the contract. Owner only.")]
    AddNodes(DelegationAddNodesArgs),

    #[command(about = "Removes inactive nodes from the contract. Owner only.")]
    RemoveNodes(DelegationNodesArgs),

    #[command(about = "Stakes nodes. Owner only.")]
    StakeNodes(DelegationNodesArgs),

    #[command(about = "Unstakes nodes. Owner only.")]
    UnstakeNodes(DelegationNodesArgs),

    #[command(about = "Unbonds unstaked nodes. Owner only.")]
    UnbondNodes(DelegationNodesArgs),

    #[command(about = "Restakes nodes that were unstaked, but not yet unbonded. Owner only.")]
    RestakeUnstakedNodes(DelegationNodesArgs),

    #[command(about = "Unjails nodes, paying the unjail fee. Owner only.")]
    UnjailNodes(DelegationUnjailNodesArgs),

    #[command(
        about = "Whitelists a validator owner, allowing it to merge its validator into this contract. Owner only."
    )]
    WhitelistForMerge(DelegationAddressArgs),

    #[command(about = "Removes the address whitelisted for merge. Owner only.")]
    DeleteWhitelistForMerge(DelegationNoArgsTxArgs),

    #[command(about = "Transfers the ownership of the contract. Owner only.")]
    ChangeOwner(DelegationAddressArgs),

    #[command(
        about = "Synchronizes the owner stored in the contract with the owner of the account."
    )]
    SynchronizeOwner(DelegationNoArgsTxArgs),

    // --- delegator ---
    #[command(about = "Delegates EGLD to the contract.")]
    Delegate(DelegationValueArgs),

    #[command(about = "Claims the delegation rewards.")]
    ClaimRewards(DelegationNoArgsTxArgs),

    #[command(about = "Redelegates the delegation rewards.")]
    RedelegateRewards(DelegationNoArgsTxArgs),

    #[command(about = "Undelegates EGLD from the contract.")]
    Undelegate(DelegationAmountArgs),

    #[command(about = "Withdraws undelegated EGLD, after the unbonding period.")]
    Withdraw(DelegationNoArgsTxArgs),

    // --- queries ---
    #[command(about = "Queries the contract configuration (no transaction, no wallet required).")]
    ContractConfig(SystemQueryArgs),

    #[command(
        about = "Queries the staking provider metadata (no transaction, no wallet required)."
    )]
    Metadata(SystemQueryArgs),

    #[command(about = "Queries the BLS keys, by state (no transaction, no wallet required).")]
    NodeStates(SystemQueryArgs),

    #[command(
        about = "Queries the total stake, number of delegators and nodes (no transaction, no wallet required)."
    )]
    Totals(SystemQueryArgs),

    #[command(
        about = "Queries the funds and rewards of a delegator (no transaction, no wallet required)."
    )]
    Delegator(DelegationDelegatorQueryArgs),

    #[command(
        about = "Queries the rewards distributed in an epoch (no transaction, no wallet required)."
    )]
    RewardData(DelegationRewardDataQueryArgs),

    #[command(
        about = "Queries the address whitelisted for merge (no transaction, no wallet required)."
    )]
    WhitelistedForMerge(SystemQueryArgs),
}

#[derive(Clone, PartialEq, Eq, Debug, Args)]
pub struct DelegationNoArgsTxArgs {
    #[command(flatten)]
    pub common: SystemTxCommonArgs,
}

#[derive(Clone, PartialEq, Eq, Debug, Args)]
pub struct DelegationMetadataArgs {
    /// Staking provider name.
    #[arg(long)]
    pub name: String,

    /// Staking provider website.
    #[arg(long)]
    pub website: String,

    /// Staking provider identifier (keybase/GitHub identity).
    #[arg(long)]
    pub identifier: String,

    #[command(flatten)]
    pub common: SystemTxCommonArgs,
}

#[derive(Clone, PartialEq, Eq, Debug, Args)]
pub struct DelegationServiceFeeArgs {
    /// Service fee, in basis points (10000 = 100%).
    #[arg(long)]
    pub service_fee: u64,

    #[command(flatten)]
    pub common: SystemTxCommonArgs,
}

#[derive(Clone, PartialEq, Eq, Debug, Args)]
pub struct DelegationCapArgs {
    /// Total delegation cap, in EGLD atto-units. 0 means uncapped.
    #[arg(long)]
    pub total_delegation_cap: RustBigUint,

    #[command(flatten)]
    pub common: SystemTxCommonArgs,
}

#[derive(Clone, PartialEq, Eq, Debug, Args)]
pub struct DelegationEnabledArgs {
    /// true or false.
    #[arg(long, action = ArgAction::Set)]
    pub enabled: bool,

    #[command(flatten)]
    pub common: SystemTxCommonArgs,
}

#[derive(Clone, PartialEq, Eq, Debug, Args)]
pub struct DelegationAddNodesArgs {
    #[command(flatten)]
    pub keys: BlsKeysSignaturesArgs,

    #[command(flatten)]
    pub common: SystemTxCommonArgs,
}

#[derive(Clone, PartialEq, Eq, Debug, Args)]
pub struct DelegationNodesArgs {
    #[command(flatten)]
    pub keys: BlsKeysArgs,

    #[command(flatten)]
    pub common: SystemTxCommonArgs,
}

#[derive(Clone, PartialEq, Eq, Debug, Args)]
pub struct DelegationUnjailNodesArgs {
    #[command(flatten)]
    pub keys: BlsKeysArgs,

    /// Unjail fee per node, in EGLD atto-units. Must match the protocol configuration exactly.
    #[arg(long, default_value = "2500000000000000000")]
    pub unjail_price_per_node: RustBigUint,

    #[command(flatten)]
    pub common: SystemTxCommonArgs,
}

#[derive(Clone, PartialEq, Eq, Debug, Args)]
pub struct DelegationAddressArgs {
    /// Bech32 address.
    #[arg(long)]
    pub address: String,

    #[command(flatten)]
    pub common: SystemTxCommonArgs,
}

#[derive(Clone, PartialEq, Eq, Debug, Args)]
pub struct DelegationValueArgs {
    /// Amount to delegate, in EGLD atto-units. Must be at least the minimum delegation amount (1 EGLD on mainnet).
    #[arg(long)]
    pub value: RustBigUint,

    #[command(flatten)]
    pub common: SystemTxCommonArgs,
}

#[derive(Clone, PartialEq, Eq, Debug, Args)]
pub struct DelegationAmountArgs {
    /// Amount to undelegate, in EGLD atto-units.
    #[arg(long)]
    pub amount: RustBigUint,

    #[command(flatten)]
    pub common: SystemTxCommonArgs,
}

#[derive(Clone, PartialEq, Eq, Debug, Args)]
pub struct DelegationDelegatorQueryArgs {
    /// Bech32 address of the delegator.
    pub address: String,

    #[command(flatten)]
    pub gateway: GatewayArgs,
}

#[derive(Clone, PartialEq, Eq, Debug, Args)]
pub struct DelegationRewardDataQueryArgs {
    /// The epoch.
    pub epoch: u64,

    #[command(flatten)]
    pub gateway: GatewayArgs,
}
