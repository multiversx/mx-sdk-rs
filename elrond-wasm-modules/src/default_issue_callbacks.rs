elrond_wasm::imports!();

// Always keep in sync with the token-related storage mappers. Only modify if really necessary.
#[elrond_wasm::module]
pub trait DefaultIssueCallbacksModule {
    #[callback]
    fn default_fungible_issue_cb(
        &self,
        initial_caller: ManagedAddress,
        storage_key: ManagedBuffer,
        #[call_result] result: ManagedAsyncCallResult<TokenIdentifier>,
    ) {
        match result {
            ManagedAsyncCallResult::Ok(token_id) => {
                let token_mapper = FungibleTokenMapper::new(storage_key.into());
                token_mapper.set_token_id(&token_id);
            },
            ManagedAsyncCallResult::Err(_) => {
                self.return_failed_issue_funds(initial_caller);
            },
        }
    }

    #[callback]
    fn default_fungible_init_supply_cb(
        &self,
        initial_caller: ManagedAddress,
        storage_key: ManagedBuffer,
        #[call_result] result: ManagedAsyncCallResult<()>,
    ) {
        match result {
            ManagedAsyncCallResult::Ok(()) => {
                let token_id = self.call_value().token();
                let token_mapper = FungibleTokenMapper::new(storage_key.into());
                token_mapper.set_token_id(&token_id);
            },
            ManagedAsyncCallResult::Err(_) => {
                self.return_failed_issue_funds(initial_caller);
            },
        }
    }

    fn return_failed_issue_funds(&self, initial_caller: ManagedAddress) {
        let egld_returned = self.call_value().egld_value();
        if egld_returned > 0u32 {
            self.send()
                .direct_egld(&initial_caller, &egld_returned, &[]);
        }
    }
}
