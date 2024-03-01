use crate::distribution_module;

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use multiversx_sc_modules::default_issue_callbacks;

const NFT_AMOUNT: u32 = 1;
const ROYALTIES_MAX: u32 = 10_000; // 100%

#[derive(TypeAbi, TopEncode, TopDecode)]
pub struct PriceTag<M: ManagedTypeApi> {
    pub token: EgldOrEsdtTokenIdentifier<M>,
    pub nonce: u64,
    pub amount: BigUint<M>,
}

#[multiversx_sc::module]
pub trait NftModule:
    distribution_module::DistributionModule + default_issue_callbacks::DefaultIssueCallbacksModule
{
    // endpoints - owner-only

    #[only_owner]
    #[payable("EGLD")]
    #[endpoint(issueToken)]
    fn issue_token(&self, token_display_name: ManagedBuffer, token_ticker: ManagedBuffer) {
        let issue_cost = self.call_value().egld_value();
        self.nft_token_id().issue_and_set_all_roles(
            EsdtTokenType::NonFungible,
            issue_cost.clone_value(),
            token_display_name,
            token_ticker,
            0,
            None,
        );
    }

    // endpoints

    #[payable("*")]
    #[endpoint(buyNft)]
    fn buy_nft(&self, nft_nonce: u64) {
        let payment = self.call_value().egld_or_single_esdt();

        self.require_token_issued();
        require!(
            !self.price_tag(nft_nonce).is_empty(),
            "Invalid nonce or NFT was already sold"
        );

        let price_tag = self.price_tag(nft_nonce).get();
        require!(
            payment.token_identifier == price_tag.token,
            "Invalid token used as payment"
        );
        require!(
            payment.token_nonce == price_tag.nonce,
            "Invalid nonce for payment token"
        );
        require!(
            payment.amount == price_tag.amount,
            "Invalid amount as payment"
        );

        self.price_tag(nft_nonce).clear();

        let nft_token_id = self.nft_token_id().get_token_id();
        let caller = self.blockchain().get_caller();
        self.send().direct_esdt(
            &caller,
            &nft_token_id,
            nft_nonce,
            &BigUint::from(NFT_AMOUNT),
        );

        self.distribute_funds(
            &payment.token_identifier,
            payment.token_nonce,
            payment.amount,
        );
    }

    // views

    #[allow(clippy::type_complexity)]
    #[view(getNftPrice)]
    fn get_nft_price(
        &self,
        nft_nonce: u64,
    ) -> OptionalValue<MultiValue3<EgldOrEsdtTokenIdentifier, u64, BigUint>> {
        if self.price_tag(nft_nonce).is_empty() {
            // NFT was already sold
            OptionalValue::None
        } else {
            let price_tag = self.price_tag(nft_nonce).get();

            OptionalValue::Some((price_tag.token, price_tag.nonce, price_tag.amount).into())
        }
    }

    // private

    #[allow(clippy::too_many_arguments)]
    fn create_nft_with_attributes<T: TopEncode>(
        &self,
        name: ManagedBuffer,
        royalties: BigUint,
        attributes: T,
        uri: ManagedBuffer,
        selling_price: BigUint,
        token_used_as_payment: EgldOrEsdtTokenIdentifier,
        token_used_as_payment_nonce: u64,
    ) -> u64 {
        self.require_token_issued();
        require!(royalties <= ROYALTIES_MAX, "Royalties cannot exceed 100%");

        let nft_token_id = self.nft_token_id().get_token_id();

        let mut serialized_attributes = ManagedBuffer::new();
        if let core::result::Result::Err(err) = attributes.top_encode(&mut serialized_attributes) {
            sc_panic!("Attributes encode error: {}", err.message_bytes());
        }

        let attributes_sha256 = self.crypto().sha256(&serialized_attributes);
        let attributes_hash = attributes_sha256.as_managed_buffer();
        let uris = ManagedVec::from_single_item(uri);
        let nft_nonce = self.send().esdt_nft_create(
            &nft_token_id,
            &BigUint::from(NFT_AMOUNT),
            &name,
            &royalties,
            attributes_hash,
            &attributes,
            &uris,
        );

        self.price_tag(nft_nonce).set(&PriceTag {
            token: token_used_as_payment,
            nonce: token_used_as_payment_nonce,
            amount: selling_price,
        });

        nft_nonce
    }

    fn require_token_issued(&self) {
        require!(!self.nft_token_id().is_empty(), "Token not issued");
    }

    // storage

    #[view(getNftTokenId)]
    #[storage_mapper("nftTokenId")]
    fn nft_token_id(&self) -> NonFungibleTokenMapper;

    #[storage_mapper("priceTag")]
    fn price_tag(&self, nft_nonce: u64) -> SingleValueMapper<PriceTag<Self::Api>>;
}
