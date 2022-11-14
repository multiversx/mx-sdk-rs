use std::{fs, process::Command};

use super::meta_config::{ContractMetadata, MetaConfig};

pub fn clean_contract_wasm(contract_metadata: &ContractMetadata) {
    let exit_status = Command::new("cargo")
        .args(["clean"])
        .current_dir(contract_metadata.wasm_crate_path.as_str())
        .spawn()
        .expect("failed to spawn contract clean process")
        .wait()
        .expect("contract clean process was not running");

    assert!(exit_status.success(), "contract clean process failed");
}

impl MetaConfig {
    pub fn clean_wasm(&self) {
        for contract in &self.contracts{
            clean_contract_wasm(contract);
        }

        fs::remove_dir_all(&self.output_dir).expect("failed to remove output directory");
    }
}
