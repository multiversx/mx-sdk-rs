pub mod cli;
pub mod cmd;
pub mod folder_structure;

pub use multiversx_sc_meta_lib::abi_json;
pub use multiversx_sc_meta_lib::ei;
pub use multiversx_sc_meta_lib::ei_check_json;
pub use multiversx_sc_meta_lib::version;
pub use multiversx_sc_meta_lib::version_history;

/// Backwards compatibility, please use `multiversx_sc_meta_lib::cli_main::<AbiObj>()`.
pub fn cli_main<AbiObj: multiversx_sc::contract_base::ContractAbiProvider>() {
    multiversx_sc_meta_lib::cli_main::<AbiObj>()
}

/// Backwards compatibility, please use `multiversx_sc_meta_lib::multi_contract_config::<AbiObj>(contract_crate_path)`.
pub fn multi_contract_config<AbiObj: multiversx_sc::contract_base::ContractAbiProvider>(
    contract_crate_path: &std::path::Path,
) -> multiversx_sc_meta_lib::contract::sc_config::ScConfig {
    multiversx_sc_meta_lib::multi_contract_config::<AbiObj>(contract_crate_path)
}
