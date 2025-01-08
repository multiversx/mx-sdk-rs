use crate::dns_proxy;

multiversx_sc::imports!();

/// Standard smart contract module that deals with registering usernames in a DNS contract.
///
/// MultiversX usernames/herotags need to be requested by the beneficiary.
/// For a contract, this means that they need an endpoint via which to request a username from the DNS.
///
#[multiversx_sc::module]
pub trait DnsModule {
    #[payable("EGLD")]
    #[only_owner]
    #[endpoint(dnsRegister)]
    fn dns_register(&self, dns_address: ManagedAddress, name: ManagedBuffer) {
        let payment = self.call_value().egld().clone_value();
        self.tx()
            .to(&dns_address)
            .typed(dns_proxy::DnsProxy)
            .register(name)
            .egld(payment)
            .async_call_and_exit();
    }
}
