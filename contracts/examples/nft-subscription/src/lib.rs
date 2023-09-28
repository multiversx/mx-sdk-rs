#![no_std]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use multiversx_sc_modules::{default_issue_callbacks, subscription};

#[multiversx_sc::contract]
pub trait NftSubscription:
    default_issue_callbacks::DefaultIssueCallbacksModule + subscription::SubscriptionModule
{
    #[init]
    fn init(&self) {}

    #[endpoint]
    fn issue(&self) {
        self.token_id().issue_and_set_all_roles(
            EsdtTokenType::NonFungible,
            self.call_value().egld_value().clone_value(),
            ManagedBuffer::from(b"Subscription"),
            ManagedBuffer::from(b"SUB"),
            0,
            None,
        )
    }

    #[endpoint]
    fn mint(&self) {
        self.token_id().nft_create_and_send(
            &self.blockchain().get_caller(),
            BigUint::from(1u8),
            &0u64,
        );
    }

    #[payable("*")]
    #[endpoint]
    fn renew(&self, duration: u64) {
        let (id, nonce, _) = self.call_value().single_esdt().into_tuple();
        self.renew_subscription(&id, nonce, duration);
        self.send().direct_esdt(
            &self.blockchain().get_caller(),
            &id,
            nonce,
            &BigUint::from(1u8),
        );
    }

    #[payable("*")]
    #[endpoint]
    fn cancel(&self) {
        let (id, nonce, _) = self.call_value().single_esdt().into_tuple();
        self.cancel_subscription(&id, nonce);
        self.send().direct_esdt(
            &self.blockchain().get_caller(),
            &id,
            nonce,
            &BigUint::from(1u8),
        );
    }

    #[view]
    fn expires(&self, id: &TokenIdentifier, nonce: u64) -> u64 {
        self.expires_at(id, nonce)
    }

    #[storage_mapper("tokenId")]
    fn token_id(&self) -> NonFungibleTokenMapper;
}
