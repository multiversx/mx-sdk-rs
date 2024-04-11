use alloc::boxed::Box;

use crate::api::VMApi;

/// CallableContract is the means by which the debugger calls methods in the contract.
pub trait CallableContract: Send + Sync {
    fn call(&self, fn_name: &str) -> bool;
}

/// Describes objects that can create instances of contract objects, with the given API.
pub trait CallableContractBuilder {
    fn new_contract_obj<A: VMApi + Send + Sync>(&self) -> Box<dyn CallableContract>;
}
