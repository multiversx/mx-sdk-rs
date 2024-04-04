use crate::Interactor;
use log::info;
use multiversx_sc_scenario::{scenario::ScenarioRunner, scenario_model::TransferStep};

impl Interactor {
    pub async fn transfer(&mut self, transfer_step: TransferStep) -> String {
        self.pre_runners.run_transfer_step(&transfer_step);

        let sender_address = &transfer_step.tx.from.value;
        let mut transaction = self.tx_call_to_blockchain_tx(&transfer_step.tx.to_tx_call());
        self.set_nonce_and_sign_tx(sender_address, &mut transaction)
            .await;
        let tx_hash = self.proxy.send_transaction(&transaction).await.unwrap();
        println!("transfer tx hash: {tx_hash}");
        info!("transfer tx hash: {}", tx_hash);

        self.retrieve_tx_on_network(tx_hash.clone()).await;

        self.post_runners.run_transfer_step(&transfer_step);

        tx_hash
    }
}
