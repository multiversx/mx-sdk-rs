use std::process;

use super::error_message::sc_call_err_message;
use crate::{network_response, InteractorBase};
use anyhow::Error;
use multiversx_sc_scenario::{
    imports::Bech32Address,
    scenario::ScenarioRunner,
    scenario_model::{ScCallStep, SetStateStep, TxCall},
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
    pub async fn sc_call<S>(&mut self, mut sc_call_step: S)
    where
        S: AsMut<ScCallStep>,
    {
        let sc_call_step = sc_call_step.as_mut();
        let tx_hash = match self.launch_sc_call(sc_call_step).await {
            Ok(hash) => hash,
            Err(err) => {
                sc_call_err_message(&err);
                process::exit(1);
            }
        };

        self.generate_blocks_until_tx_processed(&tx_hash)
            .await
            .unwrap();
        let (tx, return_code) = retrieve_tx_on_network(&self.proxy, tx_hash).await;

        sc_call_step.save_response(network_response::parse_tx_response(tx, return_code));

        if let Some(token_identifier) = sc_call_step.response().new_issued_token_identifier.clone()
        {
            println!("token identifier: {}", token_identifier);
            let set_state_step = SetStateStep::new().new_token_identifier(token_identifier);

            self.pre_runners.run_set_state_step(&set_state_step);
            self.post_runners.run_set_state_step(&set_state_step);
        }

        self.post_runners.run_sc_call_step(sc_call_step);
    }

    async fn launch_sc_call(&mut self, sc_call_step: &mut ScCallStep) -> Result<String, Error> {
        self.pre_runners.run_sc_call_step(sc_call_step);

        let sender_address = &sc_call_step.tx.from.value;
        let mut transaction = self.tx_call_to_blockchain_tx(&sc_call_step.tx);
        self.set_nonce_and_sign_tx(sender_address, &mut transaction)
            .await;
        let tx_hash = self.proxy.request(SendTxRequest(&transaction)).await;

        match tx_hash.as_ref() {
            Ok(tx_hash) => {
                println!("sc call tx hash: {tx_hash}");
                log::info!("sc call tx hash: {tx_hash}");
            }
            Err(err) => {
                println!("sc call error: {err}");
                log::error!("sc call error: {err}");
            }
        }

        tx_hash
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
            sender: Bech32Address::encode_address(hrp, normalized.from.to_address()),
            receiver: Bech32Address::encode_address(hrp, normalized.to.to_address()),
            gas_price: self.network_config.min_gas_price,
            gas_limit: normalized.gas_limit.value,
            data,
            signature: None,
            chain_id: self.network_config.chain_id.clone(),
            version: self.network_config.min_transaction_version,
            options: 0,
        }
    }
}
