use anyhow::{Context, Result, anyhow};
use clap::Args;
use multiversx_sc_snippets::sdk::{wallet::Keystore, wallet::Wallet};
use std::{io::Write, path::PathBuf};

/// Wallet / sender arguments shared by commands that sign transactions.
#[derive(Clone, PartialEq, Eq, Debug, Args)]
pub struct SenderArgs {
    /// Path to a PEM wallet file.
    #[arg(long, group = "wallet_source")]
    pub pem: Option<PathBuf>,

    /// Path to a JSON keystore wallet file.
    #[arg(long, group = "wallet_source")]
    pub keyfile: Option<PathBuf>,

    /// Keystore password (plain text). If omitted, will prompt interactively.
    #[arg(long = "keystore-password", verbatim_doc_comment)]
    pub keystore_password: Option<String>,

    /// Use a Ledger hardware wallet for signing.
    #[arg(long, group = "wallet_source")]
    pub ledger: bool,

    /// Address index to use on the Ledger device.
    #[arg(long, default_value = "0")]
    pub sender_wallet_index: u32,
}

/// Load a wallet from a PEM file, JSON keystore, or Ledger device.
/// Wallet arguments for the relayer. Mirrors [`SenderArgs`] with a `--relayer-*` prefix.
#[derive(Clone, PartialEq, Eq, Debug, Args)]
pub struct RelayerArgs {
    /// Path to a PEM wallet file for the relayer.
    #[arg(long = "relayer-pem", group = "relayer_source")]
    pub relayer_pem: Option<PathBuf>,

    /// Path to a JSON keystore wallet file for the relayer.
    #[arg(long = "relayer-keyfile", group = "relayer_source")]
    pub relayer_keyfile: Option<PathBuf>,

    /// Relayer keystore password (plain text). If omitted, will prompt interactively.
    #[arg(long = "relayer-keystore-password")]
    pub relayer_keystore_password: Option<String>,
}

/// Load a wallet from a PEM file or JSON keystore.
pub fn load_wallet(sender: &SenderArgs) -> Result<Wallet> {
    if let Some(pem) = &sender.pem {
        Wallet::from_pem_file(pem).context("failed to load PEM wallet")
    } else if let Some(keyfile) = &sender.keyfile {
        let password = match &sender.keystore_password {
            Some(pw) => pw.clone(),
            None => get_keystore_password(),
        };
        let keystore = Keystore::from_file(keyfile)?;
        keystore
            .decrypt_wallet(&password)
            .context("failed to load keystore wallet")
    } else if sender.ledger {
        load_ledger_wallet(sender.sender_wallet_index)
    } else {
        Err(anyhow!(
            "a wallet is required: use --pem, --keyfile, or --ledger"
        ))
    }
}

#[cfg(feature = "ledger")]
fn load_ledger_wallet(address_index: u32) -> Result<Wallet> {
    use multiversx_chain_core::std::Bech32Address;
    use multiversx_sc_snippets::sdk::wallet::ledger::LedgerApp;

    let mut app = LedgerApp::new().map_err(|e| anyhow!("{e}"))?;
    let bech32_str = app.get_address(address_index).map_err(|e| anyhow!("{e}"))?;
    let bech32_addr = Bech32Address::from_bech32_string(bech32_str);
    let hrp = bech32_addr.hrp;
    let address = bech32_addr.address;
    Ok(Wallet::new_ledger(address, hrp, address_index))
}

#[cfg(not(feature = "ledger"))]
fn load_ledger_wallet(address_index: u32) -> Result<Wallet> {
    let _ = address_index;
    Err(anyhow!(
        "Ledger support is not available; recompile with the `ledger` feature enabled"
    ))
}

/// Load a relayer wallet from [`RelayerArgs`], returning `None` if no relayer args are set.
pub fn load_relayer_wallet(args: &RelayerArgs) -> Result<Option<Wallet>> {
    if let Some(pem) = &args.relayer_pem {
        let wallet = Wallet::from_pem_file(pem).context("failed to load relayer PEM wallet")?;
        Ok(Some(wallet))
    } else if let Some(keyfile) = &args.relayer_keyfile {
        let password = match &args.relayer_keystore_password {
            Some(pw) => pw.clone(),
            None => get_keystore_password(),
        };
        let keystore = Keystore::from_file(keyfile)?;
        let wallet = keystore
            .decrypt_wallet(&password)
            .context("failed to load relayer keystore wallet")?;
        Ok(Some(wallet))
    } else {
        Ok(None)
    }
}

pub fn get_keystore_password() -> String {
    print!("Insert password: ");
    std::io::stdout().flush().unwrap();
    rpassword::read_password().unwrap()
}
