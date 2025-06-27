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
    StakeNode(StakeNodeArgs),
    #[command(name = "delegate", about = "Delegate funds")]
    Delegate,
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
pub struct StakeNodeArgs {
    /// Public BLS key of the node in hex format
    #[arg(short = 'k', long = "public-key")]
    pub public_key: String,
}
