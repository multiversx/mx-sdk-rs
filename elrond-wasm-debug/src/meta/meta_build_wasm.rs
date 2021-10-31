use std::{
    fs,
    io::{self, Write},
    process::Command,
};

use elrond_wasm::abi::ContractAbi;

pub fn build_wasm(abi: &ContractAbi) {
    let output = Command::new("cargo")
        .args(["build", "--target=wasm32-unknown-unknown", "--release"])
        .current_dir("../wasm")
        .env("RUSTFLAGS", "-C link-arg=-s")
        .output()
        .expect("failed to execute process");

    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();

    let source_wasm = format!(
        "../wasm/target/wasm32-unknown-unknown/release/{}_wasm.wasm",
        abi.get_module_name()
    );
    let dest_wasm = format!("../output/{}.wasm", abi.build_info.contract_crate.name);

    fs::copy(source_wasm, dest_wasm).unwrap();
}

pub fn clean_wasm() {
    let output = Command::new("cargo")
        .args(["clean"])
        .current_dir("../wasm")
        .output()
        .expect("failed to execute process");

    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();

    fs::remove_dir_all("../output").unwrap();
}
