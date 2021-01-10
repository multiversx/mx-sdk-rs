mod contract_abi_json;
mod endpoint_abi_json;
mod type_abi_json;

pub use contract_abi_json::*;
pub use endpoint_abi_json::*;
pub use type_abi_json::*;

/// Function provided for convenience.
/// Yields the ABI JSON of a contract as string.
pub fn contract_abi<C: elrond_wasm::ContractWithAbi>(contract: &C) -> String {
	let abi = contract.abi(true);
	serialize_abi_to_json(&abi)
}
