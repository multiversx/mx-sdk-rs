/// CallableContract is the means by which the debugger calls methods in the contract.
pub trait CallableContract<A> {
    fn call(&self, fn_name: &[u8]) -> bool;
}
