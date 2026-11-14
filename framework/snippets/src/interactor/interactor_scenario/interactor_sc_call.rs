use std::process;

use super::error_message::simulate_gas_sc_call_err_message;
use crate::{DispatchError, InteractorBase, interactor::interactor_dispatch::check_tx_expect};
use multiversx_sc_scenario::{
    scenario::ScenarioRunner,
    scenario_model::{ScCallStep, SetStateStep, TxCall},
};
use multiversx_sdk::chain_core::std::base64_encode;
use multiversx_sdk::chain_core::types::GasResult;
use multiversx_sdk::data::transaction::Transaction;
use multiversx_sdk::data::transaction::{TransactionOptions, TransactionVersion};
use multiversx_sdk::gateway::GatewayAsyncService;

impl<GatewayProxy> InteractorBase<GatewayProxy>
where
    GatewayProxy: GatewayAsyncService,
{
    /// Dispatches the SC call according to `self.dispatch_settings`, panics on any error.
    pub async fn sc_call(&mut self, sc_call_step: &mut ScCallStep) {
        if let Err(err) = self.sc_call_dispatch(sc_call_step).await {
            panic!("SC call: {err}");
        }
    }

    /// Signs the SC call, then writes and/or sends it, according to `self.dispatch_settings`.
    ///
    /// If the result is awaited, it is checked against the step expectation,
    /// then saved in the step, for the result handlers to process.
    pub async fn sc_call_dispatch(
        &mut self,
        sc_call_step: &mut ScCallStep,
    ) -> Result<(), DispatchError> {
        let mut transaction = self.tx_call_to_blockchain_signed_tx(sc_call_step).await;
        self.apply_simulated_gas(&mut transaction, &mut sc_call_step.tx.gas_limit, "SC call")
            .await?;

        if self.dispatch_settings.send.is_some() {
            self.pre_runners.run_sc_call_step(sc_call_step);
        }

        let tx_response = self.dispatch_signed_tx(&transaction, "sc call").await?;
        check_tx_expect(sc_call_step.expect.as_ref(), &tx_response)?;
        sc_call_step.save_response(tx_response);

        if let Some(token_identifier) = sc_call_step.response().new_issued_token_identifier.clone()
        {
            println!("token identifier: {}", token_identifier);
            let set_state_step = SetStateStep::new().new_token_identifier(token_identifier);

            self.pre_runners.run_set_state_step(&set_state_step);
            self.post_runners.run_set_state_step(&set_state_step);
        }

        self.post_runners.run_sc_call_step(sc_call_step);
        Ok(())
    }

    pub async fn sc_call_simulate(&mut self, sc_call_step: &ScCallStep) -> GasResult {
        let transaction = self.tx_call_to_blockchain_signed_tx(sc_call_step).await;
        self.sc_call_simulate_transaction(&transaction).await
    }

    async fn tx_call_to_blockchain_signed_tx(&mut self, sc_call_step: &ScCallStep) -> Transaction {
        let mut transaction = self.tx_call_to_blockchain_tx(&sc_call_step.tx);
        // Set relayer field before signing — it is included in the signing bytes.
        self.set_tx_nonce_update_sender(&mut transaction).await;
        self.sign_tx(&mut transaction);

        transaction
    }

    async fn sc_call_simulate_transaction(&mut self, transaction: &Transaction) -> GasResult {
        match self.request_simulate_gas(transaction).await {
            Ok(gas) => {
                println!("Gas simulation for the SC call: {gas}.");
                log::info!("Gas simulation for the SC call: {gas}.");
                gas
            }
            Err(err) => {
                println!("Gas simulation error: {err}");
                log::error!("Gas simulation error: {err}");
                simulate_gas_sc_call_err_message(&err);
                process::exit(1)
            }
        }
    }

    pub(crate) fn tx_call_to_blockchain_tx(&self, tx_call: &TxCall) -> Transaction {
        let hrp = self.get_hrp();
        let normalized = tx_call.normalize();
        let contract_call_tx_data = normalized.compute_data_field();
        let data = if contract_call_tx_data.is_empty() {
            None
        } else {
            Some(String::from_utf8(base64_encode(contract_call_tx_data).into()).unwrap())
        };

        Transaction {
            nonce: 0,
            value: normalized.egld_value.value.to_string(),
            sender: normalized.from.to_address().to_bech32(hrp),
            receiver: normalized.to.to_address().to_bech32(hrp),
            gas_price: self.gas_price,
            gas_limit: normalized.gas_limit.value,
            data,
            signature: None,
            chain_id: self.network_config().chain_id.clone(),
            version: TransactionVersion::V2,
            options: Some(TransactionOptions::SIGN_WITH_HASH),
            relayer: normalized
                .relayer
                .as_ref()
                .map(|a| a.to_address().to_bech32(hrp)),
            relayer_signature: None,
        }
    }
}
