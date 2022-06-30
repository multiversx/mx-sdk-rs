use crate::{
    mandos_to_erdrs_address, scr_decode::decode_scr_data_or_panic, Interactor,
    TX_GET_RESULTS_NUM_RETRIES,
};
use elrond_sdk_erdrs::data::transaction::Transaction;
use elrond_wasm_debug::{
    elrond_wasm::elrond_codec::{CodecFrom, PanicErrorHandler, TopEncodeMulti},
    mandos_system::model::{ScCallStep, TypedScCall},
};
use log::info;
use std::time::Duration;

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

    pub async fn send_sc_call(&mut self, sc_call_step: ScCallStep) -> String {
        let sender_address = &sc_call_step.tx.from.value;
        let mut transaction = self.sc_call_to_tx(&sc_call_step);
        transaction.nonce = self.recall_nonce(sender_address).await;

        let wallet = self
            .signing_wallets
            .get(sender_address)
            .expect("the wallet that was supposed to sign is not registered");

        let signature = wallet.sign_tx(&transaction);
        transaction.signature = Some(hex::encode(signature));
        info!("transaction {:#?}", transaction);

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
        println!("tx_hash {}", tx_hash);
        info!("tx_hash {}", tx_hash);

        self.waiting_time_ms = 0;
        self.sleep(Duration::from_secs(25)).await;

        let mut retries = TX_GET_RESULTS_NUM_RETRIES;
        let mut wait = 1000u64;
        let tx = loop {
            let tx_info_result = self
                .proxy
                .get_transaction_info_with_results(tx_hash.as_str())
                .await;
            match tx_info_result {
                Ok(tx) => break tx,
                Err(err) => {
                    assert!(
                        retries > 0,
                        "still no answer after {} retries",
                        TX_GET_RESULTS_NUM_RETRIES
                    );

                    info!(
                        "tx result fetch error after {} ms: {}",
                        self.waiting_time_ms, err
                    );
                    retries -= 1;
                    self.sleep(Duration::from_millis(wait)).await;
                    wait *= 2;
                },
            }
        };

        info!("tx with results: {:#?}", tx);

        let scrs = tx
            .smart_contract_results
            .expect("no smart contract results obtained");
        let first_scr = scrs.get(0).expect("no smart contract results obtained");

        let mut raw_result = decode_scr_data_or_panic(first_scr.data.as_str());
        RequestedResult::multi_decode_or_handle_err(&mut raw_result, PanicErrorHandler).unwrap()
    }
}
