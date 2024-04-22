use crate::Interactor;
use log::info;
use multiversx_sdk::data::transaction::TransactionOnNetwork;
use rand::Rng;
use std::time::{Duration, Instant};

const INITIAL_BACKOFF_DELAY: f32 = 1.4;
const MAX_RETRIES: usize = 8;
const MAX_BACKOFF_DELAY: Duration = Duration::from_secs(6);

impl Interactor {
    /// Retrieves a transaction from the network.
    pub(crate) async fn retrieve_tx_on_network(&self, tx_hash: String) -> TransactionOnNetwork {
        let mut rng = rand::thread_rng();
        let mut retries = 0;
        let mut backoff_delay = Duration::from_secs_f32(INITIAL_BACKOFF_DELAY);
        let start_time = Instant::now();

        loop {
            match self.proxy.get_transaction_info_with_results(&tx_hash).await {
                Ok(tx) => {
                    info!("Transaction retrieved successfully: {:#?}", tx);
                    return tx;
                },
                Err(err) => {
                    retries += 1;
                    if retries >= MAX_RETRIES {
                        info!("Transaction failed, max retries exceeded: {}", err);
                        println!("Transaction failed, max retries exceeded: {}", err);
                        break;
                    }

                    let backoff_time = backoff_delay
                        .mul_f32(rng.gen_range(0.8..1.2))
                        .min(MAX_BACKOFF_DELAY);
                    tokio::time::sleep(backoff_time).await;
                    backoff_delay *= 2; // exponential backoff
                },
            }
        }

        // retries have been exhausted
        let elapsed_time = start_time.elapsed();
        println!(
            "Fetching transaction failed and retries exhausted, returning default transaction. Total elapsed time: {:?}",
            elapsed_time
        );
        TransactionOnNetwork::default()
    }
}
