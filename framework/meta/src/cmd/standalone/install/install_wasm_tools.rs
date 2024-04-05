use std::process::Command;

pub fn install_wasm32_target() {
    let cmd = Command::new("rustup")
        .args(vec!["target", "add", "wasm32-unknown-unknown"])
        .status()
        .expect("failed to execute `rustup`");

    assert!(cmd.success(), "failed to install wasm32 target");

    println!("wasm32 target installed successfully");
}

pub fn install_wasm_opt() {
    let cmd = Command::new("cargo")
        .args(vec!["install", "wasm-opt"])
        .status()
        .expect("failed to execute `cargo`");

    assert!(cmd.success(), "failed to install wasm-opt");

    println!("wasm-opt installed successfully");
}
