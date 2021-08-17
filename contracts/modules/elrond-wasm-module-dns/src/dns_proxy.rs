elrond_wasm::imports!();

#[elrond_wasm::proxy]
pub trait Dns {
    #[payable("EGLD")]
    #[endpoint]
    fn register(&self, name: BoxedBytes, #[payment] payment: BigUint);
}
