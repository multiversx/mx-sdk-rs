multiversx_sc::imports!();
multiversx_sc::derive_imports!();

static ESDT_TRANSFER_FUNC_NAME: &[u8] = b"ESDTTransfer";

const GAS_LIMIT_ESDT_TRANSFER: u64 = 50_0000;
const CALLBACK_ESDT_TRANSFER_GAS_LIMIT: u64 = 100_000;

#[derive(TopEncode, TopDecode)]
pub enum TransferResult {
    None,
    Success,
    Fail,
}

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

    #[endpoint(transferFungiblePromiseWithCallback)]
    fn transfer_fungible_promise_with_callback(&self, to: ManagedAddress, amount: BigUint) {
        let token_id = self.fungible_esdt_token_id().get_token_id();
        self.tx()
            .to(to)
            .raw_call(ESDT_TRANSFER_FUNC_NAME)
            .argument(&token_id)
            .argument(&amount)
            .gas(GAS_LIMIT_ESDT_TRANSFER)
            .callback(self.callbacks().transfer_callback())
            .gas_for_callback(CALLBACK_ESDT_TRANSFER_GAS_LIMIT)
            .register_promise();
    }

    #[promises_callback]
    fn transfer_callback(&self, #[call_result] result: ManagedAsyncCallResult<()>) {
        match result {
            ManagedAsyncCallResult::Ok(()) => {
                self.latest_transfer_result().set(TransferResult::Success);
            },
            ManagedAsyncCallResult::Err(_) => {
                self.latest_transfer_result().set(TransferResult::Fail);
            },
        }
    }

    #[storage_mapper("fungibleEsdtTokenId")]
    fn fungible_esdt_token_id(&self) -> FungibleTokenMapper;

    #[storage_mapper("nonFungibleEsdtTokenId")]
    fn non_fungible_esdt_token_id(&self) -> NonFungibleTokenMapper;

    #[storage_mapper("latestTransferResult")]
    fn latest_transfer_result(&self) -> SingleValueMapper<TransferResult>;
}
