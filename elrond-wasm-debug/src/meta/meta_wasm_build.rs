use std::{fs, process::Command};

use super::meta_config::{BuildArgs, ContractMetadata, MetaConfig};

const WASM_OPT_NAME: &str = "wasm-opt";

impl MetaConfig {
    pub fn build_wasm(&mut self) {
        if self.build_args.wasm_opt && !is_wasm_opt_installed() {
            println!("Warning: {} not installed", WASM_OPT_NAME);
            self.build_args.wasm_opt = false;
        }

        if let Some(main_contract) = &self.main_contract {
            build_contract(main_contract, &self.build_args, self.output_dir.as_str());
        }

        if let Some(view_contract) = &self.view_contract {
            build_contract(view_contract, &self.build_args, self.output_dir.as_str());
        }
    }
}

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
    fs::copy(source_wasm_path.as_str(), dest_wasm_path.as_str())
        .expect("failed to copy compiled contract to output directory");

    optimize_contract(build_args, dest_wasm_path.as_str());
}

fn is_wasm_opt_installed() -> bool {
    Command::new(WASM_OPT_NAME)
        .args(["--version"])
        .output()
        .is_ok()
}

fn optimize_contract(build_args: &BuildArgs, wasm_path: &str) {
    if !build_args.wasm_opt {
        return;
    }

    let _ = Command::new(WASM_OPT_NAME)
        .args([wasm_path, "-O4", "--output", wasm_path])
        .spawn()
        .expect("failed to spawn wasm-out process")
        .wait()
        .expect("wasm-out was not running");
}
