use crate::{multiversx_sc::types::Address, Interactor, Sender};
use futures::future::join_all;
use multiversx_sc_scenario::scenario_model::ScCallStep;
use multiversx_sdk::data::transaction::{Transaction, TransactionOnNetwork};
use std::collections::HashSet;

type Txs = Vec<Transaction>;
type SenderSet = HashSet<Address>;

impl Interactor {
    pub async fn multiple_sc_calls_parallel(
        &mut self,
        sc_call_steps: &[ScCallStep],
    ) -> Vec<TransactionOnNetwork> {
        let senders = retrieve_senders(sc_call_steps);
        self.recall_senders_nonce(senders).await;

        let txs = self.retrieve_txs(sc_call_steps);
        self.process_txs(txs).await
    }

    async fn process_txs(&mut self, txs: Vec<Transaction>) -> Vec<TransactionOnNetwork> {
        let mut futures = Vec::new();

        for tx in &txs {
            let tx_hash = self
                .proxy
                .send_transaction(tx)
                .await
                .expect("failed to send transaction");

            println!("process tx: {tx_hash} with nonce: {}",tx.nonce);
            futures.push(self.retrieve_tx_on_network(tx_hash.clone()));
        }

        join_all(futures).await
    }

    async fn recall_senders_nonce(&mut self, senders: HashSet<Address>) {
        for sender_address in &senders {
            let nonce = self.recall_nonce(sender_address).await;
            let sender = self
                .sender_map
                .get_mut(sender_address)
                .expect("sender not registered");

            sender.current_nonce = Some(nonce);
            println!("sender's recalled nonce: {nonce}");
        }
    }

    fn retrieve_txs(&mut self, sc_call_steps: &[ScCallStep]) -> Vec<Transaction> {
        let mut txs = Txs::new();

        for sc_call_step in sc_call_steps {
            let mut transaction = self.tx_call_to_blockchain_tx(&sc_call_step.tx);
            let sender_address = &sc_call_step.tx.from.value;
            let sender = self
                .sender_map
                .get_mut(sender_address)
                .expect("sender not registered");

            update_nonces_and_sign_tx(&mut transaction, sender);
            txs.push(transaction);
        }
        txs
    }
}

fn retrieve_senders(sc_call_steps: &[ScCallStep]) -> SenderSet {
    let mut senders = SenderSet::new();

    for sc_call_step in sc_call_steps {
        let sender_address = &sc_call_step.tx.from.value;
        senders.insert(sender_address.clone());
    }
    senders
}

fn update_nonces_and_sign_tx(transaction: &mut Transaction, sender: &mut Sender) {
    transaction.nonce = sender.current_nonce.unwrap();
    sender.current_nonce = Some(sender.current_nonce.unwrap() + 1);

    let signature = sender.wallet.sign_tx(&*transaction);
    transaction.signature = Some(hex::encode(signature));
}
