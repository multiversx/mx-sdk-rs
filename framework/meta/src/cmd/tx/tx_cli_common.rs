use std::fs;

use anyhow::{Context, Result, anyhow};
use multiversx_chain_core::std::base64_decode;
use multiversx_sc::imports::Bech32Address;
use multiversx_sc_snippets::ExplorerUrl;
use multiversx_sc_snippets::{
    hex,
    imports::{
        BytesValue, GatewayHttpProxy, Interactor, InterpretableFrom, InterpreterContext,
        ManagedArgBuffer, ManagedBuffer, StaticApi,
    },
    sdk::data::transaction::{ApiTransactionResult, Transaction},
};
use serde::Serialize;

use multiversx_sc_scenario::imports::ReturnCode;
use multiversx_sc_snippets::network_response;
use serde_json::Value;

use super::output::TxOutputFile;
pub use crate::cli::cli_args_sender::{load_relayer_wallet, load_wallet};
use crate::cli::cli_args_tx::{GatewayArgs, RelayerArgs, TxArgs};

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
    if output.emitted_transaction.relayer.is_some()
        && output.emitted_transaction.relayer_signature.is_none()
    {
        return Err(anyhow!(
            "relayed transaction is missing relayer signature; use `tx relay` to add it"
        ));
    }

    let proxy = GatewayHttpProxy::new(proxy_url.to_string());
    let tx_hash = proxy
        .send_transaction(&output.emitted_transaction)
        .await
        .context("failed to broadcast transaction")?;
    if let Some(ex) = ExplorerUrl::from_chain_id(&output.emitted_transaction.chain_id) {
        println!("transaction: {}", ex.tx_url(&tx_hash));
    } else {
        println!("transaction hash: {tx_hash}");
    }

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

pub fn apply_gas_price(interactor: &mut Interactor, tx_args: &TxArgs) {
    if let Some(gas_price) = tx_args.gas_price {
        interactor.gas_price = gas_price;
    }
}

pub fn validate_chain_id(interactor: &Interactor, gateway_args: &GatewayArgs) -> Result<()> {
    if let Some(chain_id) = &gateway_args.chain {
        interactor.validate_chain_id(chain_id)?;
    }
    Ok(())
}

pub async fn load_relayer_for_interactor(
    interactor: &mut Interactor,
    relayer_args: &RelayerArgs,
) -> Result<Option<Bech32Address>> {
    let explicit_address = relayer_args
        .relayer
        .as_ref()
        .map(|address| {
            Bech32Address::try_from_bech32_string(address.clone())
                .map_err(|e| anyhow!("invalid --relayer address: {e}"))
        })
        .transpose()?;

    let Some(wallet) = load_relayer_wallet(relayer_args)? else {
        return Ok(explicit_address);
    };

    let wallet_address = interactor.register_wallet_bech32(wallet).await;
    if let Some(explicit_address) = explicit_address {
        if explicit_address != wallet_address {
            return Err(anyhow!(
                "relayer wallet address {} does not match --relayer address {}",
                wallet_address.to_bech32_str(),
                explicit_address.to_bech32_str(),
            ));
        }
        Ok(Some(explicit_address))
    } else {
        Ok(Some(wallet_address))
    }
}

/// Apply the nonce, sign the transaction, then
/// write / print / broadcast it according to the `TxArgs` flags.
/// `contract_address` should be `Some(bech32)` for deploy transactions.
/// The sender wallet and any relayer wallet must already be registered with `interactor`.
/// A relayer address without a registered wallet is preserved without a relayer signature.
pub async fn sign_and_dispatch(
    interactor: &Interactor,
    mut tx: Transaction,
    nonce: u64,
    tx_args: &TxArgs,
    gateway_args: &GatewayArgs,
    contract_address: Option<String>,
) -> Result<()> {
    tx.nonce = nonce;

    let decoded_data = match &tx.data {
        None => String::new(),
        Some(d) => {
            let bytes = base64_decode(d)?;
            String::from_utf8_lossy(&bytes).into_owned()
        }
    };

    interactor.sign_tx(&mut tx);

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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn explicit_relayer_address_does_not_register_wallet() {
        let mut interactor = Interactor::empty();
        let relayer = Bech32Address::zero_default_hrp();
        let args = RelayerArgs {
            relayer: Some(relayer.to_bech32_string()),
            relayer_pem: None,
            relayer_keyfile: None,
            relayer_keystore_password: None,
        };

        let loaded_relayer = load_relayer_for_interactor(&mut interactor, &args)
            .await
            .unwrap();

        assert_eq!(loaded_relayer, Some(relayer));
        assert!(interactor.sender_map.is_empty());
    }
}
