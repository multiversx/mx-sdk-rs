use std::collections::HashMap;

use crate::sdk::{data::transaction::Transaction, wallet::Wallet};
use multiversx_sc_scenario::{imports::Bech32Address, multiversx_sc::types::Address};
use multiversx_sdk::chain_core::std::Bech32Hrp;
use multiversx_sdk::data::account::Account;
use multiversx_sdk::data::esdt::EsdtBalance;
use multiversx_sdk::gateway::{
    GatewayAsyncService, GetAccountEsdtTokensRequest, GetAccountRequest, GetAccountStorageRequest,
};

use crate::InteractorBase;

/// A user account that can sign transactions (a pem is present).
pub struct Sender {
    pub address: Address,
    pub hrp: Bech32Hrp,
    pub wallet: Wallet,
    pub current_nonce: Option<u64>,
}

impl<GatewayProxy> InteractorBase<GatewayProxy>
where
    GatewayProxy: GatewayAsyncService,
{
    pub async fn recall_nonce(&self, address: &Address) -> u64 {
        let account = self
            .proxy()
            .request(GetAccountRequest::new(&address.to_bech32(self.get_hrp())))
            .await
            .expect("failed to retrieve account nonce");

        account.nonce
    }

    pub async fn get_account(&self, address: &Address) -> Account {
        self.proxy()
            .request(GetAccountRequest::new(&address.to_bech32(self.get_hrp())))
            .await
            .expect("failed to retrieve account")
    }

    pub async fn get_account_storage(&self, address: &Address) -> HashMap<String, String> {
        self.proxy()
            .request(GetAccountStorageRequest::new(
                &address.to_bech32(self.get_hrp()),
            ))
            .await
            .expect("failed to retrieve account")
    }

    /// Fetches the on-chain owner of `contract` and returns it as a [`Bech32Address`]
    /// if that owner is a registered wallet, or `None` otherwise.
    pub async fn get_registered_owner(&self, contract: &Address) -> Option<Bech32Address> {
        let account = self.get_account(contract).await;
        let owner_str = account.owner_address.filter(|s| !s.is_empty())?;
        let owner = Bech32Address::from_bech32_string(owner_str);
        if self.is_registered_wallet(owner.as_address()) {
            Some(owner)
        } else {
            None
        }
    }

    pub async fn get_account_esdt(&self, address: &Address) -> HashMap<String, EsdtBalance> {
        self.proxy()
            .request(GetAccountEsdtTokensRequest::new(
                &address.to_bech32(self.get_hrp()),
            ))
            .await
            .expect("failed to retrieve account")
    }

    /// Updates:
    /// - the transaction with the nonce read from the network
    /// - the sender's current_nonce
    pub(crate) async fn set_tx_nonce_update_sender(
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
    }

    pub(crate) fn sign_tx(&self, sender_address: &Address, transaction: &mut Transaction) {
        // read
        let sender = self
            .sender_map
            .get(sender_address)
            .expect("the wallet that was supposed to sign is not registered");

        // sign
        let signature = sender
            .wallet
            .sign_tx(transaction)
            .expect("failed to sign transaction");
        transaction.signature = Some(signature);
    }

    /// Designates a registered wallet as the relayer for all transactions from this interactor.
    ///
    /// The wallet must already be registered via [`Self::register_wallet`].
    /// Call with `None` to clear the relayer.
    pub fn set_relayer(&mut self, address: impl Into<Option<Address>>) {
        let address = address.into();
        if let Some(ref addr) = address {
            assert!(
                self.sender_map.contains_key(addr),
                "relayer wallet not registered; call register_wallet first"
            );
        }
        self.relayer_address = address;
    }

    /// Signs the transaction as the relayer (adds `relayer_signature`).
    ///
    /// No-op if no relayer is configured. Must be called **after** the sender has signed,
    /// and after [`Self::apply_relayer_address`] has set the `relayer` field.
    pub(crate) fn sign_tx_as_relayer(&self, transaction: &mut Transaction) {
        let Some(relayer_address) = &self.relayer_address else {
            return;
        };
        let relayer = self
            .sender_map
            .get(relayer_address)
            .expect("relayer wallet not registered");
        let sig = relayer
            .wallet
            .sign_tx(transaction)
            .expect("failed to sign as relayer");
        transaction.relayer_signature = Some(sig);
    }
}
