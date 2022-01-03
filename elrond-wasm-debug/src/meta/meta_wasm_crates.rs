use std::{
    fs::{self, File},
    io::Write,
};

use super::meta_config::{ContractMetadata, MetaConfig};

const WASM_LIB_PATH: &str = "../wasm/src/lib.rs";
const WASM_LIB_PATH_NO_MANAGED_EI: &str = "../wasm-no-managed-ei/src/lib.rs";

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
    writeln!(wasm_lib_file, "    {}", contract_module_name).unwrap();
    writeln!(wasm_lib_file, "    (").unwrap();
    for endpoint_name in endpoint_names {
        writeln!(wasm_lib_file, "        {}", endpoint_name).unwrap();
    }
    writeln!(wasm_lib_file, "    )").unwrap();
    writeln!(wasm_lib_file, "}}").unwrap();
}

fn write_wasm_empty_callback_macro(wasm_lib_file: &mut File) {
    writeln!(wasm_lib_file).unwrap();
    writeln!(wasm_lib_file, "elrond_wasm_node::wasm_empty_callback! {{}}").unwrap();
}

fn write_wasm_src_lib(contract_metadata: &ContractMetadata) {
    let contract_module_name = contract_metadata.abi.get_crate_name_for_code();
    let lib_path = format!("{}/src/lib.rs", &contract_metadata.wasm_crate_path);
    let mut wasm_lib_file = File::create(lib_path).unwrap();
    wasm_lib_file.write_all(PRELUDE.as_bytes()).unwrap();

    let mut endpoint_names: Vec<String> = contract_metadata
        .abi
        .endpoints
        .iter()
        .map(|endpoint| endpoint.name.to_string())
        .collect();
    endpoint_names.sort();

    let mut mandatory_endpoints = vec!["init".to_string()];
    if contract_metadata.abi.has_callback {
        mandatory_endpoints.push("callBack".to_string());
    }
    let all_endpoint_names = mandatory_endpoints.iter().chain(endpoint_names.iter());
    write_endpoints_macro(
        &mut wasm_lib_file,
        &contract_module_name,
        all_endpoint_names,
    );

    if !contract_metadata.abi.has_callback {
        write_wasm_empty_callback_macro(&mut wasm_lib_file);
    }
}

/// This one is useful for some of the special unmanaged EI tests in the framework.
/// Will do nothing for regular contracts.
pub fn copy_to_wasm_unmanaged_ei() {
    if std::path::Path::new(WASM_LIB_PATH_NO_MANAGED_EI).exists() {
        fs::copy(WASM_LIB_PATH, WASM_LIB_PATH_NO_MANAGED_EI).unwrap();
    }
}

impl MetaConfig {
    pub fn write_wasm_src_lib(&self) {
        if let Some(main_contract) = &self.main_contract {
            write_wasm_src_lib(main_contract);
        }

        if let Some(view_contract) = &self.view_contract {
            write_wasm_src_lib(view_contract);
        }
    }
}
