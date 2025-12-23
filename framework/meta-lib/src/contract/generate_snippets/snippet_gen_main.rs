use std::{
    fs::File,
    path::{Path, PathBuf},
};

use multiversx_sc::abi::ContractAbi;

use crate::cli::GenerateSnippetsArgs;

use super::{
    super::meta_config::MetaConfig,
    snippet_crate_gen::{
        create_and_get_lib_file, create_config_rust_file, create_config_toml_file,
        create_main_file, create_sc_config_file, create_snippets_cargo_toml,
        create_snippets_folder, create_snippets_gitignore, create_src_folder,
        create_test_folder_and_get_files,
    },
    snippet_sc_functions_gen::write_interact_struct_impl,
    snippet_template_gen::{
        write_chain_sim_test_to_file, write_config_constants, write_config_imports,
        write_config_struct_declaration, write_config_struct_impl,
        write_interact_struct_declaration, write_interactor_test_to_file, write_snippet_constants,
        write_snippet_imports, write_snippet_main_function, write_snippet_state_impl,
        write_state_struct_declaration,
    },
};

impl MetaConfig {
    pub fn generate_rust_snippets(&self, args: &GenerateSnippetsArgs) {
        let crate_name = &self
            .original_contract_abi
            .build_info
            .contract_crate
            .name
            .replace("-", "_");
        let mut file =
            create_snippets_crate_and_get_lib_file(&self.snippets_dir, crate_name, args.overwrite);
        write_snippets_to_file(&mut file, &self.original_contract_abi, crate_name);
        let mut config_file = create_config_and_get_file(&self.snippets_dir);
        write_config_to_file(&mut config_file);
        let (mut interactor_test_file, mut chain_sim_test_file) =
            create_test_folder_and_get_files(&self.snippets_dir);
        write_tests_to_files(
            &mut interactor_test_file,
            &mut chain_sim_test_file,
            crate_name,
        );
    }
}

#[must_use]
fn create_snippets_crate_and_get_lib_file(
    snippets_folder_path: &PathBuf,
    contract_crate_name: &str,
    overwrite: bool,
) -> File {
    create_snippets_folder(snippets_folder_path);
    create_snippets_gitignore(snippets_folder_path, overwrite);
    create_snippets_cargo_toml(snippets_folder_path, contract_crate_name, overwrite);
    create_src_folder(snippets_folder_path);
    create_sc_config_file(overwrite, contract_crate_name);
    create_main_file(snippets_folder_path, contract_crate_name);
    create_and_get_lib_file(snippets_folder_path, overwrite)
}

#[must_use]
fn create_config_and_get_file(snippets_folder_path: &Path) -> File {
    create_config_toml_file(snippets_folder_path);
    create_config_rust_file(snippets_folder_path)
}

fn write_snippets_to_file(file: &mut File, abi: &ContractAbi, contract_crate_name: &str) {
    write_snippet_imports(file, contract_crate_name);
    write_snippet_constants(file);
    write_snippet_main_function(file, abi, contract_crate_name);
    write_state_struct_declaration(file);
    write_snippet_state_impl(file);
    write_interact_struct_declaration(file);
    write_interact_struct_impl(file, abi, contract_crate_name);
}

fn write_config_to_file(file: &mut File) {
    write_config_imports(file);
    write_config_constants(file);
    write_config_struct_declaration(file);
    write_config_struct_impl(file);
}

fn write_tests_to_files(
    interactor_test_file: &mut File,
    chain_sim_test_file: &mut File,
    crate_name: &str,
) {
    write_interactor_test_to_file(interactor_test_file, crate_name);
    write_chain_sim_test_to_file(chain_sim_test_file, crate_name);
}
