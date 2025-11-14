use std::process::Command;

pub const WASM_OPT_NAME: &str = "wasm-optx";
pub const WASM_OPT_VERSION_PREFIX: &str = "wasm-opt version ";

pub fn wasm_opt_version() -> Option<String> {
    let result = Command::new(WASM_OPT_NAME).args(["--version"]).output();

    match result {
        Ok(output) => {
            let output_string = String::from_utf8(output.stdout)
                .expect("could not parse wasm-opt version string, invalid utf-8");

            let version_string = output_string.trim().strip_prefix(WASM_OPT_VERSION_PREFIX)
                .expect("could not parse wasm-opt version string, expected prefix `{WASM_OPT_VERSION_PREFIX}`");

            Some(version_string.to_owned())
        }
        Err(_) => None,
    }
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
