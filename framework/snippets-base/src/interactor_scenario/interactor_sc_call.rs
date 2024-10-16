use crate::{network_response, InteractorBase};
use log::info;
use multiversx_sc_scenario::{
    api::StaticApi,
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
        let tx_hash = self.launch_sc_call(sc_call_step).await;
        self.generate_blocks_until_tx_processed(&tx_hash)
            .await
            .unwrap();
        let tx = retrieve_tx_on_network(&self.proxy, tx_hash.clone()).await;

        sc_call_step.save_response(network_response::parse_tx_response(tx));

        if let Some(token_identifier) = sc_call_step.response().new_issued_token_identifier.clone()
        {
            println!("token identifier: {}", token_identifier);
            let set_state_step = SetStateStep::new().new_token_identifier(token_identifier);

            self.pre_runners.run_set_state_step(&set_state_step);
            self.post_runners.run_set_state_step(&set_state_step);
        }

        self.post_runners.run_sc_call_step(sc_call_step);
    }

    async fn launch_sc_call(&mut self, sc_call_step: &mut ScCallStep) -> String {
        self.pre_runners.run_sc_call_step(sc_call_step);

        let sender_address = &sc_call_step.tx.from.value;
        let mut transaction = self.tx_call_to_blockchain_tx(&sc_call_step.tx);
        self.set_nonce_and_sign_tx(sender_address, &mut transaction)
            .await;
        let tx_hash = self
            .proxy
            .request(SendTxRequest(&transaction))
            .await
            .unwrap();
        println!("sc call tx hash: {tx_hash}");
        info!("sc call tx hash: {}", tx_hash);

        tx_hash
    }

    #[allow(deprecated)] // TODO
    pub(crate) fn tx_call_to_blockchain_tx(&self, tx_call: &TxCall) -> Transaction {
        let contract_call = tx_call.to_contract_call();
        let contract_call_tx_data = contract_call_to_tx_data(&contract_call);
        let data = if contract_call_tx_data.is_empty() {
            None
        } else {
            Some(String::from_utf8(base64_encode(contract_call_tx_data).into()).unwrap())
        };

        Transaction {
            nonce: 0,
            value: contract_call.egld_payment.to_alloc().to_string(),
            sender: tx_call.from.to_address().into(),
            receiver: contract_call.basic.to.to_address().into(),
            gas_price: self.network_config.min_gas_price,
            gas_limit: tx_call.gas_limit.value,
            data,
            signature: None,
            chain_id: self.network_config.chain_id.clone(),
            version: self.network_config.min_transaction_version,
            options: 0,
        }
    }
}

#[allow(deprecated)] // TODO
fn contract_call_to_tx_data(
    contract_call: &multiversx_sc_scenario::imports::ContractCallWithEgld<StaticApi, ()>,
) -> String {
    let mut result = String::from_utf8(
        contract_call
            .basic
            .function_call
            .function_name
            .to_boxed_bytes()
            .into_vec(),
    )
    .unwrap();
    for argument in contract_call.basic.function_call.arg_buffer.raw_arg_iter() {
        result.push('@');
        result.push_str(hex::encode(argument.to_boxed_bytes().as_slice()).as_str());
    }
    result
}
