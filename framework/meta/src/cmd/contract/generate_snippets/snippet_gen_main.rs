use std::fs::File;

use multiversx_sc::abi::ContractAbi;

use crate::cli_args::GenerateSnippetsArgs;

use super::{
    super::meta_config::MetaConfig,
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
    pub fn generate_rust_snippets(&self, args: &GenerateSnippetsArgs) {
        let main_contract = self.output_contracts.main_contract();
        let crate_name = &main_contract.contract_name;
        let snake_case_name = &main_contract.public_name_snake_case();
        let wasm_output_file_path_expr = format!("\"file:../output/{crate_name}.wasm\"");
        let file =
            create_snippets_crate_and_get_lib_file(&self.snippets_dir, crate_name, args.overwrite);
        write_snippets_to_file(
            file,
            &self.original_contract_abi,
            snake_case_name,
            &wasm_output_file_path_expr,
        );
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
    snake_case_name: &str,
    wasm_output_file_path_expr: &str,
) {
    write_snippet_imports(&mut file, snake_case_name);
    write_snippet_constants(&mut file);
    write_contract_type_alias(&mut file, snake_case_name);
    write_snippet_main_function(&mut file, abi);
    write_state_struct_declaration(&mut file);
    write_state_struct_impl(&mut file, abi, wasm_output_file_path_expr);
}
