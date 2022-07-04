elrond_wasm::imports!();
elrond_wasm::derive_imports!();

static PAUSED_ERR_MSG: &[u8] = b"Contract is paused";
static SC_DOES_NOT_OWN_NFT_PARTS_ERR_MSG: &[u8] = b"NFT parts belong to another merging SC";
const MIN_MERGE_PAYMENTS: usize = 2;
const NFT_AMOUNT: u64 = 1;

#[derive(
    TypeAbi,
    TopEncode,
    TopDecode,
    NestedEncode,
    NestedDecode,
    ManagedVecItem,
    Debug,
    Clone,
    PartialEq,
)]
pub struct MergedTokenAttributesInstance<M: ManagedTypeApi> {
    pub original_token: EsdtTokenPayment<M>,
    pub attributes_raw: ManagedBuffer<M>,
    pub nr_uris: usize,
}

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

    #[payable("*")]
    #[endpoint(mergeTokens)]
    fn merge_tokens(&self) -> EsdtTokenPayment<Self::Api> {
        require!(self.not_paused(), PAUSED_ERR_MSG);

        let payments = self.call_value().all_esdt_transfers();
        require!(
            payments.len() >= MIN_MERGE_PAYMENTS,
            "Must send at least 2 tokens"
        );

        let merged_token_id = self.merged_token().get_token_id();
        let sc_address = self.blockchain().get_sc_address();

        let mut merged_attributes = ManagedVec::new();
        let mut merged_token_name = ManagedBuffer::new();
        let mut uris = ManagedVec::new();
        let mut royalties_max = BigUint::zero();
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

                let previously_merged_instance_attributes = token_data
                    .decode_attributes::<ManagedVec<MergedTokenAttributesInstance<Self::Api>>>();
                for inst in &previously_merged_instance_attributes {
                    merged_token_name
                        .append(inst.original_token.token_identifier.as_managed_buffer());
                }

                merged_attributes.append_vec(previously_merged_instance_attributes);
            } else {
                merged_token_name.append(token.token_identifier.as_managed_buffer());

                let attributes_instance = MergedTokenAttributesInstance {
                    original_token: token,
                    attributes_raw: token_data.attributes,
                    nr_uris: token_data.uris.len(),
                };
                merged_attributes.push(attributes_instance);
            }

            uris.append_vec(token_data.uris);

            if token_data.royalties > royalties_max {
                royalties_max = token_data.royalties;
            }
        }

        let nft_amount = BigUint::from(NFT_AMOUNT);
        let merged_token_nonce = self.send().esdt_nft_create(
            &merged_token_id,
            &nft_amount,
            &merged_token_name,
            &royalties_max,
            &ManagedBuffer::new(),
            &merged_attributes,
            &uris,
        );

        let caller = self.blockchain().get_caller();
        self.send()
            .direct_esdt(&caller, &merged_token_id, merged_token_nonce, &nft_amount);

        EsdtTokenPayment::new(merged_token_id, merged_token_nonce, nft_amount)
    }

    #[payable("*")]
    #[endpoint(splitTokens)]
    fn split_tokens(&self) -> ManagedVec<EsdtTokenPayment<Self::Api>> {
        require!(self.not_paused(), PAUSED_ERR_MSG);

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

            let previously_merged_instance_attributes = token_data
                .decode_attributes::<ManagedVec<MergedTokenAttributesInstance<Self::Api>>>();
            for inst in &previously_merged_instance_attributes {
                output_payments.push(inst.original_token);
            }

            self.send()
                .esdt_local_burn(&token.token_identifier, token.token_nonce, &token.amount);
        }

        let caller = self.blockchain().get_caller();
        self.send().direct_multi(&caller, &output_payments);

        output_payments
    }

    #[view(getMergedTokenId)]
    #[storage_mapper("mergedToken")]
    fn merged_token(&self) -> NonFungibleTokenMapper<Self::Api>;
}
