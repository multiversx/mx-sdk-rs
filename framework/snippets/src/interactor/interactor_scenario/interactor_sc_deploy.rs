use std::process;

use super::error_message::{deploy_err_message, simulate_gas_deploy_err_message};
use crate::{network_response, InteractorBase, SimulateGas};
use multiversx_sc_scenario::{
    imports::Bech32Address,
    mandos_system::ScenarioRunner,
    scenario_model::{ScDeployStep, SetStateStep},
};
use multiversx_sdk::{
    data::transaction::Transaction, gateway::SimulateTxRequest, utils::base64_encode,
};
use multiversx_sdk::{
    gateway::{GatewayAsyncService, SendTxRequest},
    retrieve_tx_on_network,
};

impl<GatewayProxy> InteractorBase<GatewayProxy>
where
    GatewayProxy: GatewayAsyncService,
{
    pub(crate) fn sc_deploy_to_blockchain_tx(&self, sc_deploy_step: &ScDeployStep) -> Transaction {
        let hrp = self.network_config.address_hrp.clone();

        Transaction {
            nonce: 0,
            value: sc_deploy_step.tx.egld_value.value.to_string(),
            sender: sc_deploy_step.tx.from.to_address().to_bech32(&hrp),
            receiver: Bech32Address::zero(&hrp),
            gas_price: self.network_config.min_gas_price,
            gas_limit: sc_deploy_step.tx.gas_limit.value,
            data: Some(base64_encode(sc_deploy_step.tx.to_tx_data())),
            signature: None,
            chain_id: self.network_config.chain_id.clone(),
            version: self.network_config.min_transaction_version,
            options: 0,
        }
    }

    async fn sc_deploy_to_blockchain_signed_tx(
        &mut self,
        sc_deploy_step: &ScDeployStep,
    ) -> Transaction {
        let sender_address = &sc_deploy_step.tx.from.value;
        let mut transaction = self.sc_deploy_to_blockchain_tx(sc_deploy_step);
        self.set_tx_nonce_update_sender(sender_address, &mut transaction)
            .await;
        self.sign_tx(sender_address, &mut transaction);

        transaction
    }

    async fn launch_sc_deploy(&mut self, transaction: &Transaction) -> String {
        let tx_hash_result = self.proxy.request(SendTxRequest(transaction)).await;

        match tx_hash_result {
            Ok(tx_hash) => {
                println!("sc deploy tx hash: {tx_hash}");
                log::info!("sc deploy tx hash: {tx_hash}");
                tx_hash
            }
            Err(err) => {
                println!("sc deploy error: {err}");
                log::error!("sc deploy error: {err}");
                deploy_err_message(&err);
                process::exit(1)
            }
        }
    }

    pub async fn sc_deploy(&mut self, sc_deploy_step: &mut ScDeployStep) {
        let mut transaction = self.sc_deploy_to_blockchain_signed_tx(sc_deploy_step).await;

        if SimulateGas::is_mandos_simulate_gas_marker(&sc_deploy_step.tx.gas_limit) {
            let sim_gas = self.sc_deploy_simulate_transaction(&transaction).await;
            let gas = SimulateGas::adjust_simulated_gas(sim_gas);
            sc_deploy_step.tx.gas_limit = gas.into();
            transaction.gas_limit = gas;

            // sign again, because gas changed
            let sender_address = &sc_deploy_step.tx.from.value;
            self.sign_tx(sender_address, &mut transaction);
        }

        self.pre_runners.run_sc_deploy_step(sc_deploy_step);

        let tx_hash = self.launch_sc_deploy(&transaction).await;

        self.generate_blocks_until_tx_processed(&tx_hash)
            .await
            .unwrap();
        let (tx, return_code) = retrieve_tx_on_network(&self.proxy, tx_hash.clone()).await;

        let addr = sc_deploy_step.tx.from.clone();
        let nonce = tx.nonce;
        sc_deploy_step.save_response(network_response::parse_tx_response(tx, return_code));

        let deploy_address = sc_deploy_step
            .response()
            .new_deployed_address
            .clone()
            .unwrap();
        let deploy_address_bech32 = Bech32Address::encode_address(self.get_hrp(), deploy_address);

        let set_state_step = SetStateStep::new().new_address(addr, nonce, &deploy_address_bech32);

        println!("deploy address: {deploy_address_bech32}");
        self.pre_runners.run_set_state_step(&set_state_step);
        self.post_runners.run_set_state_step(&set_state_step);

        self.post_runners.run_sc_deploy_step(sc_deploy_step);
    }

    async fn sc_deploy_simulate_transaction(&mut self, transaction: &Transaction) -> u64 {
        let gas_result = self.proxy.request(SimulateTxRequest(transaction)).await;

        match gas_result {
            Ok(gas) => {
                println!("Gas simulation for the SC deploy: {gas} units.");
                log::info!("Gas simulation for the SC deploy: {gas} units.");
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

    pub async fn sc_deploy_simulate(&mut self, sc_deploy_step: &ScDeployStep) -> u64 {
        let transaction = self.sc_deploy_to_blockchain_signed_tx(sc_deploy_step).await;
        self.sc_deploy_simulate_transaction(&transaction).await
    }
}
