multiversx_sc::imports!();

use super::{
    custom_merged_token_attributes::MergedTokenAttributesCreator,
    merged_token_instances::{MergedTokenInstances, MAX_MERGED_TOKENS},
};

const NFT_AMOUNT: u64 = 1;
pub static DIFFERENT_CREATOR_ERR_MSG: &[u8] = b"All merged tokens must have the same creator";

#[multiversx_sc::module]
pub trait MergedTokenSetupModule {
    #[only_owner]
    #[payable("EGLD")]
    #[endpoint(issueMergedToken)]
    fn issue_merged_token(&self, token_display_name: ManagedBuffer, token_ticker: ManagedBuffer) {
        let payment_amount = self.call_value().egld_value();
        self.merged_token().issue_and_set_all_roles(
            EsdtTokenType::NonFungible,
            payment_amount.clone_value(),
            token_display_name,
            token_ticker,
            0,
            None,
        );
    }

    #[only_owner]
    #[endpoint(addMergeableTokensToWhitelist)]
    fn add_mergeable_tokens_to_whitelist(&self, tokens: MultiValueEncoded<TokenIdentifier>) {
        let mut whitelist = self.mergeable_tokens_whitelist();
        for token in tokens {
            let _ = whitelist.insert(token);
        }
    }

    #[only_owner]
    #[endpoint(removeMergeableTokensFromWhitelist)]
    fn remove_mergeable_tokens_from_whitelist(&self, tokens: MultiValueEncoded<TokenIdentifier>) {
        let mut whitelist = self.mergeable_tokens_whitelist();
        for token in tokens {
            let _ = whitelist.swap_remove(&token);
        }
    }

    fn create_merged_token<AttributesCreator: MergedTokenAttributesCreator<ScType = Self>>(
        &self,
        merged_token_id: TokenIdentifier,
        merged_instances: &MergedTokenInstances<Self::Api>,
        attr_creator: &AttributesCreator,
    ) -> EsdtTokenPayment<Self::Api> {
        let nft_amount = BigUint::from(NFT_AMOUNT);
        let empty_buffer = ManagedBuffer::new();

        let all_token_data = self.collect_token_data(merged_instances);
        self.require_all_parts_same_creator(&all_token_data);

        let royalties = self.get_max_royalties(&all_token_data);
        let uri = self.create_uri_for_merged_token(merged_instances);
        let attributes =
            attr_creator.get_merged_token_attributes(self, &merged_token_id, merged_instances);
        let merged_token_nonce = self.send().esdt_nft_create(
            &merged_token_id,
            &nft_amount,
            &empty_buffer,
            &royalties,
            &empty_buffer,
            &attributes,
            &ManagedVec::from_single_item(uri),
        );

        EsdtTokenPayment::new(merged_token_id, merged_token_nonce, nft_amount)
    }

    fn create_uri_for_merged_token(
        &self,
        merged_instances: &MergedTokenInstances<Self::Api>,
    ) -> ManagedBuffer {
        let mut tokens_list = ManagedVec::<Self::Api, _>::new();
        for inst in merged_instances.get_instances() {
            tokens_list.push(inst.clone());
        }

        let mut encoded = ManagedBuffer::new();
        let _ = tokens_list.top_encode(&mut encoded);

        encoded
    }

    fn collect_token_data(
        &self,
        merged_instances: &MergedTokenInstances<Self::Api>,
    ) -> ArrayVec<EsdtTokenData<Self::Api>, MAX_MERGED_TOKENS> {
        let mut all_token_data = ArrayVec::new();
        let own_sc_address = self.blockchain().get_sc_address();
        for inst in merged_instances.get_instances() {
            let token_data = self.blockchain().get_esdt_token_data(
                &own_sc_address,
                &inst.token_identifier,
                inst.token_nonce,
            );
            unsafe {
                all_token_data.push_unchecked(token_data);
            }
        }

        all_token_data
    }

    fn require_all_parts_same_creator(
        &self,
        all_token_data: &ArrayVec<EsdtTokenData<Self::Api>, MAX_MERGED_TOKENS>,
    ) {
        if all_token_data.is_empty() {
            return;
        }

        let first_creator = unsafe { &all_token_data.get_unchecked(0).creator };
        for token_data in &all_token_data.as_slice()[1..] {
            require!(
                &token_data.creator == first_creator,
                DIFFERENT_CREATOR_ERR_MSG
            );
        }
    }

    fn get_max_royalties(
        &self,
        all_token_data: &ArrayVec<EsdtTokenData<Self::Api>, MAX_MERGED_TOKENS>,
    ) -> BigUint {
        let zero = BigUint::zero();
        let mut max_ref = &zero;
        for token_data in all_token_data {
            if &token_data.royalties > max_ref {
                max_ref = &token_data.royalties;
            }
        }

        max_ref.clone()
    }

    #[view(getMergedTokenId)]
    #[storage_mapper("mergedToken")]
    fn merged_token(&self) -> NonFungibleTokenMapper;

    #[view(getMergeableTokensWhitelist)]
    #[storage_mapper("mergeableTokensWhitelist")]
    fn mergeable_tokens_whitelist(&self) -> UnorderedSetMapper<TokenIdentifier>;
}
