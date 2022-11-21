use std::process::Command;

use crate::meta::output_contract::WASM_OPT_NAME;

use super::meta_config::MetaConfig;

impl MetaConfig {
    pub fn build_wasm(&mut self) {
        if self.build_args.wasm_opt && !is_wasm_opt_installed() {
            println!("Warning: {} not installed", WASM_OPT_NAME);
            self.build_args.wasm_opt = false;
        }

        for output_contract in &mut self.output_contracts.contracts {
            output_contract.build_contract(&self.build_args, self.output_dir.as_str());
        }
    }
}

fn is_wasm_opt_installed() -> bool {
    Command::new(WASM_OPT_NAME)
        .args(["--version"])
        .output()
        .is_ok()
}
