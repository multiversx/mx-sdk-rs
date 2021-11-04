use std::{fs, process::Command};

use elrond_wasm::abi::ContractAbi;

#[derive(Default, Debug)]
struct BuildArgs {
    debug_symbols: bool,
    wasm_name: Option<String>,
}

fn process_args(args: &[String]) -> BuildArgs {
    let mut result = BuildArgs::default();
    let mut iter = args.iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--wasm-symbols" => {
                result.debug_symbols = true;
            },
            "--wasm-name" => {
                let name = iter
                    .next()
                    .expect("argument `--wasm-name` must be followed by the desired name");
                result.wasm_name = Some(name.clone());
            },
            _ => {},
        }
    }
    result
}

pub fn build_wasm(abi: &ContractAbi, args: &[String]) {
    let build_args = process_args(args);
    let mut command = Command::new("cargo");
    command
        .args(["build", "--target=wasm32-unknown-unknown", "--release"])
        .current_dir("../wasm");
    if !build_args.debug_symbols {
        command.env("RUSTFLAGS", "-C link-arg=-s");
    }
    let exit_status = command
        .spawn()
        .expect("failed to spawn contract build process")
        .wait()
        .expect("contract build process was not running");

    assert!(exit_status.success(), "contract build process failed");

    let source_wasm = format!(
        "../wasm/target/wasm32-unknown-unknown/release/{}_wasm.wasm",
        abi.get_module_name()
    );
    let wasm_name = build_args
        .wasm_name
        .unwrap_or_else(|| format!("{}.wasm", abi.build_info.contract_crate.name));
    let dest_wasm = format!("../output/{}", wasm_name);

    fs::copy(source_wasm, dest_wasm).expect("failed to copy compiled contract to output directory");
}

pub fn clean_wasm() {
    let exit_status = Command::new("cargo")
        .args(["clean"])
        .current_dir("../wasm")
        .spawn()
        .expect("failed to spawn contract clean process")
        .wait()
        .expect("contract clean process was not running");

    assert!(exit_status.success(), "contract clean process failed");

    fs::remove_dir_all("../output").expect("failed to remove output directory");
}
