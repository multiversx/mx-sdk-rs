#![no_std]

elrond_wasm::imports!();

/// General test contract.
/// Used especially for investigating async calls and contract interaction in general.
#[elrond_wasm_derive::contract(VaultImpl)]
pub trait Vault {
	#[init]
	fn init(&self) {}

	#[endpoint]
	fn echo_arguments(
		&self,
		#[var_args] args: VarArgs<BoxedBytes>,
	) -> MultiResultVec<BoxedBytes> {
		args.into_vec().into()
	}

	#[payable("*")]
	#[endpoint(storeFunds)]
	fn store_funds(&self) {}

	#[endpoint(retrieveFunds)]
	fn retrieve_funds(&self, token: TokenIdentifier, amount: BigUint) {
		self.send().direct(&self.get_caller(), &token, &amount, &[]);
	}
}
