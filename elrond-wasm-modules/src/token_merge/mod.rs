elrond_wasm::imports!();
elrond_wasm::derive_imports!();

pub mod merged_token_attributes;

use merged_token_attributes::{MergedTokenAttributes, TokenAttributesInstance, MAX_MERGED_TOKENS};

static SC_DOES_NOT_OWN_NFT_PARTS_ERR_MSG: &[u8] = b"NFT parts belong to another merging SC";

const MIN_MERGE_PAYMENTS: usize = 2;
const NFT_AMOUNT: u64 = 1;

#[elrond_wasm::module]
pub trait TokenMergeModule:
    crate::default_issue_callbacks::DefaultIssueCallbacksModule + crate::pause::PauseModule
{
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

    #[payable("*")]
    #[endpoint(mergeTokens)]
    fn merge_tokens(&self) -> EsdtTokenPayment<Self::Api> {
        self.require_not_paused();

        let payments = self.call_value().all_esdt_transfers();
        require!(
            payments.len() >= MIN_MERGE_PAYMENTS,
            "Must send at least 2 tokens"
        );

        let merged_token_id = self.merged_token().get_token_id();
        let sc_address = self.blockchain().get_sc_address();
        let token_whitelist = self.mergeable_tokens_whitelist();

        let mut already_merged_tokens = ArrayVec::<_, MAX_MERGED_TOKENS>::new();
        let mut single_tokens = ArrayVec::<_, MAX_MERGED_TOKENS>::new();
        for token in &payments {
            let token_data = self.blockchain().get_esdt_token_data(
                &sc_address,
                &token.token_identifier,
                token.token_nonce,
            );

            if token.token_identifier == merged_token_id {
                require!(
                    token_data.creator == sc_address,
                    SC_DOES_NOT_OWN_NFT_PARTS_ERR_MSG
                );

                let merged_inst_attributes =
                    token_data.decode_attributes::<ArrayVec<TokenAttributesInstance<Self::Api>, MAX_MERGED_TOKENS>>();

                already_merged_tokens.push(merged_inst_attributes);
            } else {
                require!(
                    token_whitelist.contains(&token.token_identifier),
                    "Token {} cannot be merged",
                    (token.token_identifier)
                );

                let single_token_inst_attributes =
                    TokenAttributesInstance::from_single_token(token, token_data);

                single_tokens.push(single_token_inst_attributes);
            }
        }

        let mut already_merged_tokens_iter = already_merged_tokens.into_iter();
        let mut merged_attributes = if let Some(already_merged) = already_merged_tokens_iter.next()
        {
            MergedTokenAttributes::new_from_sorted_instances(already_merged)
        } else {
            MergedTokenAttributes::new()
        };

        for already_merged in already_merged_tokens_iter {
            merged_attributes.merge_with_other(already_merged);
        }

        for single_token_instance in single_tokens {
            merged_attributes.add_or_update_instance(single_token_instance);
        }

        let merged_token_payment = self.create_merged_token(merged_token_id, merged_attributes);
        let caller = self.blockchain().get_caller();
        self.send().direct_esdt(
            &caller,
            &merged_token_payment.token_identifier,
            merged_token_payment.token_nonce,
            &merged_token_payment.amount,
        );

        merged_token_payment
    }

    #[payable("*")]
    #[endpoint(splitTokens)]
    fn split_tokens(&self) -> MultiValueEncoded<EsdtTokenPayment<Self::Api>> {
        self.require_not_paused();

        let payments = self.call_value().all_esdt_transfers();
        require!(!payments.is_empty(), "No payments");

        let merged_token_id = self.merged_token().get_token_id();
        let sc_address = self.blockchain().get_sc_address();

        let mut output_payments = ManagedVec::new();
        for token in &payments {
            require!(
                token.token_identifier == merged_token_id,
                "Invalid token to split"
            );

            let token_data = self.blockchain().get_esdt_token_data(
                &sc_address,
                &token.token_identifier,
                token.token_nonce,
            );
            require!(
                token_data.creator == sc_address,
                SC_DOES_NOT_OWN_NFT_PARTS_ERR_MSG
            );

            let previously_merged_instance_attributes =
                token_data.decode_attributes::<ArrayVec<TokenAttributesInstance<Self::Api>, MAX_MERGED_TOKENS>>();
            for inst in previously_merged_instance_attributes {
                let original_token = EsdtTokenPayment::new(
                    TokenIdentifier::from_esdt_bytes(inst.original_token_id_raw.as_slice()),
                    inst.original_token_nonce,
                    inst.original_token_amount,
                );
                output_payments.push(original_token);
            }

            self.send()
                .esdt_local_burn(&token.token_identifier, token.token_nonce, &token.amount);
        }

        let caller = self.blockchain().get_caller();
        self.send().direct_multi(&caller, &output_payments);

        output_payments.into()
    }

    #[payable("*")]
    #[endpoint(splitTokenPartial)]
    fn split_token_partial(
        &self,
        tokens_to_remove: MultiValueEncoded<EsdtTokenPayment<Self::Api>>,
    ) -> MultiValueEncoded<EsdtTokenPayment<Self::Api>> {
        self.require_not_paused();

        let merged_token = self.call_value().single_esdt();
        self.merged_token()
            .require_same_token(&merged_token.token_identifier);

        let sc_address = self.blockchain().get_sc_address();
        let merged_token_data = self.blockchain().get_esdt_token_data(
            &sc_address,
            &merged_token.token_identifier,
            merged_token.token_nonce,
        );
        require!(
            merged_token_data.creator == sc_address,
            SC_DOES_NOT_OWN_NFT_PARTS_ERR_MSG
        );

        let merged_attributes_raw = merged_token_data
            .decode_attributes::<ArrayVec<TokenAttributesInstance<Self::Api>, MAX_MERGED_TOKENS>>();
        let mut merged_attributes =
            MergedTokenAttributes::new_from_sorted_instances(merged_attributes_raw);

        let mut tokens_to_remove_vec = tokens_to_remove.to_vec();
        for token in &tokens_to_remove_vec {
            merged_attributes.deduct_balance_for_instance(&token);
        }

        self.send().esdt_local_burn(
            &merged_token.token_identifier,
            merged_token.token_nonce,
            &merged_token.amount,
        );

        // all removed tokens get sent to user, so we can re-use this as output payments
        let new_merged_token =
            self.create_merged_token(merged_token.token_identifier, merged_attributes);
        tokens_to_remove_vec.push(new_merged_token);

        let caller = self.blockchain().get_caller();
        self.send().direct_multi(&caller, &tokens_to_remove_vec);

        tokens_to_remove_vec.into()
    }

    fn create_merged_token(
        &self,
        merged_token_id: TokenIdentifier,
        merged_attributes: MergedTokenAttributes<Self::Api>,
    ) -> EsdtTokenPayment<Self::Api> {
        let nft_amount = BigUint::from(NFT_AMOUNT);
        let empty_buffer = ManagedBuffer::new();
        let uris = merged_attributes.construct_full_uri_list();
        let royalties = merged_attributes.get_max_royalties();
        let merged_token_nonce = self.send().esdt_nft_create(
            &merged_token_id,
            &nft_amount,
            &empty_buffer,
            &royalties,
            &empty_buffer,
            &merged_attributes.into_instances(),
            &uris,
        );

        EsdtTokenPayment::new(merged_token_id, merged_token_nonce, nft_amount)
    }

    #[view(getMergedTokenId)]
    #[storage_mapper("mergedToken")]
    fn merged_token(&self) -> NonFungibleTokenMapper<Self::Api>;

    #[view(getMergeableTokensWhitelist)]
    #[storage_mapper("mergeableTokensWhitelist")]
    fn mergeable_tokens_whitelist(&self) -> UnorderedSetMapper<TokenIdentifier>;
}
