use std::{
    fs::File,
    path::{Path, PathBuf},
};

use crate::cli::GenerateSnippetsArgs;

use super::{
    super::meta_config::MetaConfig,
    snippet_abi_check::{
        add_new_endpoints_to_file, check_abi_differences, create_prev_abi_file, ShortContractAbi,
    },
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
        let original_contract_abi = ShortContractAbi::from(self.original_contract_abi.clone());
        let diff_abi = check_abi_differences(
            &original_contract_abi,
            &self.snippets_dir.to_string_lossy().to_string(),
            args.overwrite,
        );
        if diff_abi == original_contract_abi {
            let mut file = create_snippets_crate_and_get_lib_file(
                &self.snippets_dir,
                crate_name,
                args.overwrite,
            );
            write_snippets_to_file(&mut file, &original_contract_abi, crate_name);
            let mut config_file = create_config_and_get_file(&self.snippets_dir);
            write_config_to_file(&mut config_file);
            let (mut interactor_test_file, mut chain_sim_test_file) =
                create_test_folder_and_get_files(&self.snippets_dir);
            write_tests_to_files(
                &mut interactor_test_file,
                &mut chain_sim_test_file,
                crate_name,
            );
        } else {
            add_new_endpoints_to_file(&self.snippets_dir.to_string_lossy().to_string(), &diff_abi);
        }

        // create prev-abi.json file
        create_prev_abi_file(
            &self.snippets_dir.to_string_lossy().to_string(),
            &self.original_contract_abi,
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
    create_sc_config_file(overwrite);
    create_main_file(snippets_folder_path, contract_crate_name);
    create_and_get_lib_file(snippets_folder_path, overwrite)
}

#[must_use]
fn create_config_and_get_file(snippets_folder_path: &Path) -> File {
    create_config_toml_file(snippets_folder_path);
    create_config_rust_file(snippets_folder_path)
}

fn write_snippets_to_file(file: &mut File, abi: &ShortContractAbi, crate_name: &str) {
    write_snippet_imports(file);
    write_snippet_constants(file);
    write_snippet_main_function(file, abi, crate_name);
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

fn write_tests_to_files(
    interactor_test_file: &mut File,
    chain_sim_test_file: &mut File,
    crate_name: &str,
) {
    write_interactor_test_to_file(interactor_test_file, crate_name);
    write_chain_sim_test_to_file(chain_sim_test_file, crate_name);
}
