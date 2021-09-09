elrond_wasm::imports!();

use super::storage;

#[elrond_wasm::module]
pub trait ForwarderSftModule: storage::ForwarderStorageModule {
    #[payable("EGLD")]
    #[endpoint]
    fn sft_issue(
        &self,
        #[payment] issue_cost: BigUint,
        token_display_name: ManagedBuffer,
        token_ticker: ManagedBuffer,
    ) -> AsyncCall<Self::SendApi> {
        let caller = self.blockchain().get_caller();

        ESDTSystemSmartContractProxy::new_proxy_obj(self.send())
            .issue_semi_fungible(
                issue_cost,
                &token_display_name,
                &token_ticker,
                SemiFungibleTokenProperties {
                    can_freeze: true,
                    can_wipe: true,
                    can_pause: true,
                    can_change_owner: true,
                    can_upgrade: true,
                    can_add_special_roles: true,
                },
            )
            .async_call()
            .with_callback(self.callbacks().sft_issue_callback(&caller))
    }

    #[callback]
    fn sft_issue_callback(
        &self,
        caller: &ManagedAddress,
        #[call_result] result: AsyncCallResult<TokenIdentifier>,
    ) {
        match result {
            AsyncCallResult::Ok(token_identifier) => {
                self.last_issued_token().set(&token_identifier);
                self.last_error_message().clear();
            },
            AsyncCallResult::Err(message) => {
                // return issue cost to the caller
                let (returned_tokens, token_identifier) = self.call_value().payment_token_pair();
                if token_identifier.is_egld() && returned_tokens > 0 {
                    self.send().direct_egld(caller, &returned_tokens, &[]);
                }

                self.last_error_message().set(&message.err_msg);
            },
        }
    }
}
