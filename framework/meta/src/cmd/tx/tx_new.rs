use std::fs;

use anyhow::{Context, Result};
use multiversx_sc_snippets::{
    hex,
    imports::{Bech32Address, Interactor, InteractorRunAsync},
    sdk::utils::base64_encode,
};

use crate::cmd::tx::tx_common::load_wallet;

use super::{
    output::TxOutputFile,
    tx_cli_args::NewArgs,
    tx_common::{broadcast_and_save, save_output},
};

pub async fn tx_new(args: &NewArgs) {
    if let Err(e) = tx_new_inner(args).await {
        eprintln!("Error: {e:#}");
        std::process::exit(1);
    }
}

async fn tx_new_inner(args: &NewArgs) -> Result<()> {
    let wallet = load_wallet(&args.sender)?;
    let receiver = Bech32Address::try_from_bech32_string(args.receiver.clone())?;

    // Create the interactor – this fetches the network config in the process.
    let mut interactor = Interactor::new(&args.gateway.proxy).await;
    let sender_address = interactor.register_wallet(wallet).await;
    let sender = sender_address.to_bech32(interactor.get_hrp());

    // Determine nonce (explicit override or recalled from network).
    let nonce = if let Some(n) = args.tx.nonce {
        n
    } else {
        interactor.recall_nonce(&sender_address).await
    };

    // Build data field.
    let data_raw = build_data_bytes(args)?;
    let decoded_data = String::from_utf8_lossy(&data_raw).into_owned();
    let data_b64 = if data_raw.is_empty() {
        None
    } else {
        Some(base64_encode(&data_raw))
    };

    // Build Transaction via unified Tx syntax (resembles interactor code).
    let mut tx = interactor
        .tx()
        .from(&sender)
        .to(&receiver)
        .gas(args.tx.gas_limit)
        .egld(args.tx.value)
        .into_sdk_transaction();

    // Apply the fields that the Tx builder delegates to the caller.
    tx.nonce = nonce;
    tx.data = data_b64;
    if let Some(gas_price) = args.tx.gas_price {
        tx.gas_price = gas_price;
    }
    if let Some(chain_id) = &args.gateway.chain {
        tx.chain_id = chain_id.clone();
    }

    let sig = wallet.sign_tx(&tx);
    tx.signature = Some(hex::encode(sig));

    let output = TxOutputFile {
        emitted_transaction: tx,
        emitted_transaction_data: decoded_data,
        emitted_transaction_hash: String::new(),
        contract_address: None,
        transaction_on_network: None,
    };

    save_output(&output, args.tx.outfile.as_deref(), !args.tx.send)?;
    if args.tx.send {
        broadcast_and_save(
            output,
            &args.gateway.proxy,
            args.tx.outfile.as_deref(),
            args.tx.wait_result,
        )
        .await?;
    }
    Ok(())
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
