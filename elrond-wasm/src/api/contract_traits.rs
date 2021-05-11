use alloc::boxed::Box;

/// Governs the instantiation process of contract implementation.
/// Using a trait for the instantiation helps with code generation.
pub trait ContractImpl {
	type Api;

	fn new_contract_impl(api: Self::Api) -> Self;
}

pub fn new_contract_impl<A, C: ContractImpl<Api = A>>(api: A) -> C {
	C::new_contract_impl(api)
}

/// CallableContract is the means by which the debugger calls methods in the contract.
pub trait CallableContract<A> {
	fn call(&self, fn_name: &[u8]) -> bool;

	fn into_api(self: Box<Self>) -> A;
}
