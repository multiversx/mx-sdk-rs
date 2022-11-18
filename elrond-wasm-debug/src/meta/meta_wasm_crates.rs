use std::{
    fs::{self, File},
    io::Write,
};

use elrond_wasm::abi::EndpointLocationAbi;

use super::meta_config::{ContractMetadata, MetaConfig};

const WASM_LIB_PATH: &str = "../wasm/src/lib.rs";
const WASM_LIB_PATH_NO_MANAGED_EI: &str = "../wasm-no-managed-ei/src/lib.rs";

/// This one is useful for some of the special unmanaged EI tests in the framework.
/// Will do nothing for regular contracts.
pub fn copy_to_wasm_unmanaged_ei() {
    if std::path::Path::new(WASM_LIB_PATH_NO_MANAGED_EI).exists() {
        fs::copy(WASM_LIB_PATH, WASM_LIB_PATH_NO_MANAGED_EI).unwrap();
    }
}

impl MetaConfig {
    pub fn write_wasm_src_lib(&self) {
        for output_contract in &self.output_contracts.contracts {
            output_contract.write_wasm_src_lib();
        }
        // if let Some(main_contract) = &self.main_contract {
        //     write_wasm_src_lib(main_contract);
        // }

        // if let Some(view_contract) = &self.view_contract {
        //     write_wasm_src_lib(view_contract);
        // }
    }
}
