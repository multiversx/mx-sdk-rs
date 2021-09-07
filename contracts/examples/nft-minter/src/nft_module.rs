elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use elrond_wasm::elrond_codec::TopEncode;

const NFT_AMOUNT: u32 = 1;
const ROYALTIES_MAX: u32 = 10_000;

#[derive(TypeAbi, TopEncode, TopDecode)]
pub struct ExampleAttributes {
    pub creation_timestamp: u64,
}

#[derive(TypeAbi, TopEncode, TopDecode)]
pub struct PriceTag<M: ManagedTypeApi> {
    pub token: TokenIdentifier<M>,
    pub nonce: u64,
    pub amount: BigUint<M>,
}

#[elrond_wasm::module]
pub trait NftModule {
    // endpoints - owner-only

    #[only_owner]
    #[payable("EGLD")]
    #[endpoint(issueToken)]
    fn issue_token(
        &self,
        #[payment] issue_cost: BigUint,
        token_name: BoxedBytes,
        token_ticker: BoxedBytes,
    ) -> SCResult<AsyncCall<Self::SendApi>> {
        require!(self.nft_token_id().is_empty(), "Token already issued");

        Ok(ESDTSystemSmartContractProxy::new_proxy_obj(self.send())
            .issue_non_fungible(
                issue_cost,
                &token_name,
                &token_ticker,
                NonFungibleTokenProperties {
                    can_freeze: true,
                    can_wipe: true,
                    can_pause: true,
                    can_change_owner: false,
                    can_upgrade: false,
                    can_add_special_roles: true,
                },
            )
            .async_call()
            .with_callback(self.callbacks().issue_callback()))
    }

    #[only_owner]
    #[endpoint(setLocalRoles)]
    fn set_local_roles(&self) -> SCResult<AsyncCall<Self::SendApi>> {
        self.require_token_issued()?;

        Ok(ESDTSystemSmartContractProxy::new_proxy_obj(self.send())
            .set_special_roles(
                &self.blockchain().get_sc_address(),
                &self.nft_token_id().get(),
                &[EsdtLocalRole::NftCreate],
            )
            .async_call())
    }

    #[only_owner]
    #[endpoint(createNft)]
    fn create_nft(
        &self,
        name: BoxedBytes,
        royalties: BigUint,
        uri: BoxedBytes,
        selling_price: BigUint,
        #[var_args] opt_token_used_as_payment: OptionalArg<TokenIdentifier>,
        #[var_args] opt_token_used_as_payment_nonce: OptionalArg<u64>,
    ) -> SCResult<u64> {
        self.require_token_issued()?;
        self.require_local_roles_set()?;
        require!(royalties <= ROYALTIES_MAX, "Royalties cannot exceed 100%");

        let token_used_as_payment = opt_token_used_as_payment
            .into_option()
            .unwrap_or_else(|| self.types().token_identifier_egld());
        require!(
            token_used_as_payment.is_egld() || token_used_as_payment.is_valid_esdt_identifier(),
            "Invalid token used as payment"
        );

        let token_used_as_payment_nonce = if token_used_as_payment.is_egld() {
            0
        } else {
            opt_token_used_as_payment_nonce
                .into_option()
                .unwrap_or_default()
        };

        let nft_token_id = self.nft_token_id().get();
        let attributes = ExampleAttributes {
            creation_timestamp: self.blockchain().get_block_timestamp(),
        };

        let mut serialized_attributes = Vec::new();
        attributes.top_encode(&mut serialized_attributes)?;

        let attributes_hash = self.crypto().sha256(&serialized_attributes);

        let nft_nonce = self.send().esdt_nft_create(
            &nft_token_id,
            &self.types().big_uint_from(NFT_AMOUNT),
            &name,
            &royalties,
            &attributes_hash.as_bytes().into(),
            &attributes,
            &[uri],
        );

        self.price_tag(nft_nonce).set(&PriceTag {
            token: token_used_as_payment,
            nonce: token_used_as_payment_nonce,
            amount: selling_price,
        });

        Ok(nft_nonce)
    }

    // endpoints

    #[payable("*")]
    #[endpoint(buyNft)]
    fn buy_nft(
        &self,
        #[payment_token] payment_token: TokenIdentifier,
        #[payment_nonce] payment_nonce: u64,
        #[payment_amount] payment_amount: BigUint,
        nft_nonce: u64,
    ) -> SCResult<()> {
        self.require_token_issued()?;
        require!(
            !self.price_tag(nft_nonce).is_empty(),
            "Invalid nonce or NFT was already sold"
        );

        let price_tag = self.price_tag(nft_nonce).get();
        require!(
            payment_token == price_tag.token,
            "Invalid token used as payment"
        );
        require!(
            payment_nonce == price_tag.nonce,
            "Invalid nonce for payment token"
        );
        require!(
            payment_amount == price_tag.amount,
            "Invalid amount as payment"
        );

        self.price_tag(nft_nonce).clear();

        let nft_token_id = self.nft_token_id().get();
        let caller = self.blockchain().get_caller();
        self.send().direct(
            &caller,
            &nft_token_id,
            nft_nonce,
            &self.types().big_uint_from(NFT_AMOUNT),
            &[],
        );

        Ok(())
    }

    // views

    #[view(getNftPrice)]
    fn get_nft_price(
        &self,
        nft_nonce: u64,
    ) -> OptionalResult<MultiResult3<TokenIdentifier, u64, BigUint>> {
        if self.price_tag(nft_nonce).is_empty() {
            // NFT was already sold
            OptionalResult::None
        } else {
            let price_tag = self.price_tag(nft_nonce).get();

            OptionalResult::Some((price_tag.token, price_tag.nonce, price_tag.amount).into())
        }
    }

    // callbacks

    #[callback]
    fn issue_callback(&self, #[call_result] result: AsyncCallResult<TokenIdentifier>) {
        match result {
            AsyncCallResult::Ok(token_id) => {
                self.nft_token_id().set(&token_id);
            },
            AsyncCallResult::Err(_) => {
                let caller = self.blockchain().get_owner_address();
                let (returned_tokens, token_id) = self.call_value().payment_token_pair();
                if token_id.is_egld() && returned_tokens > 0 {
                    self.send()
                        .direct(&caller, &token_id, 0, &returned_tokens, &[]);
                }
            },
        }
    }

    // private

    fn require_token_issued(&self) -> SCResult<()> {
        require!(!self.nft_token_id().is_empty(), "Token not issued");
        Ok(())
    }

    fn require_local_roles_set(&self) -> SCResult<()> {
        let nft_token_id = self.nft_token_id().get();
        let roles = self.blockchain().get_esdt_local_roles(&nft_token_id);

        require!(
            roles.contains(&EsdtLocalRole::NftCreate),
            "NFTCreate role not set"
        );

        Ok(())
    }

    // storage

    #[storage_mapper("nftTokenId")]
    fn nft_token_id(&self) -> SingleValueMapper<Self::Storage, TokenIdentifier>;

    #[storage_mapper("priceTag")]
    fn price_tag(
        &self,
        nft_nonce: u64,
    ) -> SingleValueMapper<Self::Storage, PriceTag<Self::TypeManager>>;
}
