use clap::{Args, Subcommand};
use std::path::PathBuf;

use super::cli_args_tx::{GatewayArgs, RelayerArgs, SenderArgs, TxArgs};
use super::cli_args_tx_system_esdt::EsdtCliArgs;

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

impl SystemTxArgs {
    /// Converts to the generic [`TxArgs`], with the gas limit resolved.
    pub fn to_tx_args(&self, gas_limit: u64) -> TxArgs {
        TxArgs {
            gas_limit,
            gas_price: self.gas_price,
            nonce: self.nonce,
            send: self.send,
            wait_result: self.wait_result,
            outfile: self.outfile.clone(),
        }
    }
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
