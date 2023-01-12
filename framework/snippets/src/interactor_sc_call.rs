use crate::{address_h256_to_erdrs, mandos_to_erdrs_address, Interactor, InteractorResult};
use log::info;
use multiversx_sc_scenario::{
    multiversx_sc::{
        codec::{multi_types::IgnoreValue, CodecFrom, TopEncodeMulti},
        types::ContractCallWithEgld,
    },
    scenario_model::{ScCallStep, TransferStep, TxCall, TypedScCall},
    DebugApi,
};
use multiversx_sdk::data::transaction::Transaction;

fn contract_call_to_tx_data(contract_call: &ContractCallWithEgld<DebugApi, ()>) -> String {
    let mut result = String::from_utf8(
        contract_call
            .basic
            .endpoint_name
            .to_boxed_bytes()
            .into_vec(),
    )
    .unwrap();
    for argument in contract_call.basic.arg_buffer.raw_arg_iter() {
        result.push('@');
        result.push_str(hex::encode(argument.to_boxed_bytes().as_slice()).as_str());
    }
    result
}

impl Interactor {
    fn tx_call_to_blockchain_tx(&self, tx_call: &TxCall) -> Transaction {
        let contract_call = tx_call.to_contract_call();
        let contract_call_tx_data = contract_call_to_tx_data(&contract_call);
        let data = if contract_call_tx_data.is_empty() {
            None
        } else {
            Some(base64::encode(contract_call_tx_data))
        };

        Transaction {
            nonce: 0,
            value: contract_call.egld_payment.to_alloc().to_string(),
            sender: mandos_to_erdrs_address(&tx_call.from),
            receiver: address_h256_to_erdrs(&contract_call.basic.to.to_address()),
            gas_price: self.network_config.min_gas_price,
            gas_limit: tx_call.gas_limit.value,
            data,
            signature: None,
            chain_id: self.network_config.chain_id.clone(),
            version: self.network_config.min_transaction_version,
            options: 0,
        }
    }

    pub async fn sc_call<S>(&mut self, sc_call_step: S) -> String
    where
        ScCallStep: From<S>,
    {
        let sc_call_step: ScCallStep = sc_call_step.into();
        let sender_address = &sc_call_step.tx.from.value;
        let mut transaction = self.tx_call_to_blockchain_tx(&sc_call_step.tx);
        self.set_nonce_and_sign_tx(sender_address, &mut transaction)
            .await;
        let tx_hash = self.proxy.send_transaction(&transaction).await.unwrap();
        println!("sc call tx hash: {tx_hash}");
        info!("sc call tx hash: {}", tx_hash);
        tx_hash
    }

    pub async fn sc_call_get_result<OriginalResult, RequestedResult>(
        &mut self,
        typed_sc_call: TypedScCall<OriginalResult>,
    ) -> InteractorResult<RequestedResult>
    where
        OriginalResult: TopEncodeMulti,
        RequestedResult: CodecFrom<OriginalResult>,
    {
        let tx_hash = self.sc_call(typed_sc_call).await;
        let tx = self.retrieve_tx_on_network(tx_hash.as_str()).await;
        InteractorResult::new(tx)
    }

    pub async fn sc_call_get_raw_result(
        &mut self,
        sc_call_step: ScCallStep,
    ) -> InteractorResult<IgnoreValue> {
        let tx_hash = self.sc_call(sc_call_step).await;
        let tx = self.retrieve_tx_on_network(tx_hash.as_str()).await;
        InteractorResult::new(tx)
    }

    pub async fn multiple_sc_calls(&mut self, sc_call_steps: &[ScCallStep]) {
        let sender_address = &sc_call_steps.get(0).unwrap().tx.from.value;
        for sc_call_step in sc_call_steps {
            assert_eq!(
                &sc_call_step.tx.from.value, sender_address,
                "all calls are expected to have the same sender"
            );
            let mut transaction = self.tx_call_to_blockchain_tx(&sc_call_step.tx);
            self.set_nonce_and_sign_tx(sender_address, &mut transaction)
                .await;
            let _ = self.proxy.send_transaction(&transaction).await.unwrap();
        }
    }

    pub async fn transfer(&mut self, transfer_step: TransferStep) -> String {
        let sender_address = &transfer_step.tx.from.value;
        let mut transaction = self.tx_call_to_blockchain_tx(&transfer_step.tx.to_tx_call());
        self.set_nonce_and_sign_tx(sender_address, &mut transaction)
            .await;
        let tx_hash = self.proxy.send_transaction(&transaction).await.unwrap();
        println!("transfer tx hash: {tx_hash}");
        info!("transfer tx hash: {}", tx_hash);
        tx_hash
    }

    pub async fn transfer_get_raw_result(
        &mut self,
        transfer_step: TransferStep,
    ) -> InteractorResult<IgnoreValue> {
        let tx_hash = self.transfer(transfer_step).await;
        let tx = self.retrieve_tx_on_network(tx_hash.as_str()).await;
        InteractorResult::new(tx)
    }
}
