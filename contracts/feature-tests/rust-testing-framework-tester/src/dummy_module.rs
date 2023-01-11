multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait DummyModule {
    fn some_function(&self) -> BigUint {
        BigUint::zero()
    }
}
