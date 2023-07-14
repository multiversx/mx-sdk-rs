use crate::Interactor;
use log::info;
use multiversx_sdk::data::transaction::TransactionOnNetwork;
use std::time::Duration;

const TX_GET_RESULTS_NUM_RETRIES: usize = 8;
const EXTRA_WAITING_TIME_MS: Duration = Duration::from_millis(8000);
const WAITING_TIME_MS: Duration = Duration::from_secs(25);
const WAIT: u64 = 1000;

impl Interactor {
    /// Retrieves a transaction from the network.
    pub(crate) async fn retrieve_tx_on_network(&self, tx_hash: String) -> TransactionOnNetwork {
        let mut waiting_time_ms = 0;
        let mut break_outer = false;
        sleep(&mut waiting_time_ms, WAITING_TIME_MS).await;

        let tx = 'outer: loop {
            let mut retries = TX_GET_RESULTS_NUM_RETRIES;
            let mut wait = WAIT;
            loop {
                let tx_info_result = self.proxy.get_transaction_info_with_results(&tx_hash).await;
                match tx_info_result {
                    Ok(tx) => {
                        if break_outer {
                            break 'outer tx;
                        }

                        // reset waiting time
                        waiting_time_ms = WAITING_TIME_MS.as_millis() as u64;

                        tokio::time::sleep(EXTRA_WAITING_TIME_MS).await;
                        break_outer = true;

                        break;
                    },
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
                        sleep(&mut waiting_time_ms, Duration::from_millis(wait)).await;
                        wait *= 2;
                    },
                }
            }
        };

        info!("tx with results: {:#?}", tx);
        tx
    }
}

/// Sleeps for the given duration and adds the duration to the waiting time.
pub async fn sleep(waiting_time_ms: &mut u64, duration: Duration) {
    *waiting_time_ms += duration.as_millis() as u64;
    tokio::time::sleep(duration).await;
}
