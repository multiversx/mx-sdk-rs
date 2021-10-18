mod build_info_abi_json;
mod contract_abi_json;
mod endpoint_abi_json;
mod type_abi_json;

pub use build_info_abi_json::{BuildInfoAbiJson, RustcAbiJson};
pub use contract_abi_json::*;
use elrond_wasm::contract_base::ContractAbiProvider;
pub use endpoint_abi_json::*;
pub use type_abi_json::*;

/// Function provided for convenience.
/// Yields the ABI JSON of a contract as string.
pub fn contract_abi<AbiObj: ContractAbiProvider>() -> String {
    let abi = <AbiObj as ContractAbiProvider>::abi();
    let abi_json = ContractAbiJson::from(&abi);
    serialize_abi_to_json(&abi_json)
}

/// Function provided for convenience.
/// Prints the ABI JSON of a contract to console.
pub fn print_abi<AbiTrait: ContractAbiProvider>() {
    println!("{}", contract_abi::<AbiTrait>());
}

/// Same as `contract_abi`, but allows caller to replace the compiler metadata,
/// so that ABI tests are deterministc and independent on compiler version.
pub fn contract_abi_dummy_environment<AbiObj: ContractAbiProvider>() -> String {
    let abi = <AbiObj as ContractAbiProvider>::abi();
    let mut abi_json = ContractAbiJson::from(&abi);
    abi_json.build_info.rustc = RustcAbiJson {
        version: "x.x.x-nightly".to_string(),
        commit_hash: "<commit hash here>".to_string(),
        commit_date: "<commit date here>".to_string(),
        channel: "Channel".to_string(),
        short: "rustc <version> (<short hash> <date>)".to_string(),
    };
    serialize_abi_to_json(&abi_json)
}
