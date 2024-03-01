multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::module]
pub trait GlobalOperationModule {
    #[only_owner]
    #[endpoint(startGlobalOperation)]
    fn global_op_start(&self) {
        self.require_global_op_not_ongoing();
        self.global_op_is_ongoing().set(true);
    }

    #[only_owner]
    #[endpoint(stopGlobalOperation)]
    fn global_op_stop(&self) {
        self.require_global_op_ongoing();
        self.global_op_is_ongoing().set(false);
    }

    fn require_global_op_not_ongoing(&self) {
        require!(
            !self.global_op_is_ongoing().get(),
            "Global operation ongoing"
        );
    }

    fn require_global_op_ongoing(&self) {
        require!(
            self.global_op_is_ongoing().get(),
            "Global operation not ongoing"
        );
    }

    #[storage_mapper("global_operation_ongoing")]
    fn global_op_is_ongoing(&self) -> SingleValueMapper<bool>;
}
