multiversx_sc::imports!();

use super::storage;

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
pub trait ForwarderEsdtModule: storage::ForwarderStorageModule {
    #[view(getFungibleEsdtBalance)]
    fn get_fungible_esdt_balance(&self, token_identifier: &TokenIdentifier) -> BigUint {
        self.blockchain()
            .get_esdt_balance(&self.blockchain().get_sc_address(), token_identifier, 0)
    }

    #[view(getCurrentNftNonce)]
    fn get_current_nft_nonce(&self, token_identifier: &TokenIdentifier) -> u64 {
        self.blockchain()
            .get_current_esdt_nft_nonce(&self.blockchain().get_sc_address(), token_identifier)
    }

    #[endpoint]
    fn send_esdt(&self, to: &ManagedAddress, token_id: TokenIdentifier, amount: &BigUint) {
        self.send().direct_esdt(to, &token_id, 0, amount);
    }

    #[payable("*")]
    #[endpoint]
    fn send_esdt_with_fees(&self, to: ManagedAddress, percentage_fees: BigUint) {
        let (token_id, payment) = self.call_value().single_fungible_esdt();
        let fees = &payment * &percentage_fees / PERCENTAGE_TOTAL;
        let amount_to_send = payment - fees;

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
        token_payments: MultiValueEncoded<MultiValue3<TokenIdentifier, u64, BigUint>>,
    ) {
        let mut all_token_payments = ManagedVec::new();

        for multi_arg in token_payments.into_iter() {
            let (token_identifier, token_nonce, amount) = multi_arg.into_tuple();
            let payment = EsdtTokenPayment::new(token_identifier, token_nonce, amount);

            all_token_payments.push(payment);
        }

        let _ = self.send_raw().multi_esdt_transfer_execute(
            &to,
            &all_token_payments,
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
        let issue_cost = self.call_value().egld_value();
        let caller = self.blockchain().get_caller();

        self.send()
            .esdt_system_sc_proxy()
            .issue_fungible(
                issue_cost.clone_value(),
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
                self.last_issued_token()
                    .set(&token_identifier.unwrap_esdt());
                self.last_error_message().clear();
            },
            ManagedAsyncCallResult::Err(message) => {
                // return issue cost to the caller
                if token_identifier.is_egld() && returned_tokens > 0 {
                    self.send().direct_egld(caller, &returned_tokens);
                }

                self.last_error_message().set(&message.err_msg);
            },
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

    #[view]
    fn get_esdt_local_roles(&self, token_id: TokenIdentifier) -> MultiValueEncoded<ManagedBuffer> {
        let roles = self.blockchain().get_esdt_local_roles(&token_id);
        let mut result = MultiValueEncoded::new();
        for role in roles.iter_roles() {
            result.push(role.as_role_name().into());
        }
        result
    }

    #[view]
    fn get_esdt_token_data(
        &self,
        address: ManagedAddress,
        token_id: TokenIdentifier,
        nonce: u64,
    ) -> EsdtTokenDataMultiValue<Self::Api> {
        let token_data = self
            .blockchain()
            .get_esdt_token_data(&address, &token_id, nonce);

        (
            token_data.token_type,
            token_data.amount,
            token_data.frozen,
            token_data.hash,
            token_data.name,
            token_data.attributes,
            token_data.creator,
            token_data.royalties,
            token_data.uris,
        )
            .into()
    }

    #[view]
    fn is_esdt_frozen(
        &self,
        address: &ManagedAddress,
        token_id: &TokenIdentifier,
        nonce: u64,
    ) -> bool {
        self.blockchain().is_esdt_frozen(address, token_id, nonce)
    }

    #[view]
    fn is_esdt_paused(&self, token_id: &TokenIdentifier) -> bool {
        self.blockchain().is_esdt_paused(token_id)
    }

    #[view]
    fn is_esdt_limited_transfer(&self, token_id: &TokenIdentifier) -> bool {
        self.blockchain().is_esdt_limited_transfer(token_id)
    }

    #[view]
    fn validate_token_identifier(&self, token_id: TokenIdentifier) -> bool {
        token_id.is_valid_esdt_identifier()
    }
}
