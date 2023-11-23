#![no_std]

multiversx_sc::imports!();

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

    /// Add desired amount to the storage variable.
    #[endpoint]
    fn add(&self, value: BigUint) {
        self.sum().update(|sum| *sum += value);
        let (_, _) = self
            .tx()
            // .esdt(EsdtTokenPayment::new(TokenIdentifier::from(""), 0u64, BigUint::zero()))
            .egld(BigUint::from(5u64))
            .deploy(
                ManagedBuffer::new(),
                CodeMetadata::DEFAULT,
                ManagedArgBuffer::new(),
            )
            .execute_deploy();
    }

    
}
