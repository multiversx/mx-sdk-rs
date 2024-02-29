multiversx_sc::imports!();

#[multiversx_sc::contract]
pub trait MockSeedNftMinter {
    #[init]
    fn init(&self, nft_token_id: TokenIdentifier) {
        self.nft_token_id().set(nft_token_id);
    }

    #[endpoint(setNftCount)]
    fn set_nft_count(&self, nft_count: u64) {
        self.nft_count().set(nft_count);
    }

    #[view(getNftCount)]
    #[storage_mapper("nft_count")]
    fn nft_count(&self) -> SingleValueMapper<u64>;

    #[view(getNftTokenId)]
    #[storage_mapper("nft_token_id")]
    fn nft_token_id(&self) -> SingleValueMapper<TokenIdentifier>;
}
