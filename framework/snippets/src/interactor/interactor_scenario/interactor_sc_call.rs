use std::process;

use super::error_message::simulate_gas_sc_call_err_message;
use crate::{
    DispatchError, InteractorBase, SimulateGas, TxInterrupted, TxOutputFile, TxOutputTarget,
    interactor::interactor_dispatch::check_tx_expect, network_response,
};
use multiversx_sc_scenario::{
    scenario::ScenarioRunner,
    scenario_model::{ScCallStep, SetStateStep, TxCall},
};
use multiversx_sdk::chain_core::std::base64_encode;
use multiversx_sdk::chain_core::types::GasResult;
use multiversx_sdk::data::transaction::{TransactionOptions, TransactionVersion};
use multiversx_sdk::{data::transaction::Transaction, gateway::SimulateTxRequest};
use multiversx_sdk::{gateway::GatewayAsyncService, retrieve_tx_on_network};

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

        if SimulateGas::is_mandos_simulate_gas_marker(&sc_call_step.tx.gas_limit) {
            let sim_result = self
                .request_simulate_gas(&transaction)
                .await
                .map_err(DispatchError::Gateway)?;
            println!("Gas simulation for the SC call: {sim_result}.");
            let Some(sim_gas) = sim_result.gas_used() else {
                return Err(DispatchError::Simulate(sim_result));
            };
            let gas = SimulateGas::adjust_simulated_gas(sim_gas);
            sc_call_step.tx.gas_limit = gas.into();
            transaction.gas_limit = gas;

            // sign again, because gas changed
            self.sign_tx(&mut transaction);
        }

        let settings = self.dispatch_settings.clone();
        let mut output =
            TxOutputFile::from_transaction(transaction.clone()).map_err(std::io::Error::other)?;

        let Some(send) = &settings.send else {
            output.save_output(&settings.output)?;
            return Err(TxInterrupted {
                output: settings.output,
                tx_hash: None,
            }
            .into());
        };

        // the signed tx is kept, even if broadcasting fails
        if matches!(settings.output, TxOutputTarget::File(_)) {
            output.write_output(&settings.output)?;
        }

        self.pre_runners.run_sc_call_step(sc_call_step);

        let tx_hash = self
            .broadcast_transaction(&transaction, "sc call")
            .await
            .map_err(DispatchError::Gateway)?;
        output.emitted_transaction_hash = tx_hash.clone();

        if !send.wait_result {
            output.save_output(&settings.output)?;
            return Err(TxInterrupted {
                output: settings.output,
                tx_hash: Some(tx_hash),
            }
            .into());
        }

        self.generate_blocks_until_tx_processed(&tx_hash)
            .await
            .map_err(DispatchError::Gateway)?;
        let (tx_on_network, return_code) = retrieve_tx_on_network(self.proxy(), tx_hash)
            .await
            .map_err(DispatchError::Gateway)?;
        output.transaction_on_network = Some(tx_on_network.clone());
        output.save_output(&settings.output)?;

        let tx_response = network_response::parse_tx_response(tx_on_network, return_code);
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

    async fn request_simulate_gas(&self, transaction: &Transaction) -> anyhow::Result<GasResult> {
        self.proxy().request(SimulateTxRequest(transaction)).await
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
