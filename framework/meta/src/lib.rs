pub mod cli;
pub mod cmd;
pub mod folder_structure;

pub use multiversx_sc_meta_lib::abi_json;
pub use multiversx_sc_meta_lib::ei;
pub use multiversx_sc_meta_lib::ei_check_json;
pub use multiversx_sc_meta_lib::version;
pub use multiversx_sc_meta_lib::version_history;

/// Backwards compatibility, please use `multiversx_sc_meta_lib::cli_main::<AbiObj>()`.
///
/// Failure to do so will result in slower build time.
#[deprecated(
    since = "0.41.0",
    note = "Backwards compatibility only, please use `cli_main` from crate `multiversx-sc-meta-lib` instead."
)]
pub fn cli_main<AbiObj: multiversx_sc::contract_base::ContractAbiProvider>() {
    multiversx_sc_meta_lib::cli_main::<AbiObj>()
}

/// Backwards compatibility, please use `multiversx_sc_meta_lib::multi_contract_config::<AbiObj>(contract_crate_path)`.
#[deprecated(
    since = "0.41.0",
    note = "Backwards compatibility only, please use `multi_contract_config` from crate `multiversx-sc-meta-lib` instead."
)]
pub fn multi_contract_config<AbiObj: multiversx_sc::contract_base::ContractAbiProvider>(
    contract_crate_path: &std::path::Path,
) -> multiversx_sc_meta_lib::contract::sc_config::ScConfig {
    multiversx_sc_meta_lib::multi_contract_config::<AbiObj>(contract_crate_path)
}
