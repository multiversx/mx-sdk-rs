multiversx_sc::imports!();

use super::fwd_storage_legacy;

const PERCENTAGE_TOTAL: u64 = 10_000; // 100%

pub type EsdtTokenDataMultiValue<M> = MultiValue9<
    EsdtTokenType,
    BigUint<M>,
    bool,
    ManagedBuffer<M>,
    ManagedBuffer<M>,
    ManagedBuffer<M>,
    ManagedAddress<M>,
    BigUint<M>,
    ManagedVec<M, ManagedBuffer<M>>,
>;

#[multiversx_sc::module]
pub trait ForwarderEsdtModule: fwd_storage_legacy::ForwarderStorageModule {
    #[endpoint]
    fn send_esdt(&self, to: &ManagedAddress, token_id: TokenIdentifier, amount: &BigUint) {
        self.send().direct_esdt(to, &token_id, 0, amount);
    }

    #[payable("*")]
    #[endpoint]
    fn send_esdt_with_fees(&self, to: ManagedAddress, percentage_fees: u32) {
        let (token_id, payment) = self.call_value().single_fungible_esdt();
        let fees = &*payment * percentage_fees / PERCENTAGE_TOTAL;
        let amount_to_send = payment.clone() - fees;

        self.send().direct_esdt(&to, &token_id, 0, &amount_to_send);
    }

    #[endpoint]
    fn send_esdt_twice(
        &self,
        to: &ManagedAddress,
        token_id: TokenIdentifier,
        amount_first_time: &BigUint,
        amount_second_time: &BigUint,
    ) {
        self.send().direct_esdt(to, &token_id, 0, amount_first_time);
        self.send()
            .direct_esdt(to, &token_id, 0, amount_second_time);
    }

    #[endpoint]
    fn send_esdt_direct_multi_transfer(
        &self,
        to: ManagedAddress,
        payment_args: MultiValueEncoded<MultiValue3<TokenIdentifier, u64, BigUint>>,
    ) {
        self.send_raw().multi_esdt_transfer_execute(
            &to,
            &payment_args.convert_payment_multi_triples(),
            self.blockchain().get_gas_left(),
            &ManagedBuffer::new(),
            &ManagedArgBuffer::new(),
        );
    }

    #[payable("EGLD")]
    #[endpoint]
    fn issue_fungible_token(
        &self,
        token_display_name: ManagedBuffer,
        token_ticker: ManagedBuffer,
        initial_supply: BigUint,
    ) {
        let issue_cost = self.call_value().egld();
        let caller = self.blockchain().get_caller();

        self.send()
            .esdt_system_sc_proxy()
            .issue_fungible(
                issue_cost.clone(),
                &token_display_name,
                &token_ticker,
                &initial_supply,
                FungibleTokenProperties {
                    num_decimals: 0,
                    can_freeze: true,
                    can_wipe: true,
                    can_pause: true,
                    can_mint: true,
                    can_burn: true,
                    can_change_owner: true,
                    can_upgrade: true,
                    can_add_special_roles: true,
                },
            )
            .async_call()
            .with_callback(self.callbacks().esdt_issue_callback(&caller))
            .call_and_exit()
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
            }
            ManagedAsyncCallResult::Err(message) => {
                // return issue cost to the caller
                if token_identifier.is_egld() && returned_tokens > 0 {
                    self.send().direct_egld(caller, &returned_tokens);
                }

                self.last_error_message().set(&message.err_msg);
            }
        }
    }

    #[endpoint]
    fn local_mint(&self, token_identifier: TokenIdentifier, amount: BigUint) {
        self.send().esdt_local_mint(&token_identifier, 0, &amount);
    }

    #[endpoint]
    fn local_burn(&self, token_identifier: TokenIdentifier, amount: BigUint) {
        self.send().esdt_local_burn(&token_identifier, 0, &amount);
    }
}
