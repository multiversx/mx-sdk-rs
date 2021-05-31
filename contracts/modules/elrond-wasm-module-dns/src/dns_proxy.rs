elrond_wasm::imports!();

#[elrond_wasm_derive::proxy]
pub trait Dns {
	#[payable("EGLD")]
	#[endpoint]
	fn register(&self, name: BoxedBytes, #[payment] payment: Self::BigUint);
}
