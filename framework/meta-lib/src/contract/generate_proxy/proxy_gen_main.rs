use colored::Colorize;
use std::{fs, path::Path};

use crate::contract::sc_config::proxy_config::ProxyConfig;

use super::{
    super::meta_config::MetaConfig, proxy_crate_gen::create_file, proxy_generator::ProxyGenerator,
};

const PROXY_COMPARE_ERR_MSG: &str = "Contract has been modified and proxies have not been updated. Regenerate proxies to avoid inconsistencies.";

impl MetaConfig {
    pub fn generate_proxy(&mut self) {
        if self.sc_config.proxy_configs.is_empty() {
            let proxy_config_default =
                ProxyConfig::output_dir_proxy_config(self.original_contract_abi.clone());
            write_proxy_with_explicit_path(&proxy_config_default, self);
            return;
        }

        for proxy_config in &self.sc_config.proxy_configs {
            write_proxy_with_explicit_path(proxy_config, self);
        }
    }

    pub fn compare_proxy(&mut self) {
        for proxy_config in &self.sc_config.proxy_configs {
            compare_proxy_explicit_path(proxy_config, self);
        }
    }
}

fn compare_proxy_explicit_path(proxy_config: &ProxyConfig, meta_config: &MetaConfig) {
    let mut temp = Vec::<u8>::new();
    let mut proxy_generator = ProxyGenerator::new(meta_config, &mut temp, proxy_config);
    proxy_generator.write_proxy_to_file();

    let existent_proxy_path = Path::new("..").join(&proxy_config.path);
    let existent_proxy = fs::read_to_string(existent_proxy_path);

    if let Ok(existent_proxy) = existent_proxy {
        let newly_gen_proxy = String::from_utf8(temp).unwrap();

        if existent_proxy != newly_gen_proxy {
            panic!("{}", PROXY_COMPARE_ERR_MSG.to_string().red());
        }
    }
}

fn write_proxy_with_explicit_path(proxy_config: &ProxyConfig, meta_config: &MetaConfig) {
    let mut file = create_file(&proxy_config.path);
    let mut proxy_generator = ProxyGenerator::new(meta_config, &mut file, proxy_config);
    proxy_generator.write_proxy_to_file();
}
