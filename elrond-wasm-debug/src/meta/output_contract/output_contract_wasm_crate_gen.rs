use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

use super::OutputContract;

const PRELUDE: &str = "////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![no_std]

";

fn write_endpoints_macro<'a, I>(
    full_macro_name: &str,
    wasm_lib_file: &mut File,
    contract_module_name: &str,
    endpoint_names: I,
) where
    I: Iterator<Item = &'a String>,
{
    writeln!(wasm_lib_file, "{} {{", full_macro_name).unwrap();
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

impl OutputContract {
    pub fn write_wasm_src_lib(&self) {
        fs::create_dir_all(PathBuf::from(&self.wasm_crate_path()).join("src")).unwrap();

        let contract_module_name = self.abi.get_crate_name_for_code();
        let lib_path = format!("{}/src/lib.rs", &self.wasm_crate_path());
        let mut wasm_lib_file = File::create(lib_path).unwrap();
        wasm_lib_file.write_all(PRELUDE.as_bytes()).unwrap();

        let mut endpoint_names: Vec<String> = self
            .abi
            .endpoints
            .iter()
            .map(|endpoint| endpoint.name.to_string())
            .collect();
        endpoint_names.sort();

        let full_macro_name = if self.external_view {
            "elrond_wasm_node::external_view_wasm_endpoints!"
        } else {
            "elrond_wasm_node::wasm_endpoints!"
        };

        let mut mandatory_endpoints = Vec::new();
        if self.abi.has_callback {
            mandatory_endpoints.push("callBack".to_string());
        }
        let all_endpoint_names = mandatory_endpoints.iter().chain(endpoint_names.iter());

        write_endpoints_macro(
            full_macro_name,
            &mut wasm_lib_file,
            &contract_module_name,
            all_endpoint_names,
        );

        if !self.abi.has_callback {
            write_wasm_empty_callback_macro(&mut wasm_lib_file);
        }
    }
}
