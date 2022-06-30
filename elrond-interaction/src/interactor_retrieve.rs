use crate::{scr_decode::decode_scr_data_or_panic, Interactor};
use elrond_sdk_erdrs::data::transaction::TransactionOnNetwork;
use elrond_wasm_debug::elrond_wasm::elrond_codec::{PanicErrorHandler, TopDecodeMulti};
use log::info;
use std::time::Duration;

const TX_GET_RESULTS_NUM_RETRIES: usize = 8;

impl Interactor {
    pub(crate) async fn retrieve_tx_on_network(&mut self, tx_hash: &str) -> TransactionOnNetwork {
        self.waiting_time_ms = 0;
        self.sleep(Duration::from_secs(25)).await;

        let mut retries = TX_GET_RESULTS_NUM_RETRIES;
        let mut wait = 1000u64;
        let tx = loop {
            let tx_info_result = self.proxy.get_transaction_info_with_results(tx_hash).await;
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
        tx
    }

    pub(crate) fn extract_sc_call_result<RequestedResult>(
        &mut self,
        tx: TransactionOnNetwork,
    ) -> RequestedResult
    where
        RequestedResult: TopDecodeMulti,
    {
        let scrs = tx
            .smart_contract_results
            .expect("no smart contract results obtained");
        let first_scr = scrs.get(0).expect("no smart contract results obtained");

        let mut raw_result = decode_scr_data_or_panic(first_scr.data.as_str());
        RequestedResult::multi_decode_or_handle_err(&mut raw_result, PanicErrorHandler).unwrap()
    }
}
