mod contract_abi_json;
mod endpoint_abi_json;
mod type_abi_json;

pub use contract_abi_json::*;
use elrond_wasm::contract_base::ContractAbiProvider;
pub use endpoint_abi_json::*;
pub use type_abi_json::*;

/// Function provided for convenience.
/// Yields the ABI JSON of a contract as string.
pub fn contract_abi<AbiObj: ContractAbiProvider>() -> String {
    let abi = <AbiObj as ContractAbiProvider>::abi();
    serialize_abi_to_json(&abi)
}

/// Function provided for convenience.
/// Prints the ABI JSON of a contract to console.
pub fn print_abi<AbiTrait: ContractAbiProvider>() {
    print!("{}", contract_abi::<AbiTrait>());
}
