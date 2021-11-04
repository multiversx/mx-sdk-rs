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

";

fn write_endpoints_macro<'a, I>(
    wasm_lib_file: &mut File,
    contract_module_name: &str,
    endpoint_names: I,
) where
    I: Iterator<Item = &'a String>,
{
    writeln!(wasm_lib_file, "elrond_wasm_node::wasm_endpoints! {{").unwrap();
    writeln!(wasm_lib_file, "   {}", contract_module_name).unwrap();
    writeln!(wasm_lib_file, "   (").unwrap();
    for endpoint_name in endpoint_names {
        writeln!(wasm_lib_file, "        {}", endpoint_name).unwrap();
    }
    writeln!(wasm_lib_file, "   )").unwrap();
    writeln!(wasm_lib_file, "}}").unwrap();
}

pub fn write_wasm_lib(abi: &ContractAbi) {
    let contract_module_name = abi.get_module_name();
    create_dir_all(WASM_SRC_DIR).unwrap();
    let mut wasm_lib_file = File::create(WASM_SRC_PATH).unwrap();
    wasm_lib_file.write_all(PRELUDE.as_bytes()).unwrap();

    let mut endpoint_names: Vec<String> = abi
        .endpoints
        .iter()
        .map(|endpoint| endpoint.name.to_string())
        .collect();
    endpoint_names.sort();

    let mandatory_endpoints = ["init".to_string(), "callBack".to_string()];
    let all_endpoint_names = mandatory_endpoints.iter().chain(endpoint_names.iter());
    write_endpoints_macro(
        &mut wasm_lib_file,
        &contract_module_name,
        all_endpoint_names,
    );
}

/// This one is useful for some of the special unmanaged EI tests in the framework.
/// Will do nothing for regular contracts.
pub fn copy_to_wasm_unmanaged_ei() {
    if std::path::Path::new(WASM_SRC_PATH_NO_MANAGED_EI).exists() {
        fs::copy(WASM_SRC_PATH, WASM_SRC_PATH_NO_MANAGED_EI).unwrap();
    }
}
