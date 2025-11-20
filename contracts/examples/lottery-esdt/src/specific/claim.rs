use multiversx_sc::imports::*;

use crate::basics::storage;

#[multiversx_sc::module]
pub trait ClaimModule: storage::StorageModule {
    #[endpoint]
    fn claim_rewards(&self, tokens: MultiValueEncoded<TokenIdentifier>) {
        let caller = self.blockchain().get_caller();
        let caller_id = self.address_to_id_mapper().get_id_or_insert(&caller);
        require!(
            !self.user_accumulated_token_rewards(&caller_id).is_empty(),
            "You have no rewards to claim"
        );

        let mut accumulated_rewards = MultiEsdtPayment::new();

        // to save reviewers time, these 2 iterators have different generics, so it was not possible to make just 1 for loop

        if tokens.is_empty() {
            // if wanted tokens were not specified claim all, and clear user_accumulated_token_rewards storage mapper
            self.handle_claim_with_unspecified_tokens(&caller_id, &mut accumulated_rewards);
        } else {
            // otherwise claim just what was requested and remove those tokens from the user_accumulated_token_rewards storage mapper

            self.claim_rewards_user(tokens.to_vec(), &caller_id, &mut accumulated_rewards)
        };
        if !accumulated_rewards.is_empty() {
            self.tx()
                .to(&caller)
                .multi_esdt(accumulated_rewards)
                .transfer();
        }
    }

    fn handle_claim_with_unspecified_tokens(
        &self,
        caller_id: &u64,
        accumulated_rewards: &mut MultiEsdtPayment<Self::Api>,
    ) {
        let mut all_tokens: ManagedVec<Self::Api, TokenIdentifier> = ManagedVec::new();

        for token_id in self.user_accumulated_token_rewards(caller_id).iter() {
            require!(
                !self.accumulated_rewards(&token_id, caller_id).is_empty(),
                "Token requested not available for claim"
            );
            all_tokens.push(token_id);
        }

        self.claim_rewards_user(all_tokens, caller_id, accumulated_rewards)
    }

    fn claim_rewards_user(
        &self,
        tokens: ManagedVec<Self::Api, TokenIdentifier>,
        caller_id: &u64,
        accumulated_rewards: &mut MultiEsdtPayment<Self::Api>,
    ) {
        for token_id in tokens.iter().rev() {
            let _ = &self
                .user_accumulated_token_rewards(caller_id)
                .swap_remove(&token_id);

            self.prepare_token_for_claim(token_id.clone_value(), caller_id, accumulated_rewards);
        }
    }

    fn prepare_token_for_claim(
        &self,
        token_id: TokenIdentifier,
        caller_id: &u64,
        accumulated_rewards: &mut MultiEsdtPayment<Self::Api>,
    ) {
        let value = self.accumulated_rewards(&token_id, caller_id).take();
        accumulated_rewards.push(EsdtTokenPayment::new(token_id, 0, value));
    }
}
