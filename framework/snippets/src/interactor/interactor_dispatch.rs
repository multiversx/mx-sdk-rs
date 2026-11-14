use crate::{InteractorBase, sdk::data::transaction::Transaction};
use anyhow::Context;
use multiversx_sdk::gateway::GatewayAsyncService;
use serde::Serialize;

use multiversx_sc_scenario::scenario_model::{Checkable, TxExpect, TxResponse};

use super::{
    DispatchError, ExplorerUrl, ResultHandlerError, TxDispatchSettings, TxOutputFile,
    TxOutputTarget,
};

impl<GatewayProxy> InteractorBase<GatewayProxy>
where
    GatewayProxy: GatewayAsyncService,
{
    /// Serialize a value to JSON with 4-space indentation (matching mxpy output).
    pub fn to_json_pretty<T: Serialize>(value: &T) -> anyhow::Result<String> {
        to_json_pretty(value)
    }

    fn print_tx_url(tx_hash: &str, chain_id: &str, info: &str) {
        if let Some(explorer_url) = ExplorerUrl::from_chain_id(chain_id) {
            println!("{info}: {}", explorer_url.tx_url(tx_hash));
        } else {
            println!("{info} tx hash: {tx_hash}");
        }
    }

    /// Broadcast `transaction` through the configured gateway.
    ///
    /// Returns the transaction hash reported by the gateway. On success, logs the hash and
    /// prints its explorer URL when `transaction.chain_id` is supported; otherwise prints the
    /// hash. On failure, logs and prints the gateway error before returning it with `info` as
    /// context.
    pub async fn broadcast_transaction(
        &self,
        transaction: &Transaction,
        info: &str,
    ) -> anyhow::Result<String> {
        let result = self
            .proxy()
            .request(multiversx_sdk::gateway::SendTxRequest(transaction))
            .await;
        match result {
            Ok(tx_hash) => {
                log::info!("{info} tx hash: {tx_hash}");
                Self::print_tx_url(&tx_hash, &transaction.chain_id, info);
                Ok(tx_hash)
            }
            Err(err) => {
                let error = anyhow::anyhow!("{info} error: {err}");
                eprintln!("{error}");
                log::error!("{error}");
                Err(error)
            }
        }
    }

    /// Broadcast the transaction inside `output`, update the hash (and optionally
    /// the on-network result), then write/print the updated output.
    pub async fn broadcast_and_save(
        &self,
        output: TxOutputFile,
        target: &TxOutputTarget,
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

            self.generate_blocks_until_tx_processed(&tx_hash).await?;

            let (tx_on_network, return_code) =
                multiversx_sdk::retrieve_tx_on_network(self.proxy(), tx_hash).await?;
            let tx_response =
                crate::network_response::parse_tx_response(tx_on_network.clone(), return_code);
            print_tx_results(&tx_response);
            output_with_hash.transaction_on_network = Some(tx_on_network);
        }

        Ok(output_with_hash.save_output(target)?)
    }

    /// Sign `transaction`, then write / print / broadcast it according to `settings`.
    pub async fn sign_and_dispatch(
        &self,
        mut transaction: Transaction,
        settings: &TxDispatchSettings,
    ) -> anyhow::Result<()> {
        self.sign_tx(&mut transaction);
        let output = TxOutputFile::from_transaction(transaction)?;

        match &settings.send {
            Some(send) => {
                self.broadcast_and_save(output, &settings.output, send.wait_result)
                    .await
            }
            None => Ok(output.save_output(&settings.output)?),
        }
    }

    /// Sets the nonce of `transaction` (updating the sender), then signs and dispatches it.
    pub async fn set_nonce_sign_and_dispatch(
        &mut self,
        mut transaction: Transaction,
        settings: &TxDispatchSettings,
    ) -> anyhow::Result<()> {
        self.set_tx_nonce_update_sender(&mut transaction).await;
        self.sign_and_dispatch(transaction, settings).await
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

pub(crate) fn print_tx_results(tx_response: &multiversx_sc_scenario::scenario_model::TxResponse) {
    if tx_response.tx_error.is_success() {
        println!("Transaction successful.");
    } else {
        println!("Transaction failed: {}", tx_response.tx_error);
    }
    for (index, result) in tx_response.out.iter().enumerate() {
        println!("Result[{index}]: 0x{}", hex::encode(result));
    }
}

/// Checks the response of an executed transaction against the step expectation.
///
/// - Tx failed while success was expected (the default): [`DispatchError::TxFailed`].
/// - Any other mismatch, e.g. `ExpectError` but the tx succeeded:
///   [`ResultHandlerError::ExpectationMismatch`](crate::ResultHandlerError::ExpectationMismatch).
pub(crate) fn check_tx_expect(
    expect: Option<&TxExpect>,
    tx_response: &TxResponse,
) -> Result<(), DispatchError> {
    let Some(expect) = expect else {
        return Ok(());
    };

    let Err(mismatch) = expect.check_response(tx_response) else {
        return Ok(());
    };

    // status rejected, but success would have been accepted: success was expected
    if mismatch.field == "status" && expect.status.check(0u64) {
        return Err(DispatchError::TxFailed(tx_response.tx_error.clone()));
    }

    Err(ResultHandlerError::ExpectationMismatch(mismatch).into())
}
