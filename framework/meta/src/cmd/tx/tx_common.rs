use std::fs;

use anyhow::{Context, Result, anyhow};
use multiversx_sc_snippets::{
    hex,
    imports::{
        BigUint, BytesValue, GatewayHttpProxy, InterpretableFrom, InterpreterContext,
        ManagedArgBuffer, ManagedBuffer, Payment, PaymentVec, RustBigUint, StaticApi, TokenId,
    },
    sdk::{
        data::{
            keystore::InsertPassword,
            transaction::{ApiTransactionResult, Transaction},
        },
        utils::base64_decode,
        wallet::Wallet,
    },
};
use serde::Serialize;

use multiversx_sc_scenario::{imports::ReturnCode, multiversx_sc::types::CodeMetadata};
use multiversx_sc_snippets::network_response;
use serde_json::Value;

use super::{
    output::TxOutputFile,
    tx_cli_args::{GatewayArgs, MetadataArgs, PaymentArgs, SenderArgs, TxArgs},
};

/// Load a transaction from an mxpy-compatible interaction JSON file.
/// Accepts both `{"emittedTransaction": {...}}` and `{"tx": {...}}` wrappers.
pub(super) fn load_transaction_from_file(path: &std::path::Path) -> Result<Transaction> {
    let content =
        fs::read_to_string(path).with_context(|| format!("failed to read {}", path.display()))?;
    let v: Value = serde_json::from_str(&content)
        .with_context(|| format!("invalid JSON in {}", path.display()))?;

    let tx_value = v
        .get("emittedTransaction")
        .or_else(|| v.get("tx"))
        .ok_or_else(|| {
            anyhow!(
                "file {} must contain an \"emittedTransaction\" or \"tx\" key",
                path.display()
            )
        })?;

    serde_json::from_value(tx_value.clone())
        .with_context(|| format!("failed to deserialize transaction from {}", path.display()))
}

/// Wait for a transaction result on the network.
pub(super) async fn fetch_tx_on_network(
    gateway: &str,
    tx_hash: &str,
) -> Result<(ApiTransactionResult, ReturnCode)> {
    let proxy = GatewayHttpProxy::new(gateway.to_string());
    multiversx_sdk::retrieve_tx_on_network(&proxy, tx_hash.to_string()).await
}

/// Write `output` to `outfile`, or print to stdout when no outfile is given.
pub(super) fn save_output(output: &TxOutputFile, outfile: Option<&std::path::Path>) -> Result<()> {
    let json = to_json_pretty(output)?;
    if let Some(path) = outfile {
        fs::write(path, &json).with_context(|| format!("failed to write to {}", path.display()))?;
        println!("Transaction saved to {}", path.display());
    } else {
        println!("{json}");
    }
    Ok(())
}

/// Broadcast the transaction inside `output`, update the hash (and optionally
/// the on-network result), then write/print the updated output.
pub(super) async fn broadcast_and_save(
    output: TxOutputFile,
    proxy_url: &str,
    outfile: Option<&std::path::Path>,
    wait_result: bool,
) -> Result<()> {
    if output.emitted_transaction.signature.is_none() {
        return Err(anyhow!(
            "transaction is not signed; sign it before broadcasting"
        ));
    }

    let proxy = GatewayHttpProxy::new(proxy_url.to_string());
    let tx_hash = proxy
        .send_transaction(&output.emitted_transaction)
        .await
        .context("failed to broadcast transaction")?;
    println!("Transaction hash: {tx_hash}");

    let mut output_with_hash = TxOutputFile {
        emitted_transaction_hash: tx_hash.clone(),
        ..output
    };

    if wait_result {
        println!("Waiting for transaction result...");
        let (tx_on_network, return_code) = fetch_tx_on_network(proxy_url, &tx_hash).await?;
        let tx_response = network_response::parse_tx_response(tx_on_network.clone(), return_code);
        print_tx_results(&tx_response);
        output_with_hash.transaction_on_network = Some(tx_on_network);
    }

    let json = to_json_pretty(&output_with_hash)?;
    if let Some(path) = outfile {
        fs::write(path, &json).with_context(|| format!("failed to write to {}", path.display()))?;
    } else {
        println!("{json}");
    }
    Ok(())
}

/// Serialize a value to a JSON string with 4-space indentation (matches mxpy output).
pub(super) fn to_json_pretty<T: Serialize>(value: &T) -> Result<String> {
    let mut buf = Vec::new();
    let formatter = serde_json::ser::PrettyFormatter::with_indent(b"    ");
    let mut ser = serde_json::Serializer::with_formatter(&mut buf, formatter);
    value
        .serialize(&mut ser)
        .context("failed to serialize transaction")?;
    String::from_utf8(buf).context("non-UTF8 in serialized JSON")
}

pub use multiversx_sc_scenario::multiversx_sc::chain_core::std::new_address::compute_new_address_bech32;

