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
	#[endpoint]
	fn accept_funds(&self) {}
	
	#[payable("*")]
	#[endpoint]
	fn reject_funds(&self) -> SCResult<()> {
		sc_error!("reject_funds")
	}

	#[endpoint]
	fn retrieve_funds(&self, token: TokenIdentifier, amount: BigUint, #[var_args] return_message: OptionalArg<BoxedBytes>) {
		let data = match &return_message {
			OptionalArg::Some(data) => data.as_slice(),
			OptionalArg::None => &[],
		};
		self.send().direct_via_async_call(&self.get_caller(), &token, &amount, data);
	}
}
