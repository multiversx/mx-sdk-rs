use std::{fs, process::Command};

use super::OutputContract;
use crate::{
    cli_args::BuildArgs, meta_wasm_tools, output_contract::print_util::print_build_command,
};

impl OutputContract {
    pub fn build_contract(&self, build_args: &BuildArgs, output_path: &str) {
        let mut command = self.compose_build_command(build_args);

        print_build_command(self.wasm_output_name(build_args), &command);

        let exit_status = command
            .spawn()
            .expect("failed to spawn contract build process")
            .wait()
            .expect("contract build process was not running");

        assert!(exit_status.success(), "contract build process failed");

        self.finalize_build(build_args, output_path);
    }

    fn compose_build_command(&self, build_args: &BuildArgs) -> Command {
        let mut command = Command::new("cargo");
        command
            .args(["build", "--target=wasm32-unknown-unknown", "--release"])
            .current_dir(self.wasm_crate_path());
        if build_args.locked {
            command.arg("--locked");
        }
        if let Some(target_dir) = &build_args.target_dir {
            command.args(["--target-dir", target_dir]);
        }
        let rustflags = compose_rustflags(build_args);
        if !rustflags.is_empty() {
            command.env("RUSTFLAGS", rustflags);
        }
        command
    }
}

fn compose_rustflags(build_args: &BuildArgs) -> String {
    let mut rustflags = String::new();
    if !build_args.wasm_symbols {
        rustflags.push_str("-C link-arg=-s");
    }
    if build_args.emit_mir {
        if !rustflags.is_empty() {
            rustflags.push(' ');
        }
        rustflags.push_str("--emit=mir");
    }
    rustflags
}

impl OutputContract {
    fn finalize_build(&self, build_args: &BuildArgs, output_path: &str) {
        self.copy_contracts_to_output(build_args, output_path);
        self.run_wasm_opt(build_args, output_path);
        self.run_wasm2wat(build_args, output_path);
        self.extract_imports(build_args, output_path);
        self.run_twiggy(build_args, output_path);
    }

    fn copy_contracts_to_output(&self, build_args: &BuildArgs, output_path: &str) {
        let source_wasm_path = self.wasm_compilation_output_path(&build_args.target_dir);
        let output_wasm_path = format!("{output_path}/{}", self.wasm_output_name(build_args));
        fs::copy(source_wasm_path, output_wasm_path)
            .expect("failed to copy compiled contract to output directory");
    }

    fn run_wasm_opt(&self, build_args: &BuildArgs, output_path: &str) {
        if !build_args.wasm_opt {
            return;
        }

        let output_wasm_path = format!("{output_path}/{}", self.wasm_output_name(build_args));
        meta_wasm_tools::run_wasm_opt(output_wasm_path.as_str());
    }

    fn run_wasm2wat(&self, build_args: &BuildArgs, output_path: &str) {
        if !build_args.wat {
            return;
        }

        let output_wasm_path = format!("{output_path}/{}", self.wasm_output_name(build_args));
        let output_wat_path = format!("{output_path}/{}", self.wat_output_name(build_args));
        meta_wasm_tools::run_wasm2wat(output_wasm_path.as_str(), output_wat_path.as_str());
    }

    fn extract_imports(&self, build_args: &BuildArgs, output_path: &str) {
        if !build_args.extract_imports {
            return;
        }

        let output_wasm_path = format!("{output_path}/{}", self.wasm_output_name(build_args));
        let output_imports_json_path = format!(
            "{}/{}",
            output_path,
            self.imports_json_output_name(build_args)
        );
        let result = meta_wasm_tools::run_wasm_objdump(output_wasm_path.as_str());
        let import_names = meta_wasm_tools::parse_imports(result.as_str());
        write_imports_output(output_imports_json_path.as_str(), import_names.as_slice());
    }
}

fn write_imports_output(dest_path: &str, import_names: &[String]) {
    let json = serde_json::to_string_pretty(import_names).unwrap();
    fs::write(dest_path, json).expect("failed to write imports json file");
}

impl OutputContract {
    fn run_twiggy(&self, build_args: &BuildArgs, output_path: &str) {
        if build_args.has_twiggy_call() {
            let output_wasm_path = format!("{output_path}/{}", self.wasm_output_name(build_args));

            if build_args.twiggy_top {
                let output_twiggy_top_path =
                    format!("{output_path}/{}", self.twiggy_top_name(build_args));
                meta_wasm_tools::run_twiggy_top(
                    output_wasm_path.as_str(),
                    output_twiggy_top_path.as_str(),
                );
            }
            if build_args.twiggy_paths {
                let output_twiggy_paths_path =
                    format!("{output_path}/{}", self.twiggy_paths_name(build_args));
                meta_wasm_tools::run_twiggy_paths(
                    output_wasm_path.as_str(),
                    output_twiggy_paths_path.as_str(),
                );
            }
            if build_args.twiggy_monos {
                let output_twiggy_monos_path =
                    format!("{output_path}/{}", self.twiggy_monos_name(build_args));
                meta_wasm_tools::run_twiggy_monos(
                    output_wasm_path.as_str(),
                    output_twiggy_monos_path.as_str(),
                );
            }
            if build_args.twiggy_dominators {
                let output_twiggy_dominators_path =
                    format!("{output_path}/{}", self.twiggy_dominators_name(build_args));
                meta_wasm_tools::run_twiggy_dominators(
                    output_wasm_path.as_str(),
                    output_twiggy_dominators_path.as_str(),
                );
            }
        }
    }
}
