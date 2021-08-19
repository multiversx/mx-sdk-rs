elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[derive(TopEncode, TopDecode, NestedEncode, TypeAbi)]
pub struct Auction<M: ManagedTypeApi> {
    pub auctioned_token: EsdtToken,
    pub nr_auctioned_tokens: BigUint<M>,
    pub auction_type: AuctionType,

    pub payment_token: EsdtToken,
    pub min_bid: BigUint<M>,
    pub max_bid: Option<BigUint<M>>,
    pub start_time: u64,
    pub deadline: u64,

    pub original_owner: Address,
    pub current_bid: BigUint<M>,
    pub current_winner: Address,
    pub marketplace_cut_percentage: BigUint<M>,
    pub creator_royalties_percentage: BigUint<M>,
}

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, PartialEq)]
pub enum AuctionType {
    None,
    Nft,
    SftAll,
    SftOnePerPayment,
}

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, Clone)]
pub struct EsdtToken {
    pub token_type: TokenIdentifier,
    pub nonce: u64,
}

pub struct BidSplitAmounts<M: ManagedTypeApi> {
    pub creator: BigUint<M>,
    pub marketplace: BigUint<M>,
    pub seller: BigUint<M>,
}
