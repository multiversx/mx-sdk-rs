use clap::{Args, Subcommand};
use multiversx_sc_snippets::imports::RustBigUint;

use super::cli_args_tx::GatewayArgs;
use super::cli_args_tx_system::{BlsKeysArgs, BlsKeysSignaturesArgs, SystemTxCommonArgs};

#[derive(Clone, PartialEq, Eq, Debug, Args)]
pub struct ValidatorCliArgs {
    #[command(subcommand)]
    pub command: ValidatorCliAction,
}

#[derive(Clone, PartialEq, Eq, Debug, Subcommand)]
pub enum ValidatorCliAction {
    #[command(about = "Stakes new validator nodes.")]
    Stake(ValidatorStakeArgs),

    #[command(about = "Adds stake (top-up), without registering new nodes.")]
    StakeTopUp(ValidatorValueArgs),

    #[command(about = "Unstakes nodes, together with their corresponding stake.")]
    Unstake(ValidatorNodesArgs),

    #[command(about = "Unstakes nodes, but leaves their stake as top-up.")]
    UnstakeNodes(ValidatorNodesArgs),

    #[command(about = "Unstakes an amount of EGLD from the top-up.")]
    UnstakeTokens(ValidatorAmountArgs),

    #[command(about = "Unbonds unstaked nodes, together with their corresponding stake.")]
    Unbond(ValidatorNodesArgs),

    #[command(about = "Unbonds unstaked nodes, without unbonding tokens.")]
    UnbondNodes(ValidatorNodesArgs),

    #[command(about = "Unbonds an amount of previously unstaked EGLD.")]
    UnbondTokens(ValidatorAmountArgs),

    #[command(about = "Restakes nodes that were unstaked, but not yet unbonded.")]
    RestakeUnstakedNodes(ValidatorNodesArgs),

    #[command(about = "Unjails nodes, paying the unjail fee.")]
    Unjail(ValidatorUnjailArgs),

    #[command(about = "Changes the address receiving the validator rewards.")]
    ChangeRewardAddress(ValidatorRewardAddressArgs),

    #[command(about = "Removes duplicate BLS keys from the registration data of the sender.")]
    CleanRegisteredData(ValidatorNoArgsTxArgs),

    // --- queries ---
    #[command(
        about = "Queries the total stake of a validator owner (no transaction, no wallet required)."
    )]
    TotalStaked(ValidatorAddressQueryArgs),

    #[command(
        about = "Queries the top-up, total stake and staked BLS keys of a validator owner (no transaction, no wallet required)."
    )]
    TotalStakedTopUp(ValidatorAddressQueryArgs),

    #[command(
        about = "Queries the unstaked tokens of a validator owner (no transaction, no wallet required)."
    )]
    UnstakedTokens(ValidatorAddressQueryArgs),

    #[command(
        about = "Queries the status of each BLS key of a validator owner (no transaction, no wallet required)."
    )]
    BlsKeysStatus(ValidatorAddressQueryArgs),
}

#[derive(Clone, PartialEq, Eq, Debug, Args)]
pub struct ValidatorStakeArgs {
    #[command(flatten)]
    pub keys: BlsKeysSignaturesArgs,

    /// Bech32 address receiving the rewards. Only taken into account at the first stake,
    /// afterwards use change-reward-address. Defaults to the sender.
    #[arg(long)]
    pub reward_address: Option<String>,

    /// Stake, in EGLD atto-units. The total stake must cover the node price for all nodes (2500 EGLD per node on mainnet).
    #[arg(long)]
    pub value: RustBigUint,

    #[command(flatten)]
    pub common: SystemTxCommonArgs,
}

#[derive(Clone, PartialEq, Eq, Debug, Args)]
pub struct ValidatorValueArgs {
    /// Stake, in EGLD atto-units.
    #[arg(long)]
    pub value: RustBigUint,

    #[command(flatten)]
    pub common: SystemTxCommonArgs,
}

#[derive(Clone, PartialEq, Eq, Debug, Args)]
pub struct ValidatorAmountArgs {
    /// Amount, in EGLD atto-units.
    #[arg(long)]
    pub amount: RustBigUint,

    #[command(flatten)]
    pub common: SystemTxCommonArgs,
}

#[derive(Clone, PartialEq, Eq, Debug, Args)]
pub struct ValidatorNodesArgs {
    #[command(flatten)]
    pub keys: BlsKeysArgs,

    #[command(flatten)]
    pub common: SystemTxCommonArgs,
}

#[derive(Clone, PartialEq, Eq, Debug, Args)]
pub struct ValidatorUnjailArgs {
    #[command(flatten)]
    pub keys: BlsKeysArgs,

    /// Unjail fee per node, in EGLD atto-units. Must match the protocol configuration exactly.
    #[arg(long, default_value = "2500000000000000000")]
    pub unjail_price_per_node: RustBigUint,

    #[command(flatten)]
    pub common: SystemTxCommonArgs,
}

#[derive(Clone, PartialEq, Eq, Debug, Args)]
pub struct ValidatorRewardAddressArgs {
    /// Bech32 address receiving the rewards.
    #[arg(long)]
    pub reward_address: String,

    #[command(flatten)]
    pub common: SystemTxCommonArgs,
}

#[derive(Clone, PartialEq, Eq, Debug, Args)]
pub struct ValidatorNoArgsTxArgs {
    #[command(flatten)]
    pub common: SystemTxCommonArgs,
}

#[derive(Clone, PartialEq, Eq, Debug, Args)]
pub struct ValidatorAddressQueryArgs {
    /// Bech32 address of the validator owner.
    pub address: String,

    #[command(flatten)]
    pub gateway: GatewayArgs,
}
