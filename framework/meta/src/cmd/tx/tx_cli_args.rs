use clap::{Args, Subcommand};
use std::path::PathBuf;

#[derive(Clone, PartialEq, Eq, Debug, Args)]
pub struct TxCliArgs {
    #[command(subcommand)]
    pub command: TxCliAction,
}

#[derive(Clone, PartialEq, Eq, Debug, Subcommand)]
pub enum TxCliAction {
    #[command(about = "Deploys a smart contract on the blockchain.")]
    Deploy(DeployArgs),

    #[command(about = "Calls a smart contract endpoint.")]
    Call(CallArgs),

    #[command(about = "Upgrades a previously deployed smart contract.")]
    Upgrade(UpgradeArgs),

    #[command(about = "Performs a VM query (no transaction, no wallet required).")]
    Query(QueryArgs),

    #[command(about = "Creates and optionally broadcasts a generic transaction.")]
    New(NewArgs),

    #[command(about = "Broadcasts a previously signed transaction from a file.")]
    Send(SendArgs),

    #[command(about = "Signs an unsigned transaction from a file.")]
    Sign(SignArgs),
}

/// Gateway / network arguments shared by commands that talk to the blockchain.
#[derive(Clone, PartialEq, Eq, Debug, Args)]
pub struct GatewayArgs {
    /// MultiversX proxy URL (e.g. https://devnet-gateway.multiversx.com).
    #[arg(long = "proxy")]
    pub proxy: String,

    /// Chain ID override (e.g. D for devnet, T for testnet, 1 for mainnet).
    /// If omitted, the chain ID is taken from the network config automatically.
    #[arg(long = "chain")]
    pub chain: Option<String>,
}

/// Wallet / sender arguments shared by commands that sign transactions.
#[derive(Clone, PartialEq, Eq, Debug, Args)]
pub struct SenderArgs {
    /// Path to a PEM wallet file.
    #[arg(long, group = "wallet_source")]
    pub pem: Option<PathBuf>,

    /// Path to a JSON keystore wallet file.
    #[arg(long, group = "wallet_source")]
    pub keyfile: Option<PathBuf>,

    /// Wallet index used when deriving from a PEM with multiple entries (default: 0).
    #[arg(long, default_value = "0")]
    pub sender_wallet_index: u32,
}

/// Generic transaction arguments (gas, nonce, value, broadcast flags).
#[derive(Clone, PartialEq, Eq, Debug, Args)]
pub struct TxArgs {
    /// Gas limit for the transaction.
    #[arg(long)]
    pub gas_limit: u64,

    /// Gas price override in smallest EGLD denomination.
    /// If omitted, the minimum gas price is taken from the network config automatically.
    #[arg(long)]
    pub gas_price: Option<u64>,

    /// Explicit nonce to use. If omitted, the current account nonce is fetched automatically.
    #[arg(long)]
    pub nonce: Option<u64>,

    /// EGLD value to send with the transaction, in smallest denomination (default: 0).
    #[arg(long, default_value = "0")]
    pub value: u64,

    /// If set, the transaction is broadcast to the network.
    /// Without this flag the signed tx JSON is written to --outfile or stdout.
    #[arg(long, default_value = "false")]
    pub send: bool,

    /// Wait for the transaction result (only meaningful when --send is also set).
    #[arg(long, default_value = "false")]
    pub wait_result: bool,

    /// Path to write the signed tx JSON to. Defaults to stdout when --send is not set.
    #[arg(long)]
    pub outfile: Option<PathBuf>,
}

/// Code metadata flags used by deploy and upgrade.
#[derive(Clone, PartialEq, Eq, Debug, Args)]
pub struct MetadataArgs {
    /// Mark the contract as NOT upgradeable (default: upgradeable).
    #[arg(long, default_value = "false")]
    pub metadata_not_upgradeable: bool,

    /// Mark the contract as NOT readable (default: readable).
    #[arg(long, default_value = "false")]
    pub metadata_not_readable: bool,

    /// Mark the contract as payable (default: not payable).
    #[arg(long, default_value = "false")]
    pub metadata_payable: bool,

    /// Mark the contract as payable by smart contracts (default: not payable by SC).
    #[arg(long, default_value = "false")]
    pub metadata_payable_by_sc: bool,
}

#[derive(Clone, PartialEq, Eq, Debug, Args)]
pub struct DeployArgs {
    /// Path to the compiled .wasm bytecode file.
    #[arg(long)]
    pub bytecode: PathBuf,

