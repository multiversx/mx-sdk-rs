use std::{
    fs::{self, create_dir_all, File},
    io::Write,
};

use elrond_wasm::abi::ContractAbi;

const WASM_SRC_DIR: &str = "../wasm/src";
const WASM_SRC_PATH: &str = "../wasm/src/lib.rs";
const WASM_SRC_PATH_NO_MANAGED_EI: &str = "../wasm-no-managed-ei/src/lib.rs";

const PRELUDE: &str = "////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![no_std]
#![allow(non_snake_case)]

pub use elrond_wasm_output;
";

fn write_endpoint(wasm_lib_file: &mut File, contract_module_name: &str, endpoint_name: &str) {
    writeln!(
        wasm_lib_file,
        "
#[no_mangle]
pub fn {}() {{
    {}::endpoints::{}(elrond_wasm_node::arwen_api());
}}",
        endpoint_name, contract_module_name, endpoint_name
    )
    .unwrap();
}

pub fn write_wasm_lib(abi: &ContractAbi) {
    let contract_module_name = abi
        .build_info
        .contract_crate
        .name
        .replace('-', "_")
        .to_lowercase();
    create_dir_all(WASM_SRC_DIR).unwrap();
    let mut wasm_lib_file = File::create(WASM_SRC_PATH).unwrap();
    wasm_lib_file.write_all(PRELUDE.as_bytes()).unwrap();

    write_endpoint(&mut wasm_lib_file, &contract_module_name, "init");

    write_endpoint(&mut wasm_lib_file, &contract_module_name, "callBack");

    let mut endpoint_names: Vec<String> = abi
        .endpoints
        .iter()
        .map(|endpoint| endpoint.name.to_string())
        .collect();
    endpoint_names.sort();

    for endpoint_name in &endpoint_names {
        write_endpoint(&mut wasm_lib_file, &contract_module_name, endpoint_name);
    }
}

/// This one is useful for some of the special unmanaged EI tests in the framework.
/// Will do nothing for regular contracts.
pub fn copy_to_wasm_unmanaged_ei() {
    if std::path::Path::new(WASM_SRC_PATH_NO_MANAGED_EI).exists() {
        fs::copy(WASM_SRC_PATH, WASM_SRC_PATH_NO_MANAGED_EI).unwrap();
    }
}
