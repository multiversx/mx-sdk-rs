use alloc::boxed::Box;

/// Governs the instantiation process of contract implementation.
/// Using a trait for the instantiation helps with code generation.
pub trait ContractImpl<A> {
	fn new_contract_impl(api: A) -> Self;
}

pub fn new_contract_impl<A, C: ContractImpl<A>>(api: A) -> C {
	C::new_contract_impl(api)
}

/// CallableContract is the means by which the debugger calls methods in the contract.
pub trait CallableContract<A> {
	fn call(&self, fn_name: &[u8]) -> bool;

	fn clone_contract(&self) -> Box<dyn CallableContract<A>>;

	fn into_api(self: Box<Self>) -> A;
}
