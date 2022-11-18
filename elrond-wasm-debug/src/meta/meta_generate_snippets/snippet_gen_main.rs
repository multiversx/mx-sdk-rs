use std::fs::File;

use elrond_wasm::abi::ContractAbi;

use crate::meta::meta_config::MetaConfig;

use super::{
    snippet_crate_gen::{
        create_and_get_lib_file, create_snippets_cargo_toml, create_snippets_folder,
        create_snippets_gitignore, create_src_folder,
    },
    snippet_sc_functions_gen::write_state_struct_impl,
    snippet_template_gen::{
        write_contract_type_alias, write_snippet_constants, write_snippet_imports,
        write_snippet_main_function, write_state_struct_declaration,
    },
};

impl MetaConfig {
    // TODO: Handle overwrite flag
    pub fn generate_rust_snippets(&self, overwrite: bool) {
        if let Some(contract) = &self.main_contract {
            let crate_name = contract.output_base_name.clone().replace('-', "_");
            let wasm_output_file_path_expr = format!("\"file:../output/{}.wasm\"", &crate_name);
            let file =
                create_snippets_crate_and_get_lib_file(&self.snippets_dir, &crate_name, overwrite);
            write_snippets_to_file(
                file,
                &contract.original_abi,
                &crate_name,
                &wasm_output_file_path_expr,
            );
        }
    }
}

#[must_use]
fn create_snippets_crate_and_get_lib_file(
    snippets_folder_path: &str,
    contract_crate_name: &str,
    overwrite: bool,
) -> File {
    create_snippets_folder(snippets_folder_path);
    create_snippets_gitignore(snippets_folder_path, overwrite);
    create_snippets_cargo_toml(snippets_folder_path, contract_crate_name, overwrite);
    create_src_folder(snippets_folder_path);
    create_and_get_lib_file(snippets_folder_path, overwrite)
}

fn write_snippets_to_file(
    mut file: File,
    abi: &ContractAbi,
    contract_crate_name: &str,
    wasm_output_file_path_expr: &str,
) {
    write_snippet_imports(&mut file, contract_crate_name);
    write_snippet_constants(&mut file);
    write_contract_type_alias(&mut file, contract_crate_name);
    write_snippet_main_function(&mut file, abi);
    write_state_struct_declaration(&mut file);
    write_state_struct_impl(&mut file, abi, wasm_output_file_path_expr);
}
