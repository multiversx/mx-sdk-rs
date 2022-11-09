elrond_wasm::imports!();

use elrond_wasm::elrond_codec::TopEncode;

use super::{
    custom_merged_token_attributes::MergedTokenAttributesCreator,
    merged_token_attributes::MergedTokenAttributes,
};

const NFT_AMOUNT: u64 = 1;

#[elrond_wasm::module]
pub trait MergedTokenSetupModule {
    #[only_owner]
    #[payable("EGLD")]
    #[endpoint(issueMergedToken)]
    fn issue_merged_token(&self, token_display_name: ManagedBuffer, token_ticker: ManagedBuffer) {
        let payment_amount = self.call_value().egld_value();
        self.merged_token().issue_and_set_all_roles(
            EsdtTokenType::NonFungible,
            payment_amount,
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
        merged_attributes: &MergedTokenAttributes<Self::Api>,
        attr_creator: &AttributesCreator,
    ) -> EsdtTokenPayment<Self::Api> {
        let nft_amount = BigUint::from(NFT_AMOUNT);
        let empty_buffer = ManagedBuffer::new();
        let uri = self.create_uri_for_merged_token(merged_attributes);
        let royalties = merged_attributes.get_max_royalties();
        let attributes =
            attr_creator.get_merged_token_attributes(self, &merged_token_id, merged_attributes);
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
        merged_attributes: &MergedTokenAttributes<Self::Api>,
    ) -> ManagedBuffer {
        let mut tokens_list = ManagedVec::<Self::Api, _>::new();
        for inst in merged_attributes.get_instances() {
            let payment = EsdtTokenPayment::new(
                TokenIdentifier::from_esdt_bytes(inst.original_token_id_raw.as_slice()),
                inst.original_token_nonce,
                inst.original_token_amount.clone(),
            );
            tokens_list.push(payment);
        }

        let mut encoded = ManagedBuffer::new();
        let _ = tokens_list.top_encode(&mut encoded);

        encoded
    }

    #[view(getMergedTokenId)]
    #[storage_mapper("mergedToken")]
    fn merged_token(&self) -> NonFungibleTokenMapper<Self::Api>;

    #[view(getMergeableTokensWhitelist)]
    #[storage_mapper("mergeableTokensWhitelist")]
    fn mergeable_tokens_whitelist(&self) -> UnorderedSetMapper<TokenIdentifier>;
}
