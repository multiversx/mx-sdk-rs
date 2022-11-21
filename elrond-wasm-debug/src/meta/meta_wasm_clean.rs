use std::{fs, process::Command};

use super::meta_config::{ContractMetadata, MetaConfig};

impl MetaConfig {
    pub fn clean_wasm(&self) {
        for output_contract in &self.output_contracts.contracts {
            output_contract.cargo_clean();
        }

        fs::remove_dir_all(&self.output_dir).expect("failed to remove output directory");
    }
}
