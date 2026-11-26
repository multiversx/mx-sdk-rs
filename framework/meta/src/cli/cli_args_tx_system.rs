use clap::{Args, Subcommand};
use std::path::PathBuf;

use super::cli_args_tx::{GatewayArgs, RelayerArgs, SenderArgs, TxArgs};
use super::cli_args_tx_system_delegation::{DelegationCliArgs, DelegationManagerCliArgs};
use super::cli_args_tx_system_esdt::EsdtCliArgs;
use super::cli_args_tx_system_governance::GovernanceCliArgs;
use super::cli_args_tx_system_validator::ValidatorCliArgs;

#[derive(Clone, PartialEq, Eq, Debug, Args)]
pub struct SystemCliArgs {
    #[command(subcommand)]
    pub command: SystemCliAction,
}

#[derive(Clone, PartialEq, Eq, Debug, Subcommand)]
pub enum SystemCliAction {
    #[command(
        about = "Interacts with the ESDT system smart contract: token issuance, roles and management."
    )]
    Esdt(EsdtCliArgs),

    #[command(
        about = "Interacts with the governance system smart contract: proposals, voting, configuration."
    )]
    Governance(GovernanceCliArgs),

    #[command(
        about = "Interacts with the delegation manager system smart contract: creating staking providers."
    )]
    DelegationManager(DelegationManagerCliArgs),

    #[command(
        about = "Interacts with a delegation contract (staking provider): delegating, rewards, node management."
    )]
    Delegation(DelegationCliArgs),

    #[command(
        about = "Interacts with the validator (staking) system smart contract: staking nodes, unstaking, unbonding."
    )]
    Validator(ValidatorCliArgs),
}

/// Transaction arguments for system SC calls.
///
/// Same as [`TxArgs`], except that the gas limit is optional:
/// each command has its own default.
#[derive(Clone, PartialEq, Eq, Debug, Args)]
pub struct SystemTxArgs {
    /// Gas limit for the transaction. Each command has a sensible default.
    #[arg(long)]
    pub gas_limit: Option<u64>,

    /// Gas price override in smallest EGLD denomination.
    /// If omitted, the minimum gas price is taken from the network config automatically.
    #[arg(long)]
    pub gas_price: Option<u64>,

    /// Explicit nonce to use. If omitted, the current account nonce is fetched automatically.
    #[arg(long)]
    pub nonce: Option<u64>,

    /// If set, the transaction is broadcast to the network.
    /// Without this flag the signed tx JSON is written to --outfile or stdout.
    #[arg(long, default_value = "false")]
    pub send: bool,

    /// Wait for the transaction result. Requires --send.
    #[arg(long, default_value = "false", requires = "send")]
    pub wait_result: bool,

    /// Path to write the signed tx JSON to. Defaults to stdout when --send is not set.
    #[arg(long)]
    pub outfile: Option<PathBuf>,
}

/// Arguments shared by all system SC transactions.
#[derive(Clone, PartialEq, Eq, Debug, Args)]
pub struct SystemTxCommonArgs {
    #[command(flatten)]
    pub gateway: GatewayArgs,

    #[command(flatten)]
    pub sender: SenderArgs,

    #[command(flatten)]
    pub relayer: RelayerArgs,

    #[command(flatten)]
    pub tx: SystemTxArgs,
}

/// Extra gas added to the default gas limit for relayed (v3) transactions.
pub const RELAYED_TX_EXTRA_GAS: u64 = 50_000;

impl SystemTxCommonArgs {
    /// Converts to the generic [`TxArgs`].
    ///
    /// If `--gas-limit` is not given, `default_gas_limit` is used,
    /// increased with the relayed transaction extra cost when a relayer is set.
    pub fn resolve_tx_args(&self, default_gas_limit: u64) -> TxArgs {
        let gas_limit = self.tx.gas_limit.unwrap_or_else(|| {
            if self.relayer.is_set() {
                default_gas_limit + RELAYED_TX_EXTRA_GAS
            } else {
                default_gas_limit
            }
        });

        TxArgs {
            gas_limit,
            gas_price: self.tx.gas_price,
            nonce: self.tx.nonce,
            send: self.tx.send,
            wait_result: self.tx.wait_result,
            outfile: self.tx.outfile.clone(),
        }
    }
}

/// Arguments for commands that need only BLS public keys.
#[derive(Clone, PartialEq, Eq, Debug, Args)]
pub struct BlsKeysArgs {
    /// BLS public keys (hex), comma-separated.
    #[arg(
        long,
        value_delimiter = ',',
        conflicts_with = "validators_pem",
        required_unless_present = "validators_pem"
    )]
    pub bls_keys: Vec<String>,

    /// Validators PEM file, with one or more BLS keys. Only the public keys are used.
    #[arg(long)]
    pub validators_pem: Option<PathBuf>,
}

/// Arguments for commands that register BLS keys, each with a signature of the staker address.
#[derive(Clone, PartialEq, Eq, Debug, Args)]
pub struct BlsKeysSignaturesArgs {
    /// Validators PEM file, with one or more BLS keys. sc-meta produces the signatures:
    /// each BLS key signs the staker address (requires sc-meta built with the `bls` feature).
    #[arg(
        long,
        conflicts_with_all = ["bls_keys", "signatures"],
        required_unless_present = "bls_keys"
    )]
    pub validators_pem: Option<PathBuf>,

    /// BLS public keys (hex), comma-separated. Requires --signatures.
    #[arg(long, value_delimiter = ',', requires = "signatures")]
    pub bls_keys: Vec<String>,

    /// BLS signatures (hex), comma-separated, one for each key in --bls-keys.
    /// Each BLS key must sign the staker address (32 bytes).
    #[arg(long, value_delimiter = ',', requires = "bls_keys")]
    pub signatures: Vec<String>,
}

/// Arguments for system SC queries.
#[derive(Clone, PartialEq, Eq, Debug, Args)]
pub struct SystemQueryArgs {
    #[command(flatten)]
    pub gateway: GatewayArgs,
}
