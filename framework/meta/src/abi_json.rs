mod build_info_abi_json;
mod contract_abi_json;
mod endpoint_abi_json;
mod event_abi_json;
mod type_abi_json;

pub use build_info_abi_json::{BuildInfoAbiJson, RustcAbiJson};
pub use contract_abi_json::*;
pub use endpoint_abi_json::*;
pub use event_abi_json::*;
use multiversx_sc::{abi::ContractAbi, contract_base::ContractAbiProvider};
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
/// Will return the main contract ABI + view contract ABI.
pub fn abi_to_json_dummy_environment(contract_abi: &ContractAbi) -> String {
    let mut abi_json = ContractAbiJson::from(contract_abi);
    if let Some(build_info) = &mut abi_json.build_info {
        build_info.contract_crate.git_version = "<git version here>".to_string();
        build_info.rustc = RustcAbiJson {
            version: "x.x.x-nightly".to_string(),
            commit_hash: "<commit hash here>".to_string(),
            commit_date: "<commit date here>".to_string(),
            channel: "Channel".to_string(),
            short: "rustc <version> (<short hash> <date>)".to_string(),
        };
    }
    serialize_abi_to_json(&abi_json)
}
