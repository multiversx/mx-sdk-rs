use std::fs::File;

use multiversx_sc::abi::ContractAbi;

use crate::cli_args::GenerateSnippetsAndProxiesArgs;

use super::{
    proxy_trait_crate_gen::{create_and_get_lib_file, create_proxies_cargo_toml, create_proxies_folder,
                            create_proxies_gitignore, create_src_folder},
    proxy_trait_sc_functions_gen::write_state_struct_impl,
    proxy_trait_template_gen::write_proxy_imports,
    super::meta_config::MetaConfig,
};

impl MetaConfig {
    pub fn generate_rust_proxies(&self, args: &GenerateSnippetsAndProxiesArgs) {
        let main_contract = self.output_contracts.main_contract();
        let crate_name = &main_contract.contract_name;
        let file = create_proxies_crate_and_get_lib_file(&self.proxy_trait_dir, crate_name, args.overwrite);
        write_proxies_to_file(
            file,
            &self.original_contract_abi,
        );
    }
}

#[must_use]
pub fn create_proxies_crate_and_get_lib_file(
    proxies_folder_path: &str,
    contract_crate_name: &str,
    overwrite: bool,
) -> File {
    create_proxies_folder(proxies_folder_path);
    create_proxies_gitignore(proxies_folder_path, overwrite);
    create_proxies_cargo_toml(proxies_folder_path, contract_crate_name, overwrite);
    create_src_folder(proxies_folder_path);
    create_and_get_lib_file(proxies_folder_path, overwrite)
}

fn write_proxies_to_file(
    mut file: File,
    abi: &ContractAbi,
) {
    write_proxy_imports(&mut file, abi.name);
    write_state_struct_impl(&mut file, abi);
}