/// Parse the flat token-transfer list (`TOKEN-ident AMOUNT TOKEN-ident AMOUNT …`)
/// into a [`PaymentVec`] using the same extended-identifier format as mxpy:
/// NFT/SFT identifiers append a hex-encoded nonce, e.g. `NFT-abc123-0a` (nonce = 10);
/// fungible tokens are plain, e.g. `ESDT-abc123`.
/// Build a [`PaymentVec`] from [`PaymentArgs`]: the flat `--token-transfers` list plus
/// the `--value` EGLD amount (appended last as a native `EGLD-000000` payment).
/// The interactor's `.payment()` normalises the vec into the correct transaction fields.
pub fn build_payments(payment: &PaymentArgs) -> Result<PaymentVec<StaticApi>> {
    let mut payments = PaymentVec::new();
    payments.append_vec(parse_token_transfers(&payment.token_transfers)?);
    payments.append_vec(parse_payments(&payment.payments)?);
    if payment.value > 0 {
        let amount = BigUint::<StaticApi>::from(payment.value);
        payments.push(
            Payment::try_new(TokenId::native(), 0u64, amount)
                .map_err(|_| anyhow!("EGLD value must be non-zero"))?,
        );
    }
    Ok(payments)
}

/// Parse a flat `--token-transfers` list (`TOKEN-IDENT AMOUNT …` pairs) into payments.
/// The token identifier may include a hex nonce suffix for NFT/SFT: `TOKEN-abc-0a`.
fn parse_token_transfers(transfers: &[String]) -> Result<PaymentVec<StaticApi>> {
    if transfers.len() % 2 != 0 {
        return Err(anyhow!(
            "--token-transfers requires an even number of values (TOKEN-IDENT AMOUNT …)"
        ));
    }
    let mut payments = PaymentVec::new();
    for chunk in transfers.chunks(2) {
        let extended_id = &chunk[0];
        let amount_str = &chunk[1];
        let (base_id, nonce) = split_extended_identifier(extended_id);
        let rust_amount: RustBigUint = amount_str
            .parse()
            .with_context(|| format!("invalid token amount: {amount_str}"))?;
        let amount = BigUint::<StaticApi>::from(rust_amount);
        payments.push(
            Payment::try_new(
                TokenId::<StaticApi>::from(base_id.as_bytes()),
                nonce,
                amount,
            )
            .map_err(|_| anyhow!("token amount must be non-zero: {extended_id}"))?,
        );
    }
    Ok(payments)
}

/// Parse a flat `--payments` list (`TOKEN-IDENT NONCE AMOUNT …` triples) into payments.
/// Nonce is an explicit decimal `u64`; use 0 for fungible tokens.
fn parse_payments(explicit: &[String]) -> Result<PaymentVec<StaticApi>> {
    if explicit.len() % 3 != 0 {
        return Err(anyhow!(
            "--payments requires a multiple of 3 values (TOKEN-IDENT NONCE AMOUNT …)"
        ));
    }
    let mut payments = PaymentVec::new();
    for chunk in explicit.chunks(3) {
        let token_id_str = &chunk[0];
        let nonce_str = &chunk[1];
        let amount_str = &chunk[2];
        let nonce: u64 = nonce_str
            .parse()
            .with_context(|| format!("invalid nonce: {nonce_str}"))?;
        let rust_amount: RustBigUint = amount_str
            .parse()
            .with_context(|| format!("invalid token amount: {amount_str}"))?;
        let amount = BigUint::<StaticApi>::from(rust_amount);
        payments.push(
            Payment::try_new(
                TokenId::<StaticApi>::from(token_id_str.as_bytes()),
                nonce,
                amount,
            )
            .map_err(|_| anyhow!("token amount must be non-zero: {token_id_str}"))?,
        );
    }
    Ok(payments)
}

/// Split an mxpy-style extended token identifier into `(base_identifier, nonce)`.
///
/// Format: `TOKEN-xxxxxx` (fungible, nonce = 0) or `TOKEN-xxxxxx-<hex>` (NFT/SFT).
fn split_extended_identifier(extended_id: &str) -> (String, u64) {
    let parts: Vec<&str> = extended_id.split('-').collect();
    if parts.len() >= 3 {
        let last = parts[parts.len() - 1];
        if !last.is_empty() && last.bytes().all(|b| b.is_ascii_hexdigit()) {
            if let Ok(nonce) = u64::from_str_radix(last, 16) {
                let base = parts[..parts.len() - 1].join("-");
                return (base, nonce);
            }
        }
    }
    (extended_id.to_string(), 0)
}

pub fn build_code_metadata(meta: &MetadataArgs) -> CodeMetadata {
    let mut flags = CodeMetadata::DEFAULT;
    if !meta.metadata_not_upgradeable {
        flags |= CodeMetadata::UPGRADEABLE;
    }
    if !meta.metadata_not_readable {
        flags |= CodeMetadata::READABLE;
    }
    if meta.metadata_payable {
        flags |= CodeMetadata::PAYABLE;
    }
    if meta.metadata_payable_by_sc {
        flags |= CodeMetadata::PAYABLE_BY_SC;
    }
    flags
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

    if tx_args.send {
        broadcast_and_save(
            output,
            &gateway_args.proxy,
            tx_args.outfile.as_deref(),
            tx_args.wait_result,
        )
        .await?;
    } else {
        save_output(&output, tx_args.outfile.as_deref())?;
    }
    Ok(())
}

/// Print the status and hex-encoded return values of a completed transaction.
pub(super) fn print_tx_results(tx_response: &multiversx_sc_scenario::scenario_model::TxResponse) {
    if tx_response.tx_error.is_success() {
        println!("Transaction successful.");
    } else {
        println!("Transaction failed: {}", tx_response.tx_error);
    }
    for (i, result) in tx_response.out.iter().enumerate() {
        println!("Result[{i}]: 0x{}", hex::encode(result));
    }
}
