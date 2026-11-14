use std::process;

use super::error_message::simulate_gas_deploy_err_message;
use crate::{DispatchError, InteractorBase, interactor::interactor_dispatch::check_tx_expect};
use multiversx_sc_scenario::{
    imports::Bech32Address,
    mandos_system::ScenarioRunner,
    scenario_model::{ScDeployStep, SetStateStep},
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
    pub(crate) fn sc_deploy_to_blockchain_tx(&self, sc_deploy_step: &ScDeployStep) -> Transaction {
        let hrp = self.network_config().address_hrp;

        Transaction {
            nonce: 0,
            value: sc_deploy_step.tx.egld_value.value.to_string(),
            sender: sc_deploy_step.tx.from.to_address().to_bech32(hrp),
            receiver: Bech32Address::zero(hrp),
            gas_price: self.gas_price,
            gas_limit: sc_deploy_step.tx.gas_limit.value,
            data: Some(base64_encode(sc_deploy_step.tx.to_tx_data())),
            signature: None,
            chain_id: self.network_config().chain_id.clone(),
            version: TransactionVersion::V2,
            options: Some(TransactionOptions::SIGN_WITH_HASH),
            relayer: sc_deploy_step
                .tx
                .relayer
                .as_ref()
                .map(|a| a.to_address().to_bech32(hrp)),
            relayer_signature: None,
        }
    }

    async fn sc_deploy_to_blockchain_signed_tx(
        &mut self,
        sc_deploy_step: &ScDeployStep,
    ) -> Transaction {
        let mut transaction = self.sc_deploy_to_blockchain_tx(sc_deploy_step);
        // Set relayer field before signing — it is included in the signing bytes.
        self.set_tx_nonce_update_sender(&mut transaction).await;
        self.sign_tx(&mut transaction);

        transaction
    }

    /// Dispatches the SC deploy according to `self.dispatch_settings`, panics on any error.
    pub async fn sc_deploy(&mut self, sc_deploy_step: &mut ScDeployStep) {
        if let Err(err) = self.sc_deploy_dispatch(sc_deploy_step).await {
            panic!("SC deploy: {err}");
        }
    }

    /// Signs the SC deploy, then writes and/or sends it, according to `self.dispatch_settings`.
    ///
    /// If the result is awaited, it is checked against the step expectation,
    /// then saved in the step, for the result handlers to process.
    pub async fn sc_deploy_dispatch(
        &mut self,
        sc_deploy_step: &mut ScDeployStep,
    ) -> Result<(), DispatchError> {
        let mut transaction = self.sc_deploy_to_blockchain_signed_tx(sc_deploy_step).await;
        self.apply_simulated_gas(
            &mut transaction,
            &mut sc_deploy_step.tx.gas_limit,
            "SC deploy",
        )
        .await?;

        if self.dispatch_settings.send.is_some() {
            self.pre_runners.run_sc_deploy_step(sc_deploy_step);
        }

        let tx_response = self.dispatch_signed_tx(&transaction, "sc deploy").await?;
        check_tx_expect(sc_deploy_step.expect.as_ref(), &tx_response)?;
        sc_deploy_step.save_response(tx_response);

        // missing if the deploy failed, and a result handler accepted that
        if let Some(new_address) = sc_deploy_step.response().new_deployed_address.clone() {
            let new_address_bech32 = Bech32Address::encode_address(self.get_hrp(), new_address);
            if let Some(ex) = &self.explorer_url {
                println!("new contract: {}", ex.address_url(&new_address_bech32));
            } else {
                println!("new contract address: {new_address_bech32}");
            }

            let set_state_step = SetStateStep::new().new_address(
                sc_deploy_step.tx.from.clone(),
                transaction.nonce,
                &new_address_bech32,
            );
            self.pre_runners.run_set_state_step(&set_state_step);
            self.post_runners.run_set_state_step(&set_state_step);
        }

        self.post_runners.run_sc_deploy_step(sc_deploy_step);
        Ok(())
    }

    async fn sc_deploy_simulate_transaction(&mut self, transaction: &Transaction) -> GasResult {
        match self.request_simulate_gas(transaction).await {
            Ok(gas) => {
                println!("Gas simulation for the SC deploy: {gas}.");
                log::info!("Gas simulation for the SC deploy: {gas}.");
                gas
            }
            Err(err) => {
                println!("Gas simulation error: {err}");
                log::error!("Gas simulation error: {err}");
                simulate_gas_deploy_err_message(&err);
                process::exit(1)
            }
        }
    }

    pub async fn sc_deploy_simulate(&mut self, sc_deploy_step: &ScDeployStep) -> GasResult {
        let transaction = self.sc_deploy_to_blockchain_signed_tx(sc_deploy_step).await;
        self.sc_deploy_simulate_transaction(&transaction).await
    }
}
