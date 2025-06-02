use std::collections::HashMap;

use crate::sdk::{data::transaction::Transaction, wallet::Wallet};
use log::debug;
use multiversx_sc_scenario::multiversx_sc::types::Address;
use multiversx_sdk::data::account::Account;
use multiversx_sdk::data::esdt::EsdtBalance;
use multiversx_sdk::gateway::{
    GatewayAsyncService, GetAccountEsdtTokensRequest, GetAccountRequest, GetAccountStorageRequest,
};

use crate::InteractorBase;

/// A user account that can sign transactions (a pem is present).
#[derive(Debug, Clone)]
pub struct Sender {
    pub address: Address,
    pub hrp: String,
    pub wallet: Wallet,
    pub current_nonce: Option<u64>,
}

impl<GatewayProxy> InteractorBase<GatewayProxy>
where
    GatewayProxy: GatewayAsyncService,
{
    pub async fn recall_nonce(&self, address: &Address) -> u64 {
        let account = self
            .proxy
            .request(GetAccountRequest::new(self.get_hrp(), address))
            .await
            .expect("failed to retrieve account nonce");

        println!("account from recall nonce request: {:?}", account);
        account.nonce
    }

    pub async fn get_account(&self, address: &Address) -> Account {
        self.proxy
            .request(GetAccountRequest::new(self.get_hrp(), address))
            .await
            .expect("failed to retrieve account")
    }

    pub async fn get_account_storage(&self, address: &Address) -> HashMap<String, String> {
        self.proxy
            .request(GetAccountStorageRequest::new(self.get_hrp(), address))
            .await
            .expect("failed to retrieve account")
    }

    pub async fn get_account_esdt(&self, address: &Address) -> HashMap<String, EsdtBalance> {
        self.proxy
            .request(GetAccountEsdtTokensRequest::new(self.get_hrp(), address))
            .await
            .expect("failed to retrieve account")
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
