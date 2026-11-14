use std::process;

use super::error_message::simulate_gas_transfer_err_message;
use crate::{DispatchError, InteractorBase, interactor::interactor_dispatch::check_tx_expect};
use multiversx_sc_scenario::{
    scenario::ScenarioRunner,
    scenario_model::{TransferStep, TxExpect},
};
use multiversx_sdk::{
    chain_core::types::GasResult, data::transaction::Transaction, gateway::GatewayAsyncService,
};

impl<GatewayProxy> InteractorBase<GatewayProxy>
where
    GatewayProxy: GatewayAsyncService,
{
    /// Dispatches the transfer according to `self.dispatch_settings`, panics on any error.
    ///
    /// Returns the transaction hash.
    pub async fn transfer(&mut self, transfer_step: TransferStep) -> String {
        self.transfer_dispatch(&transfer_step)
            .await
            .unwrap_or_else(|err| panic!("transfer: {err}"))
    }

    /// Signs the transfer, then writes and/or sends it, according to `self.dispatch_settings`.
    ///
    /// If the result is awaited, the transfer is expected to succeed. Returns the transaction hash.
    pub async fn transfer_dispatch(
        &mut self,
        transfer_step: &TransferStep,
    ) -> Result<String, DispatchError> {
        let transaction = self.transfer_to_blockchain_signed_tx(transfer_step).await;

        if self.dispatch_settings.send.is_some() {
            self.pre_runners.run_transfer_step(transfer_step);
        }

        let tx_response = self.dispatch_signed_tx(&transaction, "transfer").await?;
        check_tx_expect(Some(&TxExpect::ok()), &tx_response)?;

        self.post_runners.run_transfer_step(transfer_step);

        Ok(tx_response.tx_hash.map(hex::encode).unwrap_or_default())
    }

    pub async fn simulate_gas_transfer(&mut self, transfer_step: TransferStep) -> GasResult {
        let transaction = self.transfer_to_blockchain_signed_tx(&transfer_step).await;

        match self.request_simulate_gas(&transaction).await {
            Ok(gas) => {
                println!("Gas simulation for the SC transfer: {gas}.");
                log::info!("Gas simulation for the SC transfer: {gas}.");
                gas
            }
            Err(err) => {
                simulate_gas_transfer_err_message(&err);
                process::exit(1);
            }
        }
    }

    async fn transfer_to_blockchain_signed_tx(
        &mut self,
        transfer_step: &TransferStep,
    ) -> Transaction {
        let mut transaction = self.tx_call_to_blockchain_tx(&transfer_step.tx.to_tx_call());
        self.set_tx_nonce_update_sender(&mut transaction).await;
        self.sign_tx(&mut transaction);

        transaction
    }
}
