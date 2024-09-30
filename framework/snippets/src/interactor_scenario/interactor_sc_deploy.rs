use crate::{network_response, Interactor};
use log::info;
use multiversx_sc_scenario::{
    imports::{Address, Bech32Address},
    mandos_system::ScenarioRunner,
    scenario_model::{ScDeployStep, SetStateStep},
};
use multiversx_sdk::{data::transaction::Transaction, utils::base64_encode};

impl Interactor {
    pub(crate) fn sc_deploy_to_blockchain_tx(&self, sc_deploy_step: &ScDeployStep) -> Transaction {
        Transaction {
            nonce: 0,
            value: sc_deploy_step.tx.egld_value.value.to_string(),
            sender: sc_deploy_step.tx.from.to_address().into(),
            receiver: Address::zero().into(),
            gas_price: self.network_config.min_gas_price,
            gas_limit: sc_deploy_step.tx.gas_limit.value,
            data: Some(base64_encode(sc_deploy_step.tx.to_tx_data())),
            signature: None,
            chain_id: self.network_config.chain_id.clone(),
            version: self.network_config.min_transaction_version,
            options: 0,
        }
    }

    pub async fn launch_sc_deploy(&mut self, sc_deploy_step: &mut ScDeployStep) -> String {
        self.pre_runners.run_sc_deploy_step(sc_deploy_step);

        let sender_address = &sc_deploy_step.tx.from.value;
        let mut transaction = self.sc_deploy_to_blockchain_tx(sc_deploy_step);
        self.set_nonce_and_sign_tx(sender_address, &mut transaction)
            .await;
        let tx_hash = self
            .proxy
            .send_transaction(&transaction)
            .await
            .expect("error sending tx (possible API failure)");
        println!("sc deploy tx hash: {tx_hash}");
        info!("sc deploy tx hash: {}", tx_hash);

        tx_hash
    }

    pub async fn sc_deploy<S>(&mut self, mut sc_deploy_step: S)
    where
        S: AsMut<ScDeployStep>,
    {
        let sc_deploy_step = sc_deploy_step.as_mut();
        let tx_hash = self.launch_sc_deploy(sc_deploy_step).await;
        let tx = self.proxy.retrieve_tx_on_network(tx_hash.clone()).await;

        let addr = sc_deploy_step.tx.from.clone();
        let nonce = tx.nonce;
        sc_deploy_step.save_response(network_response::parse_tx_response(tx));

        let deploy_address = sc_deploy_step
            .response()
            .new_deployed_address
            .clone()
            .unwrap();
        let deploy_address_bech32 = Bech32Address::from(deploy_address);

        let set_state_step = SetStateStep::new().new_address(addr, nonce, &deploy_address_bech32);

        println!("deploy address: {deploy_address_bech32}");
        self.pre_runners.run_set_state_step(&set_state_step);
        self.post_runners.run_set_state_step(&set_state_step);

        self.post_runners.run_sc_deploy_step(sc_deploy_step);
    }
}
