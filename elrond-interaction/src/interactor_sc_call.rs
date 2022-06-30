use crate::{scr_decode::decode_scr_data_or_panic, Interactor, TX_GET_RESULTS_NUM_RETRIES};
use elrond_wasm_debug::{
    elrond_wasm::elrond_codec::{CodecFrom, PanicErrorHandler, TopEncodeMulti},
    mandos_system::model::{ScCallStep, TypedScCall},
};
use log::info;
use std::time::Duration;

impl Interactor {
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
