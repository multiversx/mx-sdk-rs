use clap::{Args, Subcommand};

/// Arguments for the `sc-meta ledger` subcommand group.
#[derive(Clone, PartialEq, Eq, Debug, Args)]
pub struct LedgerArgs {
    #[command(subcommand)]
    pub command: LedgerAction,
}

#[derive(Clone, PartialEq, Eq, Debug, Subcommand)]
pub enum LedgerAction {
    #[command(about = "List addresses stored on the Ledger device.")]
    Addresses(LedgerAddressesArgs),

    #[command(about = "Print the version of the MultiversX Ledger app.")]
    Version,
}

#[derive(Clone, PartialEq, Eq, Debug, Args)]
pub struct LedgerAddressesArgs {
    /// Number of addresses to retrieve.
    #[arg(long = "num-addresses", default_value = "10")]
    pub num_addresses: u32,
}
