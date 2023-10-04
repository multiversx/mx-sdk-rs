multiversx_sc::imports!();
multiversx_sc::derive_imports!();

/// Standard smart contract module for managing a Subscription NFT.
/// Adaptation of the EIP-5643 for MultiversX, more here https://eips.ethereum.org/EIPS/eip-5643  
///
/// This standard is an extension of the MultiversX NFT standard.
/// It proposes an additional interface for NFTs to be used as recurring, expirable subscriptions.
/// The interface includes functions to renew and cancel the subscription.
///
/// Since the NFT standard only has one field for adding arbitrary data (attributes),
/// The module also provides functions for creating NFTs with subscription as well as for reading and updating attributes
/// This allows developers to add additional data to the subscription expiration
///
/// Developers should be careful when interacting with custom attributes at the same time as subscription
/// They should exclusively use the functions from this module
/// The use of the generic function for updating nft attributes might result in data loss
///
/// The module provides functions for:
/// * creating a subscription nft
/// * updating custom attributes
/// * getting custom attributes
/// * renewing a subscription
/// * cancelling a subscription
/// * getting the expiration
///
#[derive(TypeAbi, TopEncode, TopDecode)]
pub struct SubscriptionAttributes<T: NestedEncode + NestedDecode + TypeAbi> {
    pub expiration: u64,
    pub attributes: T,
}

#[multiversx_sc::module]
pub trait SubscriptionModule {
    // ** NFT and Attributes

    fn create_subscription_nft<T: NestedEncode + NestedDecode + TypeAbi>(
        &self,
        token_id: &TokenIdentifier,
        amount: &BigUint,
        name: &ManagedBuffer,
        royalties: &BigUint,
        hash: &ManagedBuffer,
        duration: u64,
        attributes: T,
        uris: &ManagedVec<ManagedBuffer>,
    ) -> u64 {
        let subscription_attributes = SubscriptionAttributes::<T> {
            expiration: self.blockchain().get_block_timestamp() + duration,
            attributes,
        };

        self.send().esdt_nft_create(
            token_id,
            amount,
            name,
            royalties,
            hash,
            &subscription_attributes,
            uris,
        )
    }

    fn update_subscription_attributes<T: NestedEncode + NestedDecode + TypeAbi>(
        &self,
        id: &TokenIdentifier,
        nonce: u64,
        attributes: T,
    ) {
        let subscription_attributes = SubscriptionAttributes::<T> {
            expiration: self.get_subscription::<T>(id, nonce),
            attributes,
        };

        self.send()
            .nft_update_attributes(id, nonce, &subscription_attributes);
    }

    // @dev should only be called if the nft is owned by the contract
    fn get_subscription_attributes<T: NestedEncode + NestedDecode + TypeAbi>(
        &self,
        id: &TokenIdentifier,
        nonce: u64,
    ) -> T {
        let subscription_attributes: SubscriptionAttributes<T> =
            self.blockchain().get_token_attributes(id, nonce);

        subscription_attributes.attributes
    }

    // ** Subscription

    #[event("subscriptionUpdate")]
    fn subscription_update_event(
        &self,
        #[indexed] token_id: &ManagedBuffer,
        #[indexed] token_nonce: u64,
        #[indexed] expiration: u64,
    );

    fn renew_subscription<T: NestedEncode + NestedDecode + TypeAbi>(
        &self,
        id: &TokenIdentifier,
        nonce: u64,
        duration: u64,
    ) {
        let time = self.blockchain().get_block_timestamp();
        let mut subscription_attributes: SubscriptionAttributes<T> =
            self.blockchain().get_token_attributes(id, nonce);
        let expiration = subscription_attributes.expiration;

        subscription_attributes.expiration = if expiration > time {
            expiration + duration
        } else {
            time + duration
        };

        self.send()
            .nft_update_attributes(id, nonce, &subscription_attributes);

        self.subscription_update_event(
            id.as_managed_buffer(),
            nonce,
            subscription_attributes.expiration,
        );
    }

    fn cancel_subscription<T: NestedEncode + NestedDecode + TypeAbi>(
        &self,
        id: &TokenIdentifier,
        nonce: u64,
    ) {
        let mut subscription_attributes: SubscriptionAttributes<T> =
            self.blockchain().get_token_attributes(id, nonce);
        subscription_attributes.expiration = 0;

        self.send()
            .nft_update_attributes(id, nonce, &subscription_attributes);

        self.subscription_update_event(id.as_managed_buffer(), nonce, 0);
    }

    // @dev should only be called if the nft is owned by the contract
    fn get_subscription<T: NestedEncode + NestedDecode + TypeAbi>(
        &self,
        id: &TokenIdentifier,
        nonce: u64,
    ) -> u64 {
        let subscription_attributes: SubscriptionAttributes<T> =
            self.blockchain().get_token_attributes(id, nonce);

        subscription_attributes.expiration
    }
}
