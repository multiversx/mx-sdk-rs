#![no_std]

use multiversx_sc::imports::*;

pub mod proxy_proxy_test;
pub mod returns_token_identifier_test_proxy;

/// One of the simplest smart contracts possible,
/// it holds a single variable in storage, which anyone can increment.
#[multiversx_sc::contract]
pub trait ReturnsTokenIdentifierTest {
    #[view(lastIssuedToken)]
    #[storage_mapper("lastIssuedToken")]
    fn last_issued_token(&self) -> SingleValueMapper<TokenIdentifier>;

    #[view(lastErrorMessage)]
    #[storage_mapper("lastErrorMessage")]
    fn last_error_message(&self) -> SingleValueMapper<ManagedBuffer>;

    #[view(getSum)]
    #[storage_mapper("sum")]
    fn sum(&self) -> SingleValueMapper<BigUint>;

    #[init]
    fn init(&self, initial_value: BigUint) {
        self.sum().set(initial_value);
    }

    #[upgrade]
    fn upgrade(&self, initial_value: BigUint) {
        self.init(initial_value);
    }

    /// Add desired amount to the storage variable.
    #[endpoint]
    fn add(&self, value: BigUint) {
        self.sum().update(|sum| *sum += value);
    }

    #[payable("*")]
    #[endpoint]
    fn call_contract(
        &self,
        contract_address: &ManagedAddress,
        token_display_name: ManagedBuffer,
        token_ticker: ManagedBuffer,
        initial_supply: BigUint,
    ) {
        let caller = self.blockchain().get_caller();
        self.tx()
            .to(contract_address)
            .typed(proxy_proxy_test::ProxyTestProxy)
            .issue_fungible_token(token_display_name, token_ticker, initial_supply)
            .with_callback(self.callbacks().esdt_issue_callback(&caller))
            .async_call_and_exit();
    }

    #[callback]
    fn esdt_issue_callback(
        &self,
        caller: &ManagedAddress,
        #[call_result] result: ManagedAsyncCallResult<()>,
    ) {
        let (token_identifier, returned_tokens) = self.call_value().egld_or_single_fungible_esdt();
        // callback is called with ESDTTransfer of the newly issued token, with the amount requested,
        // so we can get the token identifier and amount from the call data
        match result {
            ManagedAsyncCallResult::Ok(()) => {
                self.last_issued_token().set(token_identifier.unwrap_esdt());
                self.last_error_message().clear();
            },
            ManagedAsyncCallResult::Err(message) => {
                // return issue cost to the caller
                if token_identifier.is_egld() && returned_tokens > 0 {
                    self.tx().to(caller).egld(&returned_tokens).transfer();
                }

                self.last_error_message().set(&message.err_msg);
            },
        }
    }
    // erd1qqqqqqqqqqqqqpgqyu0yylry6dyvynsqsmswqzvyewes38j0a4sqevrx55
}
