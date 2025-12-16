use std::process;

use super::error_message::{simulate_gas_transfer_err_message, transfer_err_message};
use crate::InteractorBase;
use log::info;
use multiversx_sc_scenario::{scenario::ScenarioRunner, scenario_model::TransferStep};
use multiversx_sdk::{
    data::transaction::Transaction,
    gateway::{GatewayAsyncService, SendTxRequest, SimulateTxRequest},
    retrieve_tx_on_network,
};

impl<GatewayProxy> InteractorBase<GatewayProxy>
where
    GatewayProxy: GatewayAsyncService,
{
    pub async fn transfer(&mut self, transfer_step: TransferStep) -> String {
        let transaction = self.launch_transfer(&transfer_step).await;

        let tx_hash = match self.proxy.request(SendTxRequest(&transaction)).await {
            Ok(hash) => hash,
            Err(err) => {
                transfer_err_message(&err);
                process::exit(1);
            }
        };
        self.generate_blocks_until_tx_processed(&tx_hash)
            .await
            .unwrap();

        println!("transfer tx hash: {tx_hash}");
        info!("transfer tx hash: {}", tx_hash);

        retrieve_tx_on_network(&self.proxy, tx_hash.clone()).await;

        self.post_runners.run_transfer_step(&transfer_step);

        tx_hash
    }

    pub async fn simulate_gas_transfer(&mut self, transfer_step: TransferStep) -> u64 {
        let transaction = self.launch_transfer(&transfer_step).await;

        match self.proxy.request(SimulateTxRequest(&transaction)).await {
            Ok(gas) => {
                println!("Gas simulation for the SC transfer: {gas} units.");
                log::info!("Gas simulation for the SC transfer: {gas} units.");
                gas
            }
            Err(err) => {
                simulate_gas_transfer_err_message(&err);
                process::exit(1);
            }
        }
    }

    async fn launch_transfer(&mut self, transfer_step: &TransferStep) -> Transaction {
        self.pre_runners.run_transfer_step(transfer_step);

        let sender_address = &transfer_step.tx.from.value;
        let mut transaction = self.tx_call_to_blockchain_tx(&transfer_step.tx.to_tx_call());
        self.set_tx_nonce_update_sender(sender_address, &mut transaction)
            .await;
        self.sign_tx(sender_address, &mut transaction);

        transaction
    }
}
