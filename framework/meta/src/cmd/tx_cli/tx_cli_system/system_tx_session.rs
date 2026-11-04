use anyhow::{Result, anyhow};
use multiversx_sc_snippets::{
    imports::{Bech32Address, Interactor},
    sdk::data::transaction::Transaction,
};
use serde::Serialize;

use crate::cli::{
    cli_args_tx::{GatewayArgs, RelayerArgs, TxArgs},
    cli_args_tx_system::SystemTxCommonArgs,
};
use crate::cmd::tx_cli::tx_cli_common::{
    create_interactor, load_relayer_for_interactor, load_wallet,
};

/// Extra gas added to the default gas limit for relayed (v3) transactions.
const RELAYED_TX_EXTRA_GAS: u64 = 50_000;

/// Everything needed to build, sign and dispatch a transaction to a system smart contract.
///
/// Built on the same interactor machinery as the rest of `sc-meta tx`:
/// the transaction is produced with the unified `Tx` syntax and a system SC proxy,
/// converted with `into_sdk_transaction`, then signed and dispatched by the interactor.
pub(super) struct SystemTxSession {
    pub interactor: Interactor,
    pub sender: Bech32Address,
    pub relayer: Option<Bech32Address>,
    pub gas_limit: u64,
    tx_args: TxArgs,
}

impl SystemTxSession {
    /// Loads the wallets and connects to the gateway.
    ///
    /// `default_gas_limit` is used when `--gas-limit` is not given,
    /// increased with the relayed transaction extra cost when a relayer is set.
    pub async fn new(common: &SystemTxCommonArgs, default_gas_limit: u64) -> Result<Self> {
        let sender_wallet = load_wallet(&common.sender)?;

        let gas_limit = common.tx.gas_limit.unwrap_or_else(|| {
            if has_relayer(&common.relayer) {
                default_gas_limit + RELAYED_TX_EXTRA_GAS
            } else {
                default_gas_limit
            }
        });
        let tx_args = common.tx.to_tx_args(gas_limit);

        let mut interactor = create_interactor(&common.gateway, &tx_args).await?;
        let sender = interactor.register_wallet_bech32(sender_wallet).await;
        let relayer = load_relayer_for_interactor(&mut interactor, &common.relayer).await?;

        Ok(SystemTxSession {
            interactor,
            sender,
            relayer,
            gas_limit,
            tx_args,
        })
    }

    /// Sets the nonce, signs, and either broadcasts the transaction or writes it to the output.
    pub async fn dispatch(mut self, mut tx: Transaction) -> Result<()> {
        self.interactor.set_tx_nonce_update_sender(&mut tx).await;
        self.interactor
            .sign_and_dispatch(
                tx,
                self.tx_args.send,
                self.tx_args.wait_result,
                self.tx_args.outfile.as_deref(),
            )
            .await
    }
}

fn has_relayer(relayer: &RelayerArgs) -> bool {
    relayer.relayer.is_some() || relayer.relayer_pem.is_some() || relayer.relayer_keyfile.is_some()
}

/// Connects to the gateway, for queries. No wallet required.
pub(super) async fn create_query_interactor(gateway: &GatewayArgs) -> Result<Interactor> {
    let interactor = Interactor::empty().with_connection(&gateway.proxy).await;
    if let Some(chain_id) = &gateway.chain {
        interactor.validate_chain_id(chain_id)?;
    }
    Ok(interactor)
}

/// Parses a bech32 address given as a CLI argument.
pub(super) fn parse_address(arg_name: &str, address: &str) -> Result<Bech32Address> {
    Bech32Address::try_from_bech32_string(address.to_string())
        .map_err(|e| anyhow!("invalid {arg_name} address {address}: {e}"))
}

/// Prints a query result as pretty JSON.
pub(super) fn print_json<T: Serialize>(value: &T) -> Result<()> {
    println!("{}", Interactor::to_json_pretty(value)?);
    Ok(())
}
