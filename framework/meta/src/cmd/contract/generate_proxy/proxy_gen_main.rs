use colored::Colorize;
use std::fs;

use multiversx_sc::abi::ContractAbi;

use crate::cmd::contract::sc_config::ProxyConfigSerde;

use super::{
    super::meta_config::MetaConfig, proxy_crate_gen::create_file, proxy_generator::ProxyGenerator,
};

impl MetaConfig {
    pub fn generate_proxy(&mut self) {
        let default_proxy = ProxyConfigSerde::new();
        write_proxy_with_explicit_path(&default_proxy, self);
        for proxy_config in self.sc_config.proxy_configs.clone() {
            write_proxy_with_explicit_path(&proxy_config, self);
        }
    }

    pub fn compare_proxy(&mut self) {
        for proxy_config in self.sc_config.proxy_configs.clone() {
            compare_proxy_explicit_path(&proxy_config, self);
        }
    }
}

fn compare_proxy_explicit_path(proxy_config: &ProxyConfigSerde, meta_config: &MetaConfig) {
    let contract_abi = extract_contract_abi(proxy_config, meta_config);
    let mut temp = create_file("temp");
    let mut proxy_generator =
        ProxyGenerator::new(meta_config, &mut temp, proxy_config, contract_abi);
    proxy_generator.write_proxy_to_file();

    let existent_proxy_path = format!("../{}", proxy_config.path);
    let temp_path = "../temp";

    let existent_proxy = fs::read_to_string(existent_proxy_path).unwrap();
    let temp = fs::read_to_string(temp_path).unwrap();

    if existent_proxy != temp {
        fs::remove_file(temp_path).unwrap();
        panic!("{}", format!("Contract has been modified and proxies have not been updated. Regenerate proxies to avoid inconsistencies.").red());
    }

    fs::remove_file(temp_path).unwrap();
}

fn write_proxy_with_explicit_path(proxy_config: &ProxyConfigSerde, meta_config: &MetaConfig) {
    let contract_abi = extract_contract_abi(proxy_config, meta_config);
    let mut file = create_file(&proxy_config.path);
    let mut proxy_generator =
        ProxyGenerator::new(meta_config, &mut file, proxy_config, contract_abi);
    proxy_generator.write_proxy_to_file();
}

fn extract_contract_abi<'a>(
    proxy_config: &'a ProxyConfigSerde,
    meta_config: &'a MetaConfig,
) -> &'a ContractAbi {
    if proxy_config.variant.is_some() {
        let variant = proxy_config.variant.as_ref().unwrap();
        for contract_variant in &meta_config.sc_config.contracts {
            if variant == &contract_variant.public_name_snake_case() {
                return &contract_variant.abi;
            }
        }

        panic!("No variant with name \"{}\" in multicontract", variant);
    }

    &meta_config.original_contract_abi
}
