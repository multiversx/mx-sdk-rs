use alloc::boxed::Box;

/// CallableContract is the means by which the debugger calls methods in the contract.
pub trait CallableContract<A> {
	fn call(&self, fn_name: &[u8]) -> bool;

	fn clone_contract(&self) -> Box<dyn CallableContract<A>>;

	fn into_api(self: Box<Self>) -> A;
}
