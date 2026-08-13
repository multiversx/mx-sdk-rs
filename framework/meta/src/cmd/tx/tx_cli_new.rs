use std::fs;

use anyhow::{Context, Result};
use multiversx_chain_core::std::base64_encode;
use multiversx_sc_snippets::imports::{Bech32Address, InteractorIntoSdkTransaction};

use crate::cli::cli_args_tx::NewArgs;
use crate::cmd::tx::tx_cli_common::{create_interactor, load_relayer_for_interactor, load_wallet};

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
    let mut interactor = create_interactor(&args.gateway, &args.tx).await?;
    let sender_address = interactor
        .register_wallet_bech32(sender_wallet.clone())
        .await;
    let relayer_address_opt = load_relayer_for_interactor(&mut interactor, &args.relayer).await?;

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
    interactor.set_tx_nonce_update_sender(&mut tx).await;

    interactor
        .sign_and_dispatch(
            tx,
            None,
            args.tx.send,
            args.tx.wait_result,
            args.tx.outfile.as_deref(),
        )
        .await
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
