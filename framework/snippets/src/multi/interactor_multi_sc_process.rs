use crate::sdk::data::transaction::{Transaction, TransactionOnNetwork};
use crate::{multiversx_sc::types::Address, InteractorBase, Sender};
use futures::future::join_all;
use multiversx_sdk::gateway::{GatewayAsyncService, SendTxRequest};
use multiversx_sdk::retrieve_tx_on_network;
use std::collections::HashSet;

pub(crate) type Txs = Vec<Transaction>;
pub(crate) type SenderSet = HashSet<Address>;

impl<GatewayProxy> InteractorBase<GatewayProxy>
where
    GatewayProxy: GatewayAsyncService,
{
    pub(crate) async fn recall_senders_nonce(&mut self, senders: HashSet<Address>) {
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

    pub(crate) async fn process_txs(&mut self, txs: Vec<Transaction>) -> Vec<TransactionOnNetwork> {
        let mut futures = Vec::new();

        for tx in &txs {
            let tx_hash = self
                .proxy
                .request(SendTxRequest(tx))
                .await
                .expect("failed to send transaction");

            println!("process tx hash: {tx_hash} with nonce: {}", tx.nonce);
            futures.push(retrieve_tx_on_network(&self.proxy, tx_hash.clone()));
        }

        self.generate_blocks(4).await.unwrap();
        join_all(futures).await
    }
}

pub(crate) fn update_nonces_and_sign_tx(transaction: &mut Transaction, sender: &mut Sender) {
    transaction.nonce = sender.current_nonce.unwrap();
    sender.current_nonce = Some(sender.current_nonce.unwrap() + 1);

    let signature = sender.wallet.sign_tx(&*transaction);
    transaction.signature = Some(hex::encode(signature));
}
