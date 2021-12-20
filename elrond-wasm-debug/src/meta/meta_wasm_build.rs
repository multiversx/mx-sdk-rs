use std::{fs, process::Command};

use super::meta_config::{BuildArgs, ContractMetadata, MetaConfig};

fn build_contract(contract_metadata: &ContractMetadata, build_args: &BuildArgs, output_path: &str) {
    let mut command = Command::new("cargo");
    command
        .args(["build", "--target=wasm32-unknown-unknown", "--release"])
        .current_dir(&contract_metadata.wasm_crate_path);
    if !build_args.debug_symbols {
        command.env("RUSTFLAGS", "-C link-arg=-s");
    }
    let exit_status = command
        .spawn()
        .expect("failed to spawn contract build process")
        .wait()
        .expect("contract build process was not running");

    assert!(exit_status.success(), "contract build process failed");

    let source_wasm_path = contract_metadata.wasm_compilation_output_path();
    let dest_wasm_name = build_args.wasm_name(contract_metadata);
    let dest_wasm_path = format!("{}/{}", output_path, dest_wasm_name);
    fs::copy(source_wasm_path, dest_wasm_path)
        .expect("failed to copy compiled contract to output directory");
}

impl MetaConfig {
    pub fn build_wasm(&self) {
        if let Some(main_contract) = &self.main_contract {
            build_contract(main_contract, &self.build_args, self.output_dir.as_str());
        }

        if let Some(view_contract) = &self.view_contract {
            build_contract(view_contract, &self.build_args, self.output_dir.as_str());
        }
    }
}