    /// Constructor arguments in mandos expression format (e.g. 0x1a, str:hello, 42).
    #[arg(long, num_args = 0..)]
    pub arguments: Vec<String>,

    #[command(flatten)]
    pub gateway: GatewayArgs,

    #[command(flatten)]
    pub sender: SenderArgs,

    #[command(flatten)]
    pub tx: TxArgs,

    #[command(flatten)]
    pub metadata: MetadataArgs,
}

#[derive(Clone, PartialEq, Eq, Debug, Args)]
pub struct CallArgs {
    /// Bech32 address of the contract to call.
    pub contract: String,

    /// Name of the endpoint to call.
    #[arg(long)]
    pub function: String,

    /// Endpoint arguments in mandos expression format.
    #[arg(long, num_args = 0..)]
    pub arguments: Vec<String>,

    /// ESDT token transfers as a flat list: TOKEN-abc AMOUNT TOKEN-def AMOUNT ...
    #[arg(long, num_args = 0..)]
    pub token_transfers: Vec<String>,

    #[command(flatten)]
    pub gateway: GatewayArgs,

    #[command(flatten)]
    pub sender: SenderArgs,

    #[command(flatten)]
    pub tx: TxArgs,
}

#[derive(Clone, PartialEq, Eq, Debug, Args)]
pub struct UpgradeArgs {
    /// Bech32 address of the contract to upgrade.
    pub contract: String,

    /// Path to the compiled .wasm bytecode file.
    #[arg(long)]
    pub bytecode: PathBuf,

    /// Init endpoint to call after upgrade (optional).
    #[arg(long)]
    pub function: Option<String>,

    /// Arguments in mandos expression format.
    #[arg(long, num_args = 0..)]
    pub arguments: Vec<String>,

    #[command(flatten)]
    pub gateway: GatewayArgs,

    #[command(flatten)]
    pub sender: SenderArgs,

    #[command(flatten)]
    pub tx: TxArgs,

    #[command(flatten)]
    pub metadata: MetadataArgs,
}

#[derive(Clone, PartialEq, Eq, Debug, Args)]
pub struct QueryArgs {
    /// Bech32 address of the contract to query.
    pub contract: String,

    /// Name of the view endpoint to query.
    #[arg(long)]
    pub function: String,

    /// Query arguments in mandos expression format.
    #[arg(long, num_args = 0..)]
    pub arguments: Vec<String>,

    #[command(flatten)]
    pub gateway: GatewayArgs,
}

#[derive(Clone, PartialEq, Eq, Debug, Args)]
pub struct NewArgs {
    /// Bech32 address of the receiver.
    #[arg(long)]
    pub receiver: String,

    /// Raw data payload (mutually exclusive with --token-transfers).
    #[arg(long, conflicts_with = "token_transfers")]
    pub data: Option<String>,

    /// Path to a file whose contents are used as the data payload.
    #[arg(long, conflicts_with = "token_transfers")]
    pub data_file: Option<PathBuf>,

    /// ESDT token transfers as a flat list: TOKEN-abc AMOUNT TOKEN-def AMOUNT ...
    /// Mutually exclusive with --data / --data-file.
    #[arg(long, num_args = 0..)]
    pub token_transfers: Vec<String>,

    #[command(flatten)]
    pub gateway: GatewayArgs,

    #[command(flatten)]
    pub sender: SenderArgs,

    #[command(flatten)]
    pub tx: TxArgs,
}

#[derive(Clone, PartialEq, Eq, Debug, Args)]
pub struct SendArgs {
    /// MultiversX proxy URL.
    #[arg(long = "proxy")]
    pub proxy: String,

    /// Path to the signed tx JSON file to broadcast.
    #[arg(long)]
    pub infile: PathBuf,

    /// Path to write the broadcast result JSON to. Defaults to stdout.
    #[arg(long)]
    pub outfile: Option<PathBuf>,

    /// Wait for the transaction result after broadcasting.
    #[arg(long, default_value = "false")]
    pub wait_result: bool,
}

#[derive(Clone, PartialEq, Eq, Debug, Args)]
pub struct SignArgs {
    /// Path to the unsigned tx JSON file to sign.
    #[arg(long)]
    pub infile: PathBuf,

    /// Path to write the signed tx JSON to. Defaults to stdout.
    #[arg(long)]
    pub outfile: Option<PathBuf>,

    /// If set, also broadcasts the signed transaction.
    #[arg(long, default_value = "false")]
    pub send: bool,

    #[command(flatten)]
    pub sender: SenderArgs,

    #[command(flatten)]
    pub gateway: GatewayArgs,
}
