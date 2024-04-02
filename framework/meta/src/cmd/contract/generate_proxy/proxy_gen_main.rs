use super::{
    super::meta_config::MetaConfig, proxy_crate_gen::create_file, proxy_generator::ProxyGenerator,
};

const OUTPUT_PROXY_PATH: &str = "/output/proxy.rs";

impl MetaConfig {
    pub fn generate_proxy(&mut self) {
        write_proxy_with_explicit_path(OUTPUT_PROXY_PATH, self);
        let proxy_paths = self.sc_config.proxy_paths.clone();
        for path in proxy_paths {
            write_proxy_with_explicit_path(&path, self);
        }
    }
}

fn write_proxy_with_explicit_path(path: &str, meta_config: &mut MetaConfig) {
    let mut file = create_file(path);
    let mut proxy_generator = ProxyGenerator::new(meta_config, &mut file);
    proxy_generator.write_proxy_to_file();
}
