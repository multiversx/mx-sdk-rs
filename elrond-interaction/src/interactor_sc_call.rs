use crate::{mandos_to_erdrs_address, Interactor};
use elrond_sdk_erdrs::data::transaction::Transaction;
use elrond_wasm_debug::{
    elrond_wasm::{
        elrond_codec::{CodecFrom, TopEncodeMulti},
        types::Address,
    },
    mandos_system::model::{ScCallStep, TypedScCall},
};
use log::info;

impl Interactor {
    fn sc_call_to_tx(&self, sc_call_step: &ScCallStep) -> Transaction {
        Transaction {
            nonce: 0,
            value: sc_call_step.tx.egld_value.value.to_string(),
            sender: mandos_to_erdrs_address(&sc_call_step.tx.from),
            receiver: mandos_to_erdrs_address(&sc_call_step.tx.to),
            gas_price: self.network_config.min_gas_price,
            gas_limit: sc_call_step.tx.gas_limit.value,
            data: Some(base64::encode(sc_call_step.tx.to_tx_data())),
            signature: None,
            chain_id: self.network_config.chain_id.clone(),
            version: self.network_config.min_transaction_version,
            options: 0,
        }
    }

    pub(crate) fn sign_tx(&self, sender_address: &Address, transaction: &mut Transaction) {
        let wallet = self
            .signing_wallets
            .get(sender_address)
            .expect("the wallet that was supposed to sign is not registered");

        let signature = wallet.sign_tx(transaction);
        transaction.signature = Some(hex::encode(signature));
        info!("transaction {:#?}", transaction);
    }

    pub async fn send_sc_call(&mut self, sc_call_step: ScCallStep) -> String {
        let sender_address = &sc_call_step.tx.from.value;
        let mut transaction = self.sc_call_to_tx(&sc_call_step);
        transaction.nonce = self.recall_nonce(sender_address).await;
        self.sign_tx(sender_address, &mut transaction);
        self.proxy.send_transaction(&transaction).await.unwrap()
    }

    pub async fn sc_call<OriginalResult, RequestedResult>(
        &mut self,
        typed_sc_call: TypedScCall<OriginalResult>,
    ) -> RequestedResult
    where
        OriginalResult: TopEncodeMulti,
        RequestedResult: CodecFrom<OriginalResult>,
    {
        let sc_call_step: ScCallStep = typed_sc_call.into();
        let tx_hash = self.send_sc_call(sc_call_step).await;
        println!("sc call tx hash: {}", tx_hash);
        info!("sc call tx hash: {}", tx_hash);
        let tx = self.retrieve_tx_on_network(tx_hash.as_str()).await;
        self.extract_sc_call_result(tx)
    }
}
