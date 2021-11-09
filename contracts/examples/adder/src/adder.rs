#![no_std]

elrond_wasm::imports!();

/// One of the simplest smart contracts possible,
/// it holds a single variable in storage, which anyone can increment.
#[elrond_wasm::derive::contract]
pub trait Adder {
    #[view(getSum)]
    #[storage_mapper("sum")]
    fn sum(&self) -> SingleValueMapper<BigInt>;

    #[init]
    fn init(&self, initial_value: BigInt) {
        self.sum().set(&initial_value);
    }

    /// Add desired amount to the storage variable.
    #[endpoint]
    fn add(&self, value: BigInt) -> SCResult<()> {
        let biguint = BigUint::managed_from(self.type_manager(), 257u64);
        self.print().print_biguint(&biguint);
        self.sum().update(|sum| *sum += value);

        Ok(())
    }
}
