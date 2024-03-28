#![no_std]

pub mod builtin_func_proxy;

multiversx_sc::imports!();

/// Test contract for investigating async calls.
#[multiversx_sc::contract]
pub trait BuiltinFuncFeatures {
    #[init]
    fn init(&self) {}

    #[endpoint]
    fn call_set_user_name(&self, address: ManagedAddress, name: ManagedBuffer) {
        self.tx()
            .to(&address)
            .typed(builtin_func_proxy::UserBuiltinProxy)
            .set_user_name(name)
            .async_call()
            .call_and_exit()
    }

    #[endpoint]
    fn call_delete_user_name(&self, address: ManagedAddress) {
        self.tx()
            .to(&address)
            .typed(builtin_func_proxy::UserBuiltinProxy)
            .delete_user_name()
            .async_call()
            .call_and_exit()
    }
}
