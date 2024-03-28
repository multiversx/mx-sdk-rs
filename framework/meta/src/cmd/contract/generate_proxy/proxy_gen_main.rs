use std::fs::File;

use multiversx_sc::abi::ContractAbi;

use super::{
    super::meta_config::MetaConfig,
    proxy_crate_gen::create_file,
    proxy_gen_struct_enum::write_types,
    proxy_sc_functions_gen::write_content,
    proxy_template_gen::{
        write_header, write_impl_for_tx_proxy, write_struct_tx_proxy_methods,
        write_tx_proxy_type_def,
    },
};

const OUTPUT_PROXY_PATH: &str = "/output/proxy.rs";

impl MetaConfig {
    pub fn generate_proxy(&self) {
        write_proxy_with_explicit_path(OUTPUT_PROXY_PATH, &self.original_contract_abi);
        for path in &self.sc_config.proxy_paths {
            write_proxy_with_explicit_path(path, &self.original_contract_abi);
        }
    }
}

fn write_proxy_with_explicit_path(path: &str, abi: &ContractAbi) {
    let file = create_file(path);
    write_proxy_to_file(file, abi);
}

fn write_proxy_to_file(mut file: File, abi: &ContractAbi) {
    write_header(&mut file);
    write_tx_proxy_type_def(&mut file, &abi.name);
    write_impl_for_tx_proxy(&mut file, &abi.name);
    write_struct_tx_proxy_methods(&mut file, &abi.name);
    write_content(&mut file, abi.clone());
    write_types(
        &mut file,
        &abi.type_descriptions,
        abi.build_info.contract_crate.name,
    );
}
