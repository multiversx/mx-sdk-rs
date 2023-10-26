use alloc::boxed::Box;

/// CallableContract is the means by which the debugger calls methods in the contract.
pub trait CallableContract: Send + Sync {
    fn call(&self, fn_name: &str) -> bool;
}

/// Describes objects that can create instances of contract objects, with the given API.
pub trait CallableContractBuilder {
    fn new_contract_obj(&self) -> Box<dyn CallableContract>;
}
