use std::process::Command;

use multiversx_sc_meta_lib::tools;

pub fn install_wasm32_target() {
    install_target(tools::build_target::WASM32_TARGET);
    if tools::build_target::is_wasm32v1_available() {
        install_target(tools::build_target::WASM32V1_TARGET);
    }
}

fn install_target(target_name: &str) {
    let cmd = Command::new("rustup")
        .args(["target", "add", target_name])
        .status()
        .expect("failed to execute `rustup`");

    assert!(cmd.success(), "failed to install {target_name} target");

    println!("{target_name} target installed successfully");
}

pub fn install_wasm_opt() {
    let cmd = Command::new("cargo")
        .args(["install", "wasm-opt"])
        .status()
        .expect("failed to execute `cargo`");

    assert!(cmd.success(), "failed to install wasm-opt");

    println!("wasm-opt installed successfully");
}
