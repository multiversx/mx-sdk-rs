use multiversx_sc_scenario::api::StaticApi;

multiversx_sc::derive_imports!();
multiversx_sc::imports!();

// to test, run the following command in the crate folder:
// cargo expand --test derive_managed_vec_item_struct_2_test > expanded.rs

/// Obtained from a contract from the community.
///
/// Unusually large, payload size is 74.
#[derive(ManagedVecItem)]
pub struct Auction<M: ManagedTypeApi> {
    pub auctioned_token_type: EsdtTokenIdentifier<M>,
    pub auctioned_token_nonce: u64,
    pub nr_auctioned_tokens: BigUint<M>,
    pub auction_type: AuctionType,
    pub payment_token_type: EgldOrEsdtTokenIdentifier<M>,
    pub payment_token_nonce: u64,
    pub min_bid: BigUint<M>,
    pub max_bid: Option<BigUint<M>>,
    pub start_time: u64,
    pub deadline: u64,

    pub original_owner: ManagedAddress<M>,
    pub current_bid: BigUint<M>,
    pub current_winner: ManagedAddress<M>,
    pub marketplace_cut_percentage: BigUint<M>,
    pub creator_royalties_percentage: BigUint<M>,
}

#[derive(ManagedVecItem)]
pub enum AuctionType {
    None,
    NftBid,
    Nft,
    SftAll,
    SftOnePerPayment,
}

#[test]
#[allow(clippy::assertions_on_constants)]
fn struct_3_static() {
    assert_eq!(
        <Auction<StaticApi> as multiversx_sc::types::ManagedVecItem>::payload_size(),
        74
    );
    assert!(!<Auction<StaticApi> as multiversx_sc::types::ManagedVecItem>::SKIPS_RESERIALIZATION);
}
