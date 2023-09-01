use std::{path::Path, process::Command};

use colored::Colorize;

pub fn print_all_count(num_contract_crates: usize) {
    println!(
        "\n{}",
        format!("Found {num_contract_crates} contract crates.").truecolor(128, 128, 128),
    );
}

pub fn print_all_index(contract_crates_index: usize, num_contract_crates: usize) {
    println!(
        "\n{}",
        format!("({contract_crates_index}/{num_contract_crates})").truecolor(128, 128, 128),
    );
}

pub fn print_all_command(meta_path: &Path, cargo_run_args: &[String]) {
    println!(
        "{} {}\n{} `cargo {}`",
        "In".green(),
        meta_path.display(),
        "Calling".green(),
        cargo_run_args.join(" "),
    );
}

pub fn format_command(command: &Command) -> String {
    let mut result = String::new();
    for (key, opt_value) in command.get_envs() {
        if let Some(value) = opt_value {
            result +=
                format!("{}=\"{}\" ", key.to_string_lossy(), value.to_string_lossy()).as_str();
        }
    }
    result.push_str(command.get_program().to_string_lossy().as_ref());

    for arg in command.get_args() {
        result.push(' ');
        result.push_str(arg.to_string_lossy().as_ref());
    }

    result
}

pub fn print_build_command(contract_name: String, command: &Command) {
    let path = command
        .get_current_dir()
        .expect("missing command dir")
        .canonicalize()
        .expect("command dir canonicalization failed");
    println!(
        "{}\n{}",
        format!("Building {} in {} ...", contract_name, path.display()).green(),
        format_command(command).green(),
    );
}

pub fn print_copy_contract(source_wasm_path: &str, output_wasm_path: &str) {
    println!(
        "{}",
        format!("Copying {source_wasm_path} to {output_wasm_path} ...").green(),
    );
}

pub fn print_call_wasm_opt(wasm_path: &str) {
    println!("{}", format!("Calling wasm-opt on {wasm_path} ...").green(),);
}

pub fn print_call_wasm2wat(wasm_path: &str, wat_path: &str) {
    println!(
        "{}",
        format!("Calling wasm2wat on {wasm_path} -> {wat_path} ...").green(),
    );
}

pub fn print_pack_mxsc_file(output_mxsc_path: &str) {
    println!("{}", format!("Packing {output_mxsc_path} ...").green(),);
}

pub fn print_contract_size(size: usize) {
    println!("{}", format!("Contract size: {size} bytes.").blue(),);
}

pub fn print_extract_imports(imports_path: &str) {
    println!(
        "{}",
        format!("Extracting imports to {imports_path} ...").green(),
    );
}

pub fn print_check_ei(ei_version: &str) {
    print!(
        "{}",
        format!("Checking EI version: {ei_version} ...").green(),
    );
}

pub fn print_invalid_vm_hook(import_name: &str, ei_version: &str) {
    print!(
        "\n{}",
        format!(
            "WARNING! Import '{import_name}' is not available on EI version {ei_version}! This will become a hard error in the next release."
        ).yellow(),
    );
}

pub fn print_check_ei_ok() {
    println!("{}", " OK".green(),);
}

pub fn print_ignore_ei_check() {
    println!("{}", "EI version check explicitly ignored".yellow(),);
}
