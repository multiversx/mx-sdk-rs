#![no_std]

use multiversx_sc::imports::*;

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
        let nonce = self.create_subscription_nft(
            self.token_id().get_token_id_ref(),
            &BigUint::from(1u8),
            &ManagedBuffer::new(),
            &BigUint::from(0u8),
            &ManagedBuffer::new(),
            0,
            ManagedBuffer::from(b"common"),
            &ManagedVec::new(),
        );

        self.tx()
            .to(ToCaller)
            .single_esdt(
                self.token_id().get_token_id_ref(),
                nonce,
                &BigUint::from(1u8),
            )
            .transfer();
    }

    #[payable("*")]
    #[endpoint]
    fn update_attributes(&self, attributes: ManagedBuffer) {
        let (id, nonce, _) = self.call_value().single_esdt().into_tuple();
        self.update_subscription_attributes::<ManagedBuffer>(&id, nonce, attributes);
        self.tx()
            .to(ToCaller)
            .single_esdt(&id, nonce, &BigUint::from(1u8))
            .transfer();
    }

    #[payable("*")]
    #[endpoint]
    fn renew(&self, duration: u64) {
        let (id, nonce, _) = self.call_value().single_esdt().into_tuple();
        self.renew_subscription::<ManagedBuffer>(&id, nonce, duration);
        self.tx()
            .to(ToCaller)
            .single_esdt(&id, nonce, &BigUint::from(1u8))
            .transfer();
    }

    #[payable("*")]
    #[endpoint]
    fn cancel(&self) {
        let (id, nonce, _) = self.call_value().single_esdt().into_tuple();
        self.cancel_subscription::<ManagedBuffer>(&id, nonce);

        self.tx()
            .to(ToCaller)
            .single_esdt(&id, nonce, &BigUint::from(1u8))
            .transfer();
    }

    #[storage_mapper("tokenId")]
    fn token_id(&self) -> NonFungibleTokenMapper;
}
