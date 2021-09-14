#![no_std]

mod dns_proxy;

elrond_wasm::imports!();

/// The module deals with registering usernames in a DNS contract.
#[elrond_wasm::module]
pub trait DnsModule {
    #[proxy]
    fn dns_proxy(&self, to: ManagedAddress) -> dns_proxy::Proxy<Self::Api>;

    #[payable("EGLD")]
    #[endpoint(dnsRegister)]
    fn dns_register(
        &self,
        dns_address: ManagedAddress,
        name: BoxedBytes,
        #[payment] payment: BigUint,
    ) -> SCResult<AsyncCall> {
        only_owner!(self, "only owner can call dnsRegister");

        Ok(self
            .dns_proxy(dns_address)
            .register(name, payment)
            .async_call())
    }
}
