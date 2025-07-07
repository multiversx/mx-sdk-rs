use std::process;

use super::error_message::deploy_err_message;
use crate::{network_response, InteractorBase};
use anyhow::Error;
use log::info;
use multiversx_sc_scenario::{
    imports::Bech32Address,
    mandos_system::ScenarioRunner,
    scenario_model::{ScDeployStep, SetStateStep},
};
use multiversx_sdk::{data::transaction::Transaction, utils::base64_encode};
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

    pub async fn launch_sc_deploy(
        &mut self,
        sc_deploy_step: &mut ScDeployStep,
    ) -> Result<String, Error> {
        self.pre_runners.run_sc_deploy_step(sc_deploy_step);

        let sender_address = &sc_deploy_step.tx.from.value;
        let mut transaction = self.sc_deploy_to_blockchain_tx(sc_deploy_step);
        self.set_nonce_and_sign_tx(sender_address, &mut transaction)
            .await;
        let tx_hash = self.proxy.request(SendTxRequest(&transaction)).await;

        if let Ok(tx_hash) = tx_hash.as_ref() {
            println!("sc deploy tx hash: {tx_hash}");
            info!("sc deploy tx hash: {tx_hash}");
        }

        tx_hash
    }

    pub async fn sc_deploy<S>(&mut self, mut sc_deploy_step: S)
    where
        S: AsMut<ScDeployStep>,
    {
        let sc_deploy_step = sc_deploy_step.as_mut();
        let tx_hash = match self.launch_sc_deploy(sc_deploy_step).await {
            Ok(hash) => hash,
            Err(err) => {
                deploy_err_message(&err);
                process::exit(1);
            },
        };

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
}
