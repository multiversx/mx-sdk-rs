elrond_wasm::imports!();

#[elrond_wasm_derive::callable(VaultProxy)]
pub trait Vault {
	fn echo_arguments(
		&self,
		args: &VarArgs<BoxedBytes>,
	) -> ContractCall<BigUint, MultiResultVec<BoxedBytes>>;

	#[payable("*")]
	fn accept_funds(&self) -> ContractCall<BigUint, ()>;

	#[payable("*")]
	fn reject_funds(&self) -> ContractCall<BigUint, ()>;

	fn retrieve_funds(&self, token: TokenIdentifier, amount: BigUint) -> ContractCall<BigUint, ()>;
}
