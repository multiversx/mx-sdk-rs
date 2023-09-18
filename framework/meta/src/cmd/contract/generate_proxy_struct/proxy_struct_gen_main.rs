use std::fs::File;

use multiversx_sc::abi::ContractAbi;

use crate::cmd::contract::generate_proxy_trait::proxy_trait_crate_gen::create_and_get_lib_file;

use super::{
    proxy_struct_sc_functions_gen::write_content,
    proxy_struct_template_gen::{write_imports, write_struct_template},
    super::meta_config::MetaConfig,
};

static PROXIES_SOURCE_FILE_NAME: &str = "proxies_struct_interactor_main.rs";

impl MetaConfig {
    pub fn generate_rust_proxies_struct(&self) {
        let file = create_and_get_lib_file(PROXIES_SOURCE_FILE_NAME);
        write_proxies_struct_to_file(
            file,
            &self.original_contract_abi,
        );
    }
}

fn write_proxies_struct_to_file(
    mut file: File,
    abi: &ContractAbi,
) {
    write_imports(&mut file);
    write_struct_template(&mut file);
    write_content(&mut file, abi);
}
