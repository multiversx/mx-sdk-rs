use crate::{mandos_to_erdrs_address, Interactor};
use log::info;
use multiversx_sc_scenario::{
    bech32,
    mandos_system::ScenarioRunner,
    scenario_model::{ScDeployStep, SetStateStep, TxResponse},
};
use multiversx_sdk::data::{address::Address as ErdrsAddress, transaction::Transaction};

const DEPLOY_RECEIVER: [u8; 32] = [0u8; 32];

impl Interactor {
    pub(crate) fn sc_deploy_to_blockchain_tx(&self, sc_deploy_step: &ScDeployStep) -> Transaction {
        Transaction {
            nonce: 0,
            value: sc_deploy_step.tx.egld_value.value.to_string(),
            sender: mandos_to_erdrs_address(&sc_deploy_step.tx.from),
            receiver: ErdrsAddress::from_bytes(DEPLOY_RECEIVER),
            gas_price: self.network_config.min_gas_price,
            gas_limit: sc_deploy_step.tx.gas_limit.value,
            data: Some(base64::encode(sc_deploy_step.tx.to_tx_data())),
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
        let tx = self.retrieve_tx_on_network(tx_hash.clone()).await;

        let addr = sc_deploy_step.tx.from.clone();
        let nonce = tx.nonce;
        sc_deploy_step.save_response(TxResponse::from_network_tx(tx));

        let deploy_address = sc_deploy_step
            .response()
            .new_deployed_address
            .clone()
            .unwrap();

        let set_state_step = SetStateStep::new().new_address(
            addr,
            nonce,
            format!("0x{}", hex::encode(&deploy_address)).as_str(),
        );

        println!("deploy address: {}", bech32::encode(&deploy_address));
        self.pre_runners.run_set_state_step(&set_state_step);
        self.post_runners.run_set_state_step(&set_state_step);

        self.post_runners.run_sc_deploy_step(sc_deploy_step);
    }
}
