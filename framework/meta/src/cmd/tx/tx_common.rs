use std::fs;

use anyhow::{Context, Result, anyhow};
use multiversx_sc_snippets::{
    hex,
    imports::{
        BytesValue, GatewayHttpProxy, InterpretableFrom, InterpreterContext, ManagedArgBuffer,
        ManagedBuffer, StaticApi,
    },
    sdk::{
        data::{keystore::InsertPassword, transaction::Transaction},
        utils::base64_decode,
        wallet::Wallet,
    },
};
use serde::Serialize;

use crate::cmd::tx::tx_send::fetch_tx_on_network;

use super::{
    output::TxOutputFile,
    tx_cli_args::{GatewayArgs, SenderArgs, TxArgs},
};

/// Serialize a value to a JSON string with 4-space indentation (matches mxpy output).
fn to_json_pretty<T: Serialize>(value: &T) -> Result<String> {
    let mut buf = Vec::new();
    let formatter = serde_json::ser::PrettyFormatter::with_indent(b"    ");
    let mut ser = serde_json::Serializer::with_formatter(&mut buf, formatter);
    value
        .serialize(&mut ser)
        .context("failed to serialize transaction")?;
    String::from_utf8(buf).context("non-UTF8 in serialized JSON")
}

pub use multiversx_sc_scenario::multiversx_sc::chain_core::std::new_address::compute_new_address_bech32;

/// Load a wallet from a PEM file or JSON keystore.
pub fn load_wallet(sender: &SenderArgs) -> Result<Wallet> {
    if let Some(pem) = &sender.pem {
        Wallet::from_pem_file(pem.to_str().context("invalid pem path")?)
            .context("failed to load PEM wallet")
    } else if let Some(keyfile) = &sender.keyfile {
        Wallet::from_keystore_secret(
            keyfile.to_str().context("invalid keyfile path")?,
            InsertPassword::StandardInput,
        )
        .context("failed to load keystore wallet")
    } else {
        Err(anyhow!("a wallet is required: use --pem or --keyfile"))
    }
}

/// Interpret a list of mandos-format argument strings (e.g. `0x1a`, `str:hello`, `42`)
/// into a `ManagedArgBuffer` ready to be passed to `.arguments_raw()`.
pub fn build_arg_buffer(arguments: &[String]) -> Result<ManagedArgBuffer<StaticApi>> {
    let context =
        InterpreterContext::new().with_dir(std::env::current_dir().context("failed to get cwd")?);
    let mut arg_buffer = ManagedArgBuffer::<StaticApi>::new();
    for s in arguments {
        let bv = BytesValue::interpret_from(s.as_str(), &context);
        arg_buffer.push_arg_raw(ManagedBuffer::new_from_bytes(&bv.value));
    }
    Ok(arg_buffer)
}

/// Apply nonce / gas-price / chain-id overrides, sign the transaction, then
/// write / print / broadcast it according to the `TxArgs` flags.
/// `contract_address` should be `Some(bech32)` for deploy transactions.
pub async fn sign_and_dispatch(
    wallet: Wallet,
    mut tx: Transaction,
    nonce: u64,
    tx_args: &TxArgs,
    gateway_args: &GatewayArgs,
    contract_address: Option<String>,
) -> Result<()> {
    // Apply caller-controlled overrides.
    tx.nonce = nonce;
    if let Some(gas_price) = tx_args.gas_price {
        tx.gas_price = gas_price;
    }
    if let Some(chain_id) = &gateway_args.chain {
        tx.chain_id = chain_id.clone();
    }

    let decoded_data = tx
        .data
        .as_ref()
        .map(|d| String::from_utf8_lossy(&base64_decode(d)).into_owned())
        .unwrap_or_default();

    let sig = wallet.sign_tx(&tx);
    tx.signature = Some(hex::encode(sig));

    let output = TxOutputFile {
        emitted_transaction: tx,
        emitted_transaction_data: decoded_data,
        emitted_transaction_hash: String::new(),
        contract_address,
        transaction_on_network: None,
    };

    let json = to_json_pretty(&output)?;

    // Write / print the signed tx.
    if let Some(outfile) = &tx_args.outfile {
        fs::write(outfile, &json)
            .with_context(|| format!("failed to write to {}", outfile.display()))?;
        println!("Transaction saved to {}", outfile.display());
    } else if !tx_args.send {
        println!("{json}");
    }

    // Optionally broadcast.
    if tx_args.send {
        let proxy = GatewayHttpProxy::new(gateway_args.proxy.clone());
        let tx_hash = proxy
            .send_transaction(&output.emitted_transaction)
            .await
            .context("failed to broadcast transaction")?;
        println!("Transaction hash: {tx_hash}");

        let mut output_with_hash = TxOutputFile {
            emitted_transaction_hash: tx_hash.clone(),
            ..output
        };

        if tx_args.wait_result {
            println!("Waiting for transaction result...");
            let result = fetch_tx_on_network(&gateway_args.proxy, &tx_hash).await?;
            output_with_hash.transaction_on_network = Some(result);
        }

        let json = to_json_pretty(&output_with_hash)?;
        if let Some(outfile) = &tx_args.outfile {
            fs::write(outfile, &json)
                .with_context(|| format!("failed to write to {}", outfile.display()))?;
        } else {
            println!("{json}");
        }
    }

    Ok(())
}
