use std::fs::File;

use multiversx_sc::abi::ContractAbi;

use crate::cli_args::GenerateOverwriteArg;

use super::{
    super::meta_config::MetaConfig,
    proxy_crate_gen::create_file,
    proxy_sc_functions_gen::write_content,
    proxy_template_gen::{write_imports, write_struct_template},
};

const PROXIES_SOURCE_FILE_NAME: &str = "proxies_struct_interactor_main.rs";

impl MetaConfig {
    pub fn generate_rust_proxies_struct(&self, args: &GenerateOverwriteArg) {
        let file = create_file(PROXIES_SOURCE_FILE_NAME, args.overwrite);
        write_proxies_to_file(file, self.original_contract_abi.clone());
    }
}

fn write_proxies_to_file(mut file: File, abi: ContractAbi) {
    write_imports(&mut file);
    write_struct_template(&mut file);
    write_content(&mut file, abi);
}
