use std::{
    fs::{self, File},
    io::Write,
};

use multiversx_sc::abi::ContractAbi;

use crate::{cli_args::GenerateRustTestSetupArgs, meta_config::MetaConfig};

use super::{
    test_base_struct_gen::{write_test_setup_imports, write_test_setup_struct_declaration},
    test_gen_common::{capitalize_first_letter, to_camel_case},
    test_setup_wrapper_functions_gen::write_struct_constructor,
};

pub(crate) fn create_test_folders(tests_folder_path: &str, contract_name: &str) {
    // returns error if folder already exists, so we ignore the result

    // create "tests" folder
    let _ = fs::create_dir(tests_folder_path);

    // create "name_setup" folder, which will contain the helper functions
    let setup_folder_path = format!("{tests_folder_path}/{contract_name}_setup/");
    let _ = fs::create_dir(setup_folder_path);
}

#[must_use]
pub(crate) fn create_and_get_test_setup_mod_file(setup_folder_path: &str, overwrite: bool) -> File {
    let file_path = format!("{setup_folder_path}mod.rs");
    println!("File path: {file_path}");
    if overwrite {
        return File::create(&file_path).unwrap();
    }

    match File::options()
        .create_new(true)
        .write(true)
        .open(&file_path)
    {
        Ok(f) => f,
        Err(_) => panic!("setup file already exists, --overwrite option was not provided"),
    }
}

impl MetaConfig {
    pub fn generate_rust_tests_setup(&self, args: &GenerateRustTestSetupArgs) {
        let main_contract = self.output_contracts.main_contract();
        let crate_name = &main_contract.contract_name;
        let snake_case_name = &main_contract.public_name_snake_case();
        let tests_folder = format!("../tests/");
        create_test_folders(&tests_folder, snake_case_name);

        let setup_folder_path = format!("{tests_folder}{snake_case_name}_setup/");
        let mut file = create_and_get_test_setup_mod_file(&setup_folder_path, args.overwrite);
        write_rust_tests_setup_to_file(
            &mut file,
            &self.original_contract_abi,
            crate_name,
            snake_case_name,
        );
    }
}

fn write_rust_tests_setup_to_file(
    file: &mut File,
    abi: &ContractAbi,
    contract_crate_name: &str,
    snake_case_name: &str,
) {
    let mut base_name = to_camel_case(contract_crate_name.to_string());
    capitalize_first_letter(&mut base_name);

    let struct_name = base_name.clone() + "Setup";
    let builder_func_name = base_name.clone() + "ObjBuilder";

    write_test_setup_imports(file, snake_case_name);
    write_test_setup_struct_declaration(file, snake_case_name, &struct_name, &builder_func_name);
    write_struct_constructor(
        file,
        contract_crate_name,
        &builder_func_name,
        &abi.constructors[0],
    );

    writeln!(file, "}}").unwrap();
}
