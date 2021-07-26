elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[elrond_wasm_derive::module]
pub trait GlobalOperationModule {
    #[only_owner]
    #[endpoint(startGlobalOperation)]
    fn global_op_start(&self) -> SCResult<()> {
        require!(
            !self.global_op_is_ongoing().get(),
            "Global operation already ongoing"
        );
        self.global_op_is_ongoing().set(&true);
        Ok(())
    }

    #[only_owner]
    #[endpoint(stopGlobalOperation)]
    fn global_op_stop(&self) -> SCResult<()> {
        require!(
            self.global_op_is_ongoing().get(),
            "Global operation not ongoing"
        );
        self.global_op_is_ongoing().set(&false);
        Ok(())
    }

    fn require_global_op_not_ongoing(&self) -> SCResult<()> {
        require!(
            !self.global_op_is_ongoing().get(),
            "Global operation ongoing"
        );
        Ok(())
    }

    fn require_global_op_ongoing(&self) -> SCResult<()> {
        require!(
            self.global_op_is_ongoing().get(),
            "Global operation not ongoing"
        );
        Ok(())
    }

    #[storage_mapper("global_operation_ongoing")]
    fn global_op_is_ongoing(&self) -> SingleValueMapper<Self::Storage, bool>;
}
