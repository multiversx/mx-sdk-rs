use std::fs;

use anyhow::{Context, Result, anyhow};
use multiversx_sc::imports::Bech32Address;
use multiversx_sc_snippets::{
    imports::{
        BytesValue, Interactor, InterpretableFrom, InterpreterContext, ManagedArgBuffer,
        ManagedBuffer, StaticApi,
    },
    sdk::data::transaction::Transaction,
};

use serde_json::Value;

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

/// Create an interactor connected to `gateway`, with chain-simulator mode auto-detected,
/// the explicit nonce override from `tx` applied, the gas price override applied, and
/// the chain ID validated against `gateway.chain` (if given).
pub(super) async fn create_interactor(gateway: &GatewayArgs, tx: &TxArgs) -> Result<Interactor> {
    let mut interactor = Interactor::empty()
        .with_connection(&gateway.proxy)
        .await
        .use_chain_simulator_auto();
    interactor.override_next_tx_nonce = tx.nonce;
    apply_gas_price(&mut interactor, tx);
    validate_chain_id(&interactor, gateway)?;
    Ok(interactor)
}

fn apply_gas_price(interactor: &mut Interactor, tx_args: &TxArgs) {
    if let Some(gas_price) = tx_args.gas_price {
        interactor.gas_price = gas_price;
    }
}

fn validate_chain_id(interactor: &Interactor, gateway_args: &GatewayArgs) -> Result<()> {
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
