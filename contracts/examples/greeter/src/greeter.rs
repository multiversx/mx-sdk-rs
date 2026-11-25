#![no_std]

use multiversx_sc::imports::*;

pub mod greeter_proxy;

/// A minimal greeter contract. Starts from `sc-meta new --template empty
/// --name greeter` (see this recipe's README for the exact, unmodified
/// output of that command) with one endpoint and one storage mapper added
/// on top — the smallest real next step after scaffolding, and the
/// natural bridge into the storage-mapper-decision-table recipe.
#[multiversx_sc::contract]
pub trait Greeter {
    #[init]
    fn init(&self) {}

    #[upgrade]
    fn upgrade(&self) {}

    /// Stores a greeting message for the calling address. Overwrites any
    /// previous greeting for the same caller.
    #[endpoint(setGreeting)]
    fn set_greeting(&self, message: ManagedBuffer) {
        let caller = self.blockchain().get_caller();
        self.greeting(&caller).set(message);
    }

    /// Reads back the greeting message stored for `address`. Returns an
    /// empty ManagedBuffer if that address never called setGreeting.
    #[view(getGreeting)]
    #[storage_mapper("greeting")]
    fn greeting(&self, address: &ManagedAddress) -> SingleValueMapper<ManagedBuffer>;
}
