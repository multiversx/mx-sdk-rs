#![no_std]

use multiversx_sc::imports::*;

use multiversx_sc_modules::default_issue_callbacks;
mod fractional_uri_info;
use fractional_uri_info::FractionalUriInfo;
pub mod nft_marketplace_proxy;

#[multiversx_sc::contract]
pub trait FractionalNfts: default_issue_callbacks::DefaultIssueCallbacksModule {
    #[init]
    fn init(&self) {}

    #[only_owner]
    #[payable("EGLD")]
    fn issue_and_set_all_roles(
        &self,
        token_display_name: ManagedBuffer,
        token_ticker: ManagedBuffer,
        num_decimals: usize,
    ) {
        let issue_cost = self.call_value().egld_value();
        self.fractional_token().issue_and_set_all_roles(
            EsdtTokenType::SemiFungible,
            issue_cost.clone_value(),
            token_display_name,
            token_ticker,
            num_decimals,
            None,
        );
    }

    #[only_owner]
    #[endpoint(claimRoyaltiesFromMarketplace)]
    fn claim_royalties_from_marketplace(
        &self,
        marketplace_address: ManagedAddress,
        token_id: TokenIdentifier,
        token_nonce: u64,
    ) {
        let caller = self.blockchain().get_caller();
        self.tx()
            .to(&marketplace_address)
            .typed(nft_marketplace_proxy::NftMarketplaceProxy)
            .claim_tokens(caller, token_id, token_nonce)
            .async_call_and_exit();
    }

    #[payable("*")]
    #[endpoint(fractionalizeNFT)]
    fn fractionalize_nft(
        &self,
        initial_fractional_amount: BigUint,
        name: ManagedBuffer,
        attributes: ManagedBuffer,
    ) {
        let original_payment = self.call_value().single_esdt();
        let sc_address = self.blockchain().get_sc_address();
        let original_token_data = self.blockchain().get_esdt_token_data(
            &sc_address,
            &original_payment.token_identifier,
            original_payment.token_nonce,
        );

        let sc_owner = self.blockchain().get_owner_address();
        require!(
            original_token_data.creator == sc_address || original_token_data.creator == sc_owner,
            "Wrong payment creator"
        );

        let fractional_token_mapper = self.fractional_token();
        fractional_token_mapper.require_issued_or_set();
        let fractional_token = fractional_token_mapper.get_token_id_ref();
        let hash = ManagedBuffer::new();
        let fractional_info =
            FractionalUriInfo::new(original_payment, initial_fractional_amount.clone());
        let uris = fractional_info.to_uris();

        let fractional_nonce = self.send().esdt_nft_create(
            fractional_token,
            &initial_fractional_amount,
            &name,
            &original_token_data.royalties,
            &hash,
            &attributes,
            &uris,
        );

        self.tx()
            .to(ToCaller)
            .single_esdt(
                fractional_token,
                fractional_nonce,
                &initial_fractional_amount,
            )
            .transfer();
    }

    #[payable("*")]
    #[endpoint(unFractionalizeNFT)]
    fn unfractionalize_nft(&self) {
        let fractional_payment = self.call_value().single_esdt();
        let fractional_token_mapper = self.fractional_token();

        fractional_token_mapper.require_issued_or_set();
        fractional_token_mapper.require_same_token(&fractional_payment.token_identifier);

        let sc_address = self.blockchain().get_sc_address();
        let token_data = self.blockchain().get_esdt_token_data(
            &sc_address,
            &fractional_payment.token_identifier,
            fractional_payment.token_nonce,
        );

        let fractional_info = FractionalUriInfo::from_uris(token_data.uris);
        require!(
            fractional_info.initial_fractional_amount == fractional_payment.amount,
            "Must provide the full initial amount"
        );

        self.send().esdt_local_burn(
            &fractional_payment.token_identifier,
            fractional_payment.token_nonce,
            &fractional_payment.amount,
        );

        let original = fractional_info.original_payment;

        self.tx()
            .to(ToCaller)
            .single_esdt(
                &original.token_identifier,
                original.token_nonce,
                &original.amount,
            )
            .transfer();
    }

    #[view(getFractionalToken)]
    #[storage_mapper("fractional_token")]
    fn fractional_token(&self) -> NonFungibleTokenMapper;
}
