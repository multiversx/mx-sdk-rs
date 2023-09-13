use std::fs::File;

use multiversx_sc::abi::ContractAbi;

use crate::cli_args::GenerateSnippetsAndProxiesArgs;

use super::{
    proxy_trait_crate_gen::create_and_get_lib_file,
    proxy_trait_sc_functions_gen::write_state_struct_impl,
    proxy_trait_template_gen::write_proxy_imports,
    super::meta_config::MetaConfig,
};

static PROXIES_SOURCE_FILE_NAME: &str = "proxies_trait_interactor_main.rs";

impl MetaConfig {
    pub fn generate_rust_proxies(&self, args: &GenerateSnippetsAndProxiesArgs) {
        let file = create_and_get_lib_file(PROXIES_SOURCE_FILE_NAME, args.overwrite);
        write_proxies_to_file(
            file,
            &self.original_contract_abi,
        );
    }
}

fn write_proxies_to_file(
    mut file: File,
    abi: &ContractAbi,
) {
    write_proxy_imports(&mut file, abi.name);
    write_state_struct_impl(&mut file, abi);
}
