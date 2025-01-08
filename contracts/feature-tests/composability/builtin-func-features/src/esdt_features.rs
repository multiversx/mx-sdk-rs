multiversx_sc::imports!();

static ESDT_TRANSFER_FUNC_NAME: &[u8] = b"ESDTTransfer";
// static ESDT_NFT_TRANSFER_FUNC_NAME: &[u8] = b"ESDTNFTTransfer";

const GAS_LIMIT_ESDT_TRANSFER: u64 = 50_0000;
// const CALLBACK_ESDT_TRANSFER_GAS_LIMIT: u64 = 100_000; // TODO: Change if needed

#[multiversx_sc::module]
pub trait EsdtFeaturesModule {
    #[endpoint(transferFungiblePromiseNoCallback)]
    fn transfer_fungible_promise_no_callback(&self, to: ManagedAddress, amount: BigUint) {
        let token_id = self.fungible_esdt_token_id().get_token_id();
        self.tx()
            .to(to)
            .raw_call(ESDT_TRANSFER_FUNC_NAME)
            .argument(&token_id)
            .argument(&amount)
            .gas(GAS_LIMIT_ESDT_TRANSFER)
            .register_promise();
    }

    #[storage_mapper("fungibleEsdtTokenId")]
    fn fungible_esdt_token_id(&self) -> FungibleTokenMapper;

    #[storage_mapper("nonFungibleEsdtTokenId")]
    fn non_fungible_esdt_token_id(&self) -> NonFungibleTokenMapper;
}
