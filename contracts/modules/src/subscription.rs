multiversx_sc::imports!();

/// Standard smart contract module for managing a Subscription NFT.
/// Adaptation of the EIP-5643 for MultiversX, more here https://eips.ethereum.org/EIPS/eip-5643  
///
/// This standard is an extension of MultiversX NFT standard.
/// It proposes an additional interface for NFTs to be used as recurring, expirable subscriptions.
/// The interface includes functions to renew and cancel the subscription.
///
/// It provides functions for:
/// * renewing a subscription
/// * cancelling a subscription
/// * getting the expiration
///
#[multiversx_sc::module]
pub trait SubscriptionModule {
    #[event("subscriptionUpdate")]
    fn subscription_update_event(
        &self,
        #[indexed] token_id: &ManagedBuffer,
        #[indexed] token_nonce: u64,
        #[indexed] expiration: u64,
    );

    #[endpoint(renewSubscription)]
    fn renew_subscription(&self, id: &TokenIdentifier, nonce: u64, duration: u64) {
        let expiration = self.expires_at(id, nonce);
        let time = self.blockchain().get_block_timestamp();

        let new_expiration = if expiration > time {
            expiration + duration
        } else {
            time + duration
        };

        self.send()
            .nft_update_attributes(id, nonce, &new_expiration);
        self.subscription_update_event(id.as_managed_buffer(), nonce, new_expiration);
    }

    #[endpoint(cancelSubscription)]
    fn cancel_subscription(&self, id: &TokenIdentifier, nonce: u64) {
        self.send().nft_update_attributes(id, nonce, &0);
        self.subscription_update_event(id.as_managed_buffer(), nonce, 0);
    }

    // @dev should only be called if the nft is owned by the contract
    #[view(getExpiresAt)]
    fn expires_at(&self, id: &TokenIdentifier, nonce: u64) -> u64 {
        self.blockchain().get_token_attributes(id, nonce)
    }
}
