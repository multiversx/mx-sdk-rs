elrond_wasm::imports!();

#[elrond_wasm::module]
pub trait DummyModule {
    fn some_function(&self) -> BigUint {
        BigUint::zero()
    }
}
