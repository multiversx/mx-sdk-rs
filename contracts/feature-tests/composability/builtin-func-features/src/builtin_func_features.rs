#![no_std]

pub mod builtin_func_proxy;

multiversx_sc::imports!();

/// Test contract for investigating async calls.
#[multiversx_sc::contract]
pub trait BuiltinFuncFeatures {
    #[proxy]
    fn builtin_func_proxy(&self, to: ManagedAddress) -> builtin_func_proxy::Proxy<Self::Api>;

    #[init]
    fn init(&self) {}

    #[endpoint]
    fn call_set_user_name(&self, address: ManagedAddress, name: ManagedBuffer) {
        self.builtin_func_proxy(address)
            .set_user_name(&name)
            .async_call()
            .call_and_exit()
    }

    #[endpoint]
    fn call_delete_user_name(&self, address: ManagedAddress) {
        self.builtin_func_proxy(address)
            .delete_user_name()
            .async_call()
            .call_and_exit();
    }
}
