use std::fs;

use anyhow::{Context, Result};
use multiversx_chain_core::std::{base64_decode, base64_encode};
use multiversx_sc_snippets::imports::{Bech32Address, Interactor, InteractorIntoSdkTransaction};

use crate::cli::cli_args_tx::NewArgs;
use crate::cmd::tx::tx_cli_common::{load_relayer_wallet, load_wallet};

use super::{
    output::TxOutputFile,
    parse_payments::parse_all_payment_args,
    tx_cli_common::{broadcast_and_save, save_output},
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
    let sender_address = interactor.register_wallet(wallet.clone()).await;
    let sender = sender_address.to_bech32(interactor.get_hrp());

    // Determine nonce (explicit override or recalled from network).
    let nonce = if let Some(n) = args.tx.nonce {
        n
    } else {
        interactor.recall_nonce(&sender_address).await
    };

    // Build Transaction via unified Tx syntax (resembles interactor code).
    let payments = parse_all_payment_args(&args.payment)?;
    let mut tx = interactor
        .tx()
        .from(&sender)
        .to(&receiver)
        .gas(args.tx.gas_limit)
        .payment(payments)
        .into_sdk_transaction();

    // Data field (mutually exclusive with --token-transfers; overrides only when provided).
    let data_raw = build_data_bytes(args)?;
    if !data_raw.is_empty() {
        tx.data = Some(base64_encode(&data_raw));
    }

    // Decode the data field for the human-readable output.
    let decoded_data = match &tx.data {
        None => String::new(),
        Some(d) => {
            let bytes = base64_decode(d)?;
            String::from_utf8_lossy(&bytes).into_owned()
        }
    };
    tx.nonce = nonce;
    if let Some(gas_price) = args.tx.gas_price {
        tx.gas_price = gas_price;
    }
    if let Some(chain_id) = &args.gateway.chain {
        tx.chain_id = chain_id.clone();
    }

    // Set relayer address before signing — it is included in the signing bytes.
    // Priority: explicit --relayer flag > address derived from --relayer-pem.
    let relayer_wallet = load_relayer_wallet(&args.relayer)?;
    if let Some(relayer_str) = &args.tx.relayer {
        let relayer_addr = Bech32Address::try_from_bech32_string(relayer_str.clone())
            .map_err(|e| anyhow::anyhow!("invalid --relayer address: {e}"))?;
        tx.relayer = Some(relayer_addr);
    } else if let Some(ref rw) = relayer_wallet {
        tx.relayer = Some(rw.to_address().to_bech32_default());
    }

    let sig = wallet.sign_tx(&tx)?;
    tx.signature = Some(sig);

    // Optionally sign as relayer.
    if let Some(relayer_w) = relayer_wallet {
        let relayer_addr = relayer_w.to_address().to_bech32_default();
        if let Some(tx_relayer) = &tx.relayer {
            if relayer_addr != *tx_relayer {
                return Err(anyhow::anyhow!(
                    "relayer wallet address {} does not match --relayer {}",
                    relayer_addr.to_bech32_str(),
                    tx_relayer.to_bech32_str(),
                ));
            }
        }
        let relayer_sig = relayer_w.sign_tx(&tx)?;
        tx.relayer_signature = Some(relayer_sig);
    }

    let output = TxOutputFile {
        emitted_transaction: tx,
        emitted_transaction_data: decoded_data,
        emitted_transaction_hash: String::new(),
        contract_address: None,
        transaction_on_network: None,
    };

    if args.tx.send {
        broadcast_and_save(
            output,
            &args.gateway.proxy,
            args.tx.outfile.as_deref(),
            args.tx.wait_result,
        )
        .await?;
    } else {
        save_output(&output, args.tx.outfile.as_deref())?;
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
