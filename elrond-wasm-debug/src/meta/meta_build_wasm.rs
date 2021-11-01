use std::{
    fs,
    io::{self, Write},
    process::Command,
};

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
    let output = command.output().expect("failed to execute process");

    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();

    let source_wasm = format!(
        "../wasm/target/wasm32-unknown-unknown/release/{}_wasm.wasm",
        abi.get_module_name()
    );
    let wasm_name = build_args
        .wasm_name
        .unwrap_or_else(|| format!("{}.wasm", abi.build_info.contract_crate.name));
    let dest_wasm = format!("../output/{}", wasm_name);

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
