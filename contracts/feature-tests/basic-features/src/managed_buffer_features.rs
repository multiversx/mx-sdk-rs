elrond_wasm::imports!();

#[elrond_wasm::module]
pub trait ManagedBufferFeatures {
    #[endpoint]
    fn concat(&self, a: &[u8], b: &[u8]) -> BoxedBytes {
        // a.sqrt()
    }
}
