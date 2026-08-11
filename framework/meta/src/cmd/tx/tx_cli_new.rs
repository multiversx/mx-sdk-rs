use std::fs;

use anyhow::{Context, Result};
use multiversx_chain_core::std::base64_encode;
use multiversx_sc_snippets::imports::{Bech32Address, Interactor, InteractorIntoSdkTransaction};

use crate::cli::cli_args_tx::NewArgs;
use crate::cmd::tx::tx_cli_common::{
    apply_gas_price, load_relayer_for_interactor, load_wallet, sign_and_dispatch, validate_chain_id,
};

use super::parse_payments::parse_all_payment_args;

pub async fn tx_new(args: &NewArgs) {
    if let Err(e) = tx_new_inner(args).await {
        eprintln!("Error: {e:#}");
        std::process::exit(1);
    }
}

async fn tx_new_inner(args: &NewArgs) -> Result<()> {
    let sender_wallet = load_wallet(&args.sender)?;
    let receiver = Bech32Address::try_from_bech32_string(args.receiver.clone())?;

    // Create the interactor – this fetches the network config in the process.
    let mut interactor = Interactor::new(&args.gateway.proxy).await;
    let sender_address = interactor
        .register_wallet_bech32(sender_wallet.clone())
        .await;
    let relayer_address_opt = load_relayer_for_interactor(&mut interactor, &args.relayer).await?;

    // Determine nonce (explicit override or recalled from network).
    let nonce = if let Some(n) = args.tx.nonce {
        n
    } else {
        interactor.recall_nonce(&sender_address.address).await
    };

    apply_gas_price(&mut interactor, &args.tx);
    validate_chain_id(&interactor, &args.gateway)?;

    // Build Transaction via unified Tx syntax (resembles interactor code).
    let payments = parse_all_payment_args(&args.payment)?;
    let mut tx = interactor
        .tx()
        .from(&sender_address)
        .to(&receiver)
        .gas(args.tx.gas_limit)
        .payment(payments)
        .opt_relayer(relayer_address_opt)
        .into_sdk_transaction();

    // Data field (mutually exclusive with --token-transfers; overrides only when provided).
    let data_raw = build_data_bytes(args)?;
    if !data_raw.is_empty() {
        tx.data = Some(base64_encode(&data_raw));
    }

    sign_and_dispatch(&interactor, tx, nonce, &args.tx, &args.gateway, None).await
}

// ---------------------------------------------------------------------------
// Data field helpers
// ---------------------------------------------------------------------------

fn build_data_bytes(args: &NewArgs) -> Result<Vec<u8>> {
    if let Some(data) = &args.data {
        Ok(data.as_bytes().to_vec())
    } else if let Some(data_file) = &args.data_file {
        fs::read(data_file)
            .with_context(|| format!("failed to read data file {}", data_file.display()))
    } else {
        Ok(Vec::new())
    }
}

// ---------------------------------------------------------------------------
// Broadcast
// ---------------------------------------------------------------------------
// (handled via GatewayHttpProxy::send_transaction from sdk/http)
