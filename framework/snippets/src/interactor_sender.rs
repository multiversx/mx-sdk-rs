use log::debug;
use multiversx_sc_scenario::multiversx_sc::types::Address;
use multiversx_sdk::{data::transaction::Transaction, wallet::Wallet};

use crate::{address_h256_to_erdrs, Interactor};

/// A user account that can sign transactions (a pem is present).
pub struct Sender {
    pub address: Address,
    pub wallet: Wallet,
    pub current_nonce: Option<u64>,
}

impl Interactor {
    pub async fn recall_nonce(&self, address: &Address) -> u64 {
        let erdrs_address = address_h256_to_erdrs(address);
        let account = self
            .proxy
            .get_account(&erdrs_address)
            .await
            .expect("failed to retrieve account nonce");
        account.nonce
    }

    pub(crate) async fn set_nonce_and_sign_tx(
        &mut self,
        sender_address: &Address,
        transaction: &mut Transaction,
    ) {
        // read
        let sender = self
            .sender_map
            .get(sender_address)
            .expect("the wallet that was supposed to sign is not registered");

        // recall
        let nonce = self.recall_nonce(&sender.address).await;
        println!("sender's recalled nonce: {nonce}");

        // set tx nonce
        transaction.nonce = nonce;
        println!("-- tx nonce: {}", transaction.nonce);

        // update
        let sender = self
            .sender_map
            .get_mut(sender_address)
            .expect("the wallet that was supposed to sign is not registered");
        sender.current_nonce = Some(nonce + 1);

        // sign
        let signature = sender.wallet.sign_tx(transaction);
        transaction.signature = Some(hex::encode(signature));
        debug!("transaction {:#?}", transaction);
    }
}
