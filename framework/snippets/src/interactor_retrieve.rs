use crate::Interactor;
use log::info;
use multiversx_sdk::data::transaction::TransactionOnNetwork;
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
                        "still no answer after {TX_GET_RESULTS_NUM_RETRIES} retries"
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
}
