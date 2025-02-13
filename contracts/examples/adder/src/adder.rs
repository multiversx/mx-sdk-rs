#![no_std]

use multiversx_sc::{imports::*, storage::StorageKey};

pub mod adder_proxy;

/// One of the simplest smart contracts possible,
/// it holds a single variable in storage, which anyone can increment.
#[multiversx_sc::contract]
pub trait Adder {
    #[view(getSum)]
    #[storage_mapper("sum")]
    fn sum(&self) -> SingleValueMapper<BigUint>;

    #[init]
    fn init(&self, initial_value: BigUint) {
        self.sum().set(initial_value);
    }

    #[upgrade]
    fn upgrade(&self, initial_value: BigUint) {
        self.init(initial_value);
    }

    // #[endpoint]
    // fn get_addresses_with_transfer_role(
    //     &self,
    //     token_id: TokenIdentifier,
    // ) -> ManagedVec<ManagedAddress> {
    //     let key = ManagedBuffer::new_from_bytes(b"ELRONDtransferesdt");
    //     let base_key = key.concat(token_id.into_managed_buffer());
    //     let remote = SingleValueMapper::<Self::Api, ManagedVec<Self::Api, ManagedAddress>, _>::new_from_address(
    //         SystemSCAddress.to_managed_address(),
    //         StorageKey::from(base_key),
    //     );

    //     remote.get()
    // }

    // #[endpoint]
    // fn get_addresses_with_transfer_role(&self, token_id: TokenIdentifier) -> usize {
    //     let key = ManagedBuffer::new_from_bytes(b"ELRONDtransferesdt");
    //     let base_key = key.concat(token_id.into_managed_buffer());
    //     let remote = UnorderedSetMapper::<Self::Api, ManagedAddress, _>::new_from_address(
    //         SystemSCAddress.to_managed_address(),
    //         StorageKey::from(base_key),
    //     );

    //     remote.len()
    // }

    #[endpoint]
    fn get_addresses_with_transfer_role(&self, token_id: TokenIdentifier) -> ManagedBuffer {
        let key = ManagedBuffer::new_from_bytes(b"ELRONDtransferesdt");
        let base_key = key.concat(token_id.into_managed_buffer());
        let remote = SingleValueMapper::<Self::Api, ManagedBuffer, _>::new_from_address(
            SystemSCAddress.to_managed_address(),
            StorageKey::from(base_key),
        );

        remote.get()
    }

    /// Add desired amount to the storage variable.
    #[endpoint]
    fn add(&self, value: BigUint) {
        self.sum().update(|sum| *sum += value);
    }
}
