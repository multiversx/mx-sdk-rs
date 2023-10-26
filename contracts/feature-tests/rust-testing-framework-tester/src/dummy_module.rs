multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait DummyModule {
    fn some_function(&self) -> BaseBigUint {
        BaseBigUint::zero()
    }
}
