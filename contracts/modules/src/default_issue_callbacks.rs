multiversx_sc::imports!();
multiversx_sc::derive_imports!();

// Always keep in sync with the token-related storage mappers. Only modify if really necessary.
#[multiversx_sc::module]
pub trait DefaultIssueCallbacksModule {
    #[callback]
    fn default_issue_cb(
        &self,
        initial_caller: ManagedAddress,
        storage_key: ManagedBuffer,
        #[call_result] result: ManagedAsyncCallResult<TokenIdentifier>,
    ) {
        let mapper =
            SingleValueMapper::<Self::Api, TokenMapperState<Self::Api>>::new(storage_key.into());
        match result {
            ManagedAsyncCallResult::Ok(token_id) => {
                mapper.set(TokenMapperState::Token(token_id));
            },
            ManagedAsyncCallResult::Err(_) => {
                self.return_failed_issue_funds(initial_caller);
            },
        }
    }

    #[callback]
    fn default_issue_init_supply_cb(
        &self,
        initial_caller: ManagedAddress,
        storage_key: ManagedBuffer,
        #[call_result] result: ManagedAsyncCallResult<()>,
    ) {
        let mapper =
            SingleValueMapper::<Self::Api, TokenMapperState<Self::Api>>::new(storage_key.into());
        match result {
            ManagedAsyncCallResult::Ok(()) => {
                let token_id = self.call_value().single_esdt().token_identifier;
                mapper.set(TokenMapperState::Token(token_id));
            },
            ManagedAsyncCallResult::Err(_) => {
                self.return_failed_issue_funds(initial_caller);
            },
        }
    }

    fn return_failed_issue_funds(&self, initial_caller: ManagedAddress) {
        let egld_returned = self.call_value().egld_value();
        if *egld_returned > 0u32 {
            self.send().direct_egld(&initial_caller, &egld_returned);
        }
    }
}
