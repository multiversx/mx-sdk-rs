use std::fs::File;

use multiversx_sc::abi::ContractAbi;

use crate::cli_args::GenerateSnippetsAndProxiesArgs;
use crate::cmd::contract::generate_proxy_trait::proxy_trait_gen_main::create_proxies_crate_and_get_lib_file;

use super::{
    proxy_struct_sc_functions_gen::write_content,
    proxy_struct_template_gen::write_struct_template,
    super::meta_config::MetaConfig,
};

impl MetaConfig {
    pub fn generate_rust_proxies_struct(&self, args: &GenerateSnippetsAndProxiesArgs) {
        let main_contract = self.output_contracts.main_contract();
        let crate_name = &main_contract.contract_name;
        let file = create_proxies_crate_and_get_lib_file(&self.proxy_struct_dir, crate_name, args.overwrite);
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
    write_struct_template(&mut file);
    write_content(&mut file, abi);
}
