use crate::{InteractorBase, sdk::data::transaction::Transaction};
use anyhow::Context;
use multiversx_sdk::gateway::GatewayAsyncService;
use serde::Serialize;

use super::{ExplorerUrl, TxOutputFile};
use std::path::Path;

impl<GatewayProxy> InteractorBase<GatewayProxy>
where
    GatewayProxy: GatewayAsyncService,
{
    /// Serialize a value to JSON with 4-space indentation (matching mxpy output).
    pub fn to_json_pretty<T: Serialize>(value: &T) -> anyhow::Result<String> {
        to_json_pretty(value)
    }

    pub async fn broadcast_transaction(
        &self,
        transaction: &Transaction,
        info: &str,
    ) -> anyhow::Result<String> {
        let result = self
            .proxy()
            .request(multiversx_sdk::gateway::SendTxRequest(transaction))
            .await;
        let Ok(tx_hash) = result else {
            let Err(err) = result else {
                unreachable!();
            };
            let error = anyhow::anyhow!("{info} error: {err}");
            eprintln!("{error}");
            log::error!("{error}");
            return Err(error);
        };

        log::info!("{info} tx hash: {tx_hash}");
        if let Some(explorer_url) = ExplorerUrl::from_chain_id(&transaction.chain_id) {
            println!("{info}: {}", explorer_url.tx_url(&tx_hash));
        } else {
            println!("{info} tx hash: {tx_hash}");
        }

        Ok(tx_hash)
    }

    /// Broadcast the transaction inside `output`, update the hash (and optionally
    /// the on-network result), then write/print the updated output.
    pub async fn broadcast_and_save(
        &self,
        output: TxOutputFile,
        outfile: Option<&Path>,
        wait_result: bool,
    ) -> anyhow::Result<()> {
        if output.emitted_transaction.signature.is_none() {
            anyhow::bail!("transaction is not signed; sign it before broadcasting");
        }
        if output.emitted_transaction.relayer.is_some()
            && output.emitted_transaction.relayer_signature.is_none()
        {
            anyhow::bail!(
                "relayed transaction is missing relayer signature; use `tx relay` to add it"
            );
        }

        let tx_hash = self
            .broadcast_transaction(&output.emitted_transaction, "transaction")
            .await?;

        let mut output_with_hash = TxOutputFile {
            emitted_transaction_hash: tx_hash.clone(),
            ..output
        };

        if wait_result {
            println!("Waiting for transaction result...");
            let (tx_on_network, return_code) =
                multiversx_sdk::retrieve_tx_on_network(self.proxy(), tx_hash).await?;
            let tx_response =
                crate::network_response::parse_tx_response(tx_on_network.clone(), return_code);
            print_tx_results(&tx_response);
            output_with_hash.transaction_on_network = Some(tx_on_network);
        }

        output_with_hash.save_output(outfile)
    }

    /// Sign `transaction`, then write / print / broadcast it according to the supplied flags.
    pub async fn sign_and_dispatch(
        &self,
        mut transaction: Transaction,
        contract_address: Option<String>,
        send: bool,
        wait_result: bool,
        outfile: Option<&Path>,
    ) -> anyhow::Result<()> {
        self.sign_tx(&mut transaction);
        let output = TxOutputFile::from_transaction(transaction, contract_address)?;

        if send {
            self.broadcast_and_save(output, outfile, wait_result).await
        } else {
            output.save_output(outfile)
        }
    }
}

fn to_json_pretty<T: Serialize>(value: &T) -> anyhow::Result<String> {
    let mut buf = Vec::new();
    let formatter = serde_json::ser::PrettyFormatter::with_indent(b"    ");
    let mut serializer = serde_json::Serializer::with_formatter(&mut buf, formatter);
    value
        .serialize(&mut serializer)
        .context("failed to serialize transaction")?;
    String::from_utf8(buf).context("non-UTF8 in serialized JSON")
}

fn print_tx_results(tx_response: &multiversx_sc_scenario::scenario_model::TxResponse) {
    if tx_response.tx_error.is_success() {
        println!("Transaction successful.");
    } else {
        println!("Transaction failed: {}", tx_response.tx_error);
    }
    for (index, result) in tx_response.out.iter().enumerate() {
        println!("Result[{index}]: 0x{}", hex::encode(result));
    }
}
