use anyhow::{Context, Result, anyhow};
use clap::Args;
use multiversx_sc_snippets::sdk::{data::keystore::InsertPassword, wallet::Wallet};
use std::path::PathBuf;

/// Wallet / sender arguments shared by commands that sign transactions.
#[derive(Clone, PartialEq, Eq, Debug, Args)]
pub struct SenderArgs {
    /// Path to a PEM wallet file.
    #[arg(long, group = "wallet_source")]
    pub pem: Option<PathBuf>,

    /// Path to a JSON keystore wallet file.
    #[arg(long, group = "wallet_source")]
    pub keyfile: Option<PathBuf>,
}

/// Load a wallet from a PEM file or JSON keystore.
pub fn load_wallet(sender: &SenderArgs) -> Result<Wallet> {
    if let Some(pem) = &sender.pem {
        Wallet::from_pem_file(pem).context("failed to load PEM wallet")
    } else if let Some(keyfile) = &sender.keyfile {
        Wallet::from_keystore_secret(keyfile, InsertPassword::StandardInput)
            .context("failed to load keystore wallet")
    } else {
        Err(anyhow!("a wallet is required: use --pem or --keyfile"))
    }
}
