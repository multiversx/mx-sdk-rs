use std::fs::File;

use multiversx_sc::abi::ContractAbi;

use crate::cli::GenerateSnippetsArgs;

use super::{
    super::meta_config::MetaConfig,
    snippet_crate_gen::{
        create_and_get_lib_file, create_config_rust_file, create_config_toml_file,
        create_sc_config_file, create_snippets_cargo_toml, create_snippets_folder,
        create_snippets_gitignore, create_src_folder,
    },
    snippet_sc_functions_gen::write_interact_struct_impl,
    snippet_template_gen::{
        write_config_constants, write_config_imports, write_config_struct_declaration,
        write_config_struct_impl, write_interact_struct_declaration, write_snippet_constants,
        write_snippet_imports, write_snippet_main_function, write_snippet_state_impl,
        write_state_struct_declaration,
    },
};

impl MetaConfig {
    pub fn generate_rust_snippets(&self, args: &GenerateSnippetsArgs) {
        let main_contract = self.sc_config.main_contract();
        let crate_name = &main_contract.contract_name;
        let mut file =
            create_snippets_crate_and_get_lib_file(&self.snippets_dir, crate_name, args.overwrite);
        write_snippets_to_file(&mut file, &self.original_contract_abi, crate_name);
        let mut config_file = create_config_and_get_file(&self.snippets_dir);
        write_config_to_file(&mut config_file);
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
    create_sc_config_file(overwrite);
    create_and_get_lib_file(snippets_folder_path, overwrite)
}

#[must_use]
fn create_config_and_get_file(snippets_folder_path: &str) -> File {
    create_config_toml_file(snippets_folder_path);
    create_config_rust_file(snippets_folder_path)
}

fn write_snippets_to_file(file: &mut File, abi: &ContractAbi, crate_name: &str) {
    write_snippet_imports(file);
    write_snippet_constants(file);
    write_snippet_main_function(file, abi);
    write_state_struct_declaration(file);
    write_snippet_state_impl(file);
    write_interact_struct_declaration(file);
    write_interact_struct_impl(file, abi, crate_name);
}

fn write_config_to_file(file: &mut File) {
    write_config_imports(file);
    write_config_constants(file);
    write_config_struct_declaration(file);
    write_config_struct_impl(file);
}
