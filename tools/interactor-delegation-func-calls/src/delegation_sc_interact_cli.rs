use clap::{Args, Parser, Subcommand};

/// DelegationFuncCalls Interact CLI
#[derive(Default, PartialEq, Eq, Debug, Parser)]
#[command(version, about)]
#[command(propagate_version = true)]
pub struct InteractCli {
    #[command(subcommand)]
    pub command: Option<InteractCliCommand>,
}

/// DelegationFuncCalls Interact CLI Commands
#[derive(Clone, PartialEq, Eq, Debug, Subcommand)]
pub enum InteractCliCommand {
    #[command(name = "create", about = "Create new delegation contract")]
    Create(CreateArgs),
    #[command(name = "get-address", about = "Get all contract addresses")]
    GetAllContractAddresses,
    #[command(name = "get-node-states", about = "Get all node states")]
    GetAllNodeStates,
    #[command(
        name = "set-metadata",
        about = "Store information that identifies the staking provider"
    )]
    SetMetadata(MetadataArgs),
    #[command(
        name = "service-fee",
        about = "Store information that identifies the staking provider"
    )]
    ChangeServiceFee(ServiceFeeArgs),
    #[command(
        name = "set-automatic-activation",
        about = "Enable automatic activation"
    )]
    SetAutomaticActivation(SetAutomaticActivationArgs),
    #[command(
        name = "modify-total-delegation-cap",
        about = "Modify the total delegation cap of the delegation contract"
    )]
    ModifyTotalDelegationCap(ModifyTotalDelegationCapArgs),
    #[command(name = "add-node", about = "Add a new node to the delegation contract")]
    AddNode(AddNodeArgs),
    #[command(name = "stake-node", about = "Stake a node in the delegation contract")]
    StakeNode(NodeArgs),
    #[command(
        name = "unstake-node",
        about = "Unstake a node in the delegation contract"
    )]
    UnstakeNode(NodeArgs),
    #[command(
        name = "restake-node",
        about = "Validator nodes that have been unstaked can be restaked"
    )]
    RestakeNode(NodeArgs),
    #[command(
        name = "unbond-node",
        about = "Nodes that have been unstaked can be completely deactivate"
    )]
    UnbondNode(NodeArgs),
    #[command(name = "remove-node", about = "Inactive nodes can be removed")]
    RemoveNode(NodeArgs),
    #[command(
        name = "unjail-node",
        about = "Unjail a node in the delegation contract"
    )]
    UnjailNode(NodeArgs),
    #[command(name = "delegate", about = "Delegate funds")]
    Delegate(DelegateArgs),
    #[command(
        name = "claim-rewards",
        about = "Claim rewards earned by validator nodes"
    )]
    ClaimRewards(ClaimRewardsArgs),
    #[command(
        name = "redelegate-rewards",
        about = "Redelegate rewards earned by validator nodes"
    )]
    RedelegateRewards(ClaimRewardsArgs),
    #[command(
        name = "undelegate",
        about = "Undelegate funds from a delegation contract"
    )]
    UndelegateFunds(DelegateArgs),
    #[command(
        name = "withdraw",
        about = "Withdraw rewards from a delegation contract"
    )]
    Withdraw(ClaimRewardsArgs),
    #[command(
        name = "check-cap",
        about = "Set the check cap on redelegate rewards flag"
    )]
    SetCheckCapOnRedelegateRewards(SetCheckCapOnRedelegateRewardsArgs),
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct CreateArgs {
    /// Total delegation cap in EGLD
    #[arg(short = 't', long = "total")]
    pub total_delegation_cap: u128,

    /// Service fee percentage
    #[arg(short = 'f', long = "fee")]
    pub service_fee: u64,
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct MetadataArgs {
    /// Name of the staking provider
    #[arg(short = 'n', long = "name")]
    pub name: String,

    /// Website of the staking provider
    #[arg(short = 'w', long = "website")]
    pub website: String,

    /// Github identity of the staking provider
    #[arg(short = 'i', long = "identifier")]
    pub identifier: String,
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct ServiceFeeArgs {
    /// Service fee in percentage
    #[arg(short = 'f', long = "name")]
    pub fee: u64,
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct SetAutomaticActivationArgs {
    /// Automatic activation flag
    #[arg(short = 'a', long = "activation")]
    pub automatic_activation: bool,
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct ModifyTotalDelegationCapArgs {
    /// New total delegation cap in EGLD
    #[arg(short = 'c', long = "cap")]
    pub total_delegation_cap: u128,
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct AddNodeArgs {
    /// Public BLS key of the node in hex format
    #[arg(short = 'k', long = "public-key")]
    pub public_key: String,

    /// Verified message signed with the secret BLS key of the node
    #[arg(short = 'm', long = "verified-message")]
    pub verified_message: String,
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct NodeArgs {
    /// Public BLS key of the node in hex format
    #[arg(short = 'k', long = "public-key")]
    pub public_key: String,
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct DelegateArgs {
    /// Funds delegated, minimum 1 EGLD
    #[arg(short = 'e', long = "egld-amount")]
    pub egld: u128,

    /// Account address of funds holder
    #[arg(short = 's', long = "sender")]
    pub from: String,
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct ClaimRewardsArgs {
    /// Account address of existing delegator
    #[arg(short = 's', long = "sender")]
    pub from: String,
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct SetCheckCapOnRedelegateRewardsArgs {
    /// Check cap on redelegate rewards flag
    #[arg(short = 'c', long = "check-cap")]
    pub check_cap_redelegate_rewards: bool,
}
