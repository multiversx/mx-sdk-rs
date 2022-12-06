use std::{fs, process::Command};

use crate::meta::meta_build_args::BuildArgs;

use super::OutputContract;

pub const WASM_OPT_NAME: &str = "wasm-opt";
pub const WASM2WAT_NAME: &str = "wasm2wat";

impl OutputContract {
    pub fn build_contract(&self, build_args: &BuildArgs, output_path: &str) {
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

        self.finalize_build(build_args, output_path);
    }

    fn finalize_build(&self, build_args: &BuildArgs, output_path: &str) {
        self.copy_contracts_to_output(build_args, output_path);
        self.run_wasm_opt(build_args, output_path);
        self.run_wasm2wat(build_args, output_path);
    }

    fn copy_contracts_to_output(&self, build_args: &BuildArgs, output_path: &str) {
        let source_wasm_path = self.wasm_compilation_output_path(&build_args.target_dir);
        let output_wasm_path = format!("{}/{}", output_path, self.wasm_output_name(build_args));
        fs::copy(source_wasm_path, output_wasm_path)
            .expect("failed to copy compiled contract to output directory");
    }

    fn run_wasm_opt(&self, build_args: &BuildArgs, output_path: &str) {
        if !build_args.wasm_opt {
            return;
        }

        let output_wasm_path = format!("{}/{}", output_path, self.wasm_output_name(build_args));
        let _ = Command::new(WASM_OPT_NAME)
            .args([
                output_wasm_path.as_str(),
                "-Oz",
                "--output",
                output_wasm_path.as_str(),
            ])
            .spawn()
            .expect("failed to spawn wasm-out process")
            .wait()
            .expect("wasm-out was not running");
    }

    fn run_wasm2wat(&self, build_args: &BuildArgs, output_path: &str) {
        if !build_args.wat {
            return;
        }

        let output_wasm_path = format!("{}/{}", output_path, self.wasm_output_name(build_args));
        let output_wat_path = format!("{}/{}", output_path, self.wat_output_name(build_args));
        let _ = Command::new(WASM2WAT_NAME)
            .args([
                output_wasm_path.as_str(),
                "--output",
                output_wat_path.as_str(),
            ])
            .spawn()
            .expect("failed to spawn wasm2wat process")
            .wait()
            .expect("wasm2wat was not running");
    }
}
