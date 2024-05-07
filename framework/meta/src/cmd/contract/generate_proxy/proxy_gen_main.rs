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
}

fn write_proxy_with_explicit_path(proxy_config: &ProxyConfigSerde, meta_config: &mut MetaConfig) {
    let mut file = create_file(&proxy_config.path);
    let mut proxy_generator = ProxyGenerator::new(meta_config, &mut file, proxy_config);
    proxy_generator.write_proxy_to_file();
}
