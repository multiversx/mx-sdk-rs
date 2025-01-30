use clap::{Args, Parser, Subcommand};

/// Adder Interact CLI
#[derive(Default, PartialEq, Eq, Debug, Parser)]
#[command(version, about)]
#[command(propagate_version = true)]
pub struct InteractCli {
    #[command(subcommand)]
    pub command: Option<InteractCliCommand>,
}

/// Adder Interact CLI Commands
#[derive(Clone, PartialEq, Eq, Debug, Subcommand)]
pub enum InteractCliCommand {
    #[command(name = "deploy", about = "Deploy contract")]
    Deploy,
    #[command(name = "callback-data", about = "Callback data")]
    CallbackData,
    #[command(name = "callback-data-at-index", about = "Callback data at index")]
    CallbackDataAtIndex(CallbackDataAtIndexArgs),
    #[command(name = "clear_callback_data", about = "Clear callback data")]
    ClearCallbackData,
    #[command(
        name = "forward-promise-accept-funds",
        about = "Forward promise accept funds"
    )]
    ForwardPromiseAcceptFunds(ForwardPromiseFundsArgs),
    #[command(
        name = "forward-promise-retrieve-funds",
        about = "Forward promise retrieve funds"
    )]
    ForwardPromiseRetrieveFunds(ForwardPromiseFundsArgs),
    #[command(name = "forward-payment-callback", about = "Forward payment callback")]
    ForwardPaymentCallback(ForwardPromiseFundsArgs),
    #[command(name = "promise-raw-single-token", about = "Promise raw single token")]
    PromiseRawSingleToken(PromiseRawSingleTokenArgs),
    #[command(
        name = "promise-raw-multi-transfer",
        about = "Promise raw multi transfer"
    )]
    PromiseRawMultiTransfer(PromiseRawMultiTransferArgs),
    #[command(
        name = "forward-sync-retrieve-funds-bt",
        about = "Forward sync retrieve funds bt"
    )]
    ForwardSyncRetrieveFundsBt(ForwardPromiseFundsArgs),
    #[command(
        name = "forward-sync-retrieve-funds-bt-twice",
        about = "Forward sync retrieve funds bt twice"
    )]
    ForwardSyncRetrieveFundsBtTwice(ForwardPromiseFundsArgs),
    #[command(
        name = "forward-promise-retrieve-funds-back-transfers",
        about = "Forward sync retrieve funds bt twice"
    )]
    ForwardPromiseRetrieveFundsBackTransfers(ForwardPromiseFundsArgs),
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct CallbackDataAtIndexArgs {
    #[arg(short = 'i', long = "index")]
    pub index: u32,
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct ForwardPromiseFundsArgs {
    #[arg(short = 'i', long = "token-id")]
    pub token_id: String,
    #[arg(short = 'n', long = "token-nonce")]
    pub token_nonce: u64,
    #[arg(short = 'a', long = "token-amount")]
    pub token_amount: u64,
    #[arg(short = 't', long = "to")]
    pub to: String,
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct PromiseRawSingleTokenArgs {
    #[arg(short = 'i', long = "token-id")]
    pub token_id: String,
    #[arg(short = 'n', long = "token-nonce")]
    pub token_nonce: u64,
    #[arg(short = 'a', long = "token-amount")]
    pub token_amount: u64,
    #[arg(short = 't', long = "to")]
    pub to: String,
    #[arg(short = 'e', long = "endpoint-name")]
    pub endpoint_name: String,
    #[arg(short = 'g', long = "gas-limit")]
    pub gas_limit: u64,
    #[arg(short = 'x', long = "extra-gas-for-callback")]
    pub extra_gas_for_callback: u64,
    #[arg(short = 'a', long = "args")]
    pub args: String,
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct PromiseRawMultiTransferArgs {
    #[arg(short = 't', long = "to")]
    pub to: String,
    #[arg(short = 'e', long = "endpoint-name")]
    pub endpoint_name: String,
    #[arg(short = 'x', long = "extra-gas-for-callback")]
    pub extra_gas_for_callback: u64,
}
