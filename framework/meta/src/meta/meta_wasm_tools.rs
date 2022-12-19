use std::process::Command;

use super::meta_config::MetaConfig;

const WASM_OPT_NAME: &str = "wasm-opt";
const WASM2WAT_NAME: &str = "wasm2wat";
const WASM_OBJDUMP_NAME: &str = "wasm-objdump";

impl MetaConfig {
    pub(crate) fn check_tools_installed(&mut self) {
        if self.build_args.wasm_opt && !is_wasm_opt_installed() {
            println!("Warning: {WASM_OPT_NAME} not installed");
            self.build_args.wasm_opt = false;
        }
        if self.build_args.wat && !is_wasm2wat_installed() {
            println!("Warning: {WASM2WAT_NAME} not installed");
            self.build_args.wat = false;
        }
        if self.build_args.extract_imports && !is_wasm_objdump_installed() {
            println!("Warning: {WASM_OBJDUMP_NAME} not installed");
            self.build_args.extract_imports = false;
        }
    }
}

fn is_wasm_opt_installed() -> bool {
    Command::new(WASM_OPT_NAME)
        .args(["--version"])
        .output()
        .is_ok()
}

fn is_wasm2wat_installed() -> bool {
    Command::new(WASM2WAT_NAME)
        .args(["--version"])
        .output()
        .is_ok()
}

fn is_wasm_objdump_installed() -> bool {
    Command::new(WASM_OBJDUMP_NAME)
        .args(["--version"])
        .output()
        .is_ok()
}

pub(crate) fn run_wasm_opt(output_wasm_path: &str) {
    let _ = Command::new(WASM_OPT_NAME)
        .args([output_wasm_path, "-Oz", "--output", output_wasm_path])
        .spawn()
        .expect("failed to spawn wasm-out process")
        .wait()
        .expect("wasm-out was not running");
}

pub(crate) fn run_wasm2wat(output_wasm_path: &str, output_wat_path: &str) {
    let _ = Command::new(WASM2WAT_NAME)
        .args([output_wasm_path, "--output", output_wat_path])
        .spawn()
        .expect("failed to spawn wasm2wat process")
        .wait()
        .expect("wasm2wat was not running");
}

pub(crate) fn run_wasm_objdump(output_wasm_path: &str) -> String {
    let output = Command::new(WASM_OBJDUMP_NAME)
        .args([output_wasm_path, "--details", "--section", "Import"])
        .output()
        .expect("failed to execute wasm-objdump");

    assert!(
        output.status.success(),
        "wasm-objdump exited with error: {}",
        core::str::from_utf8(&output.stderr).unwrap()
    );
    String::from_utf8(output.stdout).unwrap()
}

pub(crate) fn parse_imports(result: &str) -> Vec<String> {
    let mut import_names = Vec::new();
    for line in result.lines() {
        let split = line.split("<- env.").collect::<Vec<_>>();
        if split.len() == 2 {
            import_names.push(split[1].to_string());
        }
    }
    import_names
}
