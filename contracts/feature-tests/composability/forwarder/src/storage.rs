elrond_wasm::imports!();

#[elrond_wasm::module]
pub trait ForwarderStorageModule {
	#[view(lastIssuedToken)]
	#[storage_mapper("lastIssuedToken")]
	fn last_issued_token(&self) -> SingleValueMapper<Self::Storage, TokenIdentifier>;

	#[view(lastErrorMessage)]
	#[storage_mapper("lastErrorMessage")]
	fn last_error_message(&self) -> SingleValueMapper<Self::Storage, BoxedBytes>;
}
