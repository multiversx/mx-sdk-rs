multiversx_sc::imports!();

use super::storage;

#[multiversx_sc::module]
pub trait ForwarderSftModule: storage::ForwarderStorageModule {
    #[payable("EGLD")]
    #[endpoint]
    fn sft_issue(&self, token_display_name: ManagedBuffer, token_ticker: ManagedBuffer) {
        let issue_cost = self.call_value().egld_value();
        let caller = self.blockchain().get_caller();

        self.send()
            .esdt_system_sc_proxy()
            .issue_semi_fungible(
                issue_cost.clone_value(),
                &token_display_name,
                &token_ticker,
                SemiFungibleTokenProperties {
                    can_freeze: true,
                    can_wipe: true,
                    can_pause: true,
                    can_transfer_create_role: true,
                    can_change_owner: true,
                    can_upgrade: true,
                    can_add_special_roles: true,
                },
            )
            .async_call()
            .with_callback(self.callbacks().sft_issue_callback(&caller))
            .call_and_exit()
    }

    #[callback]
    fn sft_issue_callback(
        &self,
        caller: &ManagedAddress,
        #[call_result] result: ManagedAsyncCallResult<TokenIdentifier>,
    ) {
        match result {
            ManagedAsyncCallResult::Ok(token_identifier) => {
                self.last_issued_token().set(&token_identifier);
                self.last_error_message().clear();
            },
            ManagedAsyncCallResult::Err(message) => {
                // return issue cost to the caller
                let (token_identifier, returned_tokens) =
                    self.call_value().egld_or_single_fungible_esdt();
                if token_identifier.is_egld() && returned_tokens > 0 {
                    self.send().direct_egld(caller, &returned_tokens);
                }

                self.last_error_message().set(&message.err_msg);
            },
        }
    }
}
