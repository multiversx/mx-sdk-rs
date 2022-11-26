use std::{fs, process::Command};

use crate::meta::meta_build_args::BuildArgs;

use super::OutputContract;

pub const WASM_OPT_NAME: &str = "wasm-opt";

impl OutputContract {
    pub fn build_contract(&mut self, build_args: &BuildArgs, output_path: &str) {
        let mut command = Command::new("cargo");
        command
            .args(["build", "--target=wasm32-unknown-unknown", "--release"])
            .current_dir(&self.wasm_crate_path());
        if let Some(target_dir) = &build_args.target_dir {
            command.args(["--target-dir", target_dir]);
        }
        if !build_args.debug_symbols {
            command.env("RUSTFLAGS", "-C link-arg=-s");
        }
        let exit_status = command
            .spawn()
            .expect("failed to spawn contract build process")
            .wait()
            .expect("contract build process was not running");

        assert!(exit_status.success(), "contract build process failed");

        self.copy_contracts_to_output(build_args, output_path);
    }

    fn copy_contracts_to_output(&mut self, build_args: &BuildArgs, output_path: &str) {
        let source_wasm_path = self.wasm_compilation_output_path(&build_args.target_dir);
        let dest_wasm_path = format!("{}/{}", output_path, self.wasm_output_name());
        fs::copy(source_wasm_path.as_str(), dest_wasm_path.as_str())
            .expect("failed to copy compiled contract to output directory");

        optimize_contract(build_args, dest_wasm_path.as_str());
    }
}

fn optimize_contract(build_args: &BuildArgs, wasm_path: &str) {
    if !build_args.wasm_opt {
        return;
    }

    let _ = Command::new(WASM_OPT_NAME)
        .args([wasm_path, "-Oz", "--output", wasm_path])
        .spawn()
        .expect("failed to spawn wasm-out process")
        .wait()
        .expect("wasm-out was not running");
}
