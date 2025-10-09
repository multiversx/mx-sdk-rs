use std::process::Command;

pub const WASM_OPT_NAME: &str = "wasm-opt";

pub fn is_wasm_opt_installed() -> bool {
    Command::new(WASM_OPT_NAME)
        .args(["--version"])
        .output()
        .is_ok()
}

pub fn install_wasm_opt() {
    let cmd = Command::new("cargo")
        .args(["install", "wasm-opt"])
        .status()
        .expect("failed to execute `cargo`");

    assert!(cmd.success(), "failed to install wasm-opt");

    println!("wasm-opt installed successfully");
}

pub fn run_wasm_opt(output_wasm_path: &str) {
    let exit_status = Command::new(WASM_OPT_NAME)
        .arg(output_wasm_path)
        .arg("-Oz")
        .arg("--enable-bulk-memory")
        .arg("--output")
        .arg(output_wasm_path)
        .spawn()
        .expect("failed to spawn wasm-opt process")
        .wait()
        .expect("wasm-opt was not running");

    assert!(exit_status.success(), "wasm-opt process failed");
}
