elrond_wasm::imports!();
elrond_wasm::derive_imports!();

pub mod custom_merged_token_attributes;
pub mod merged_token_attributes;
pub mod merged_token_setup;

use merged_token_attributes::{MergedTokenAttributes, TokenAttributesInstance, MAX_MERGED_TOKENS};

use self::custom_merged_token_attributes::MergedTokenAttributesCreator;

static SC_DOES_NOT_OWN_NFT_PARTS_ERR_MSG: &[u8] = b"NFT parts belong to another merging SC";

const MIN_MERGE_PAYMENTS: usize = 2;

#[elrond_wasm::module]
pub trait TokenMergeModule:
    merged_token_setup::MergedTokenSetupModule
    + crate::default_issue_callbacks::DefaultIssueCallbacksModule
    + crate::pause::PauseModule
{
    fn merge_tokens<AttributesCreator: MergedTokenAttributesCreator<ScType = Self>>(
        &self,
        payments: ManagedVec<EsdtTokenPayment>,
        attr_creator: &AttributesCreator,
    ) -> EsdtTokenPayment<Self::Api> {
        self.require_not_paused();
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

                let merged_inst_attributes: MergedTokenAttributes<Self::Api> =
                    token_data.decode_attributes();
                already_merged_tokens.push(merged_inst_attributes);
            } else {
                require!(
                    token_whitelist.contains(&token.token_identifier),
                    "Token {} cannot be merged",
                    (token.token_identifier)
                );

                let single_token_inst_attributes =
                    TokenAttributesInstance::from_single_token(token, token_data.royalties);
                single_tokens.push((single_token_inst_attributes, token_data.creator));
            }
        }

        let mut merged_attributes = MergedTokenAttributes::new();
        for already_merged in already_merged_tokens {
            merged_attributes.merge_with_other(already_merged);
        }

        for (single_token_instance, creator) in single_tokens {
            merged_attributes.add_or_update_instance(single_token_instance, &creator);
        }

        let merged_token_payment =
            self.create_merged_token(merged_token_id, &merged_attributes, attr_creator);
        let caller = self.blockchain().get_caller();
        self.send()
            .direct_non_zero_esdt_payment(&caller, &merged_token_payment);

        merged_token_payment
    }

    fn split_tokens(&self, payments: ManagedVec<EsdtTokenPayment>) -> ManagedVec<EsdtTokenPayment> {
        self.require_not_paused();
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

            let previously_merged_instance_attributes: MergedTokenAttributes<Self::Api> =
                token_data.decode_attributes();
            for inst in previously_merged_instance_attributes.into_instances() {
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

    fn split_token_partial<AttributesCreator: MergedTokenAttributesCreator<ScType = Self>>(
        &self,
        merged_token: EsdtTokenPayment,
        mut tokens_to_remove: ManagedVec<EsdtTokenPayment>,
        attr_creator: &AttributesCreator,
    ) -> ManagedVec<EsdtTokenPayment> {
        self.require_not_paused();
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

        let mut merged_attributes: MergedTokenAttributes<Self::Api> =
            merged_token_data.decode_attributes();
        for token in &tokens_to_remove {
            merged_attributes.deduct_balance_for_instance(&token);
        }

        self.send().esdt_local_burn(
            &merged_token.token_identifier,
            merged_token.token_nonce,
            &merged_token.amount,
        );

        // all removed tokens get sent to user, so we can re-use this as output payments
        let new_merged_token = self.create_merged_token(
            merged_token.token_identifier,
            &merged_attributes,
            attr_creator,
        );
        tokens_to_remove.push(new_merged_token);

        let caller = self.blockchain().get_caller();
        self.send().direct_multi(&caller, &tokens_to_remove);

        tokens_to_remove
    }
}
