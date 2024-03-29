use super::{
    super::meta_config::MetaConfig, proxy_crate_gen::create_file, proxy_generator::ProxyGenerator,
};

const OUTPUT_PROXY_PATH: &str = "/output/proxy.rs";

impl MetaConfig {
    pub fn generate_proxy(&self) {
        write_proxy_with_explicit_path(OUTPUT_PROXY_PATH, self);
        for path in &self.sc_config.proxy_paths {
            write_proxy_with_explicit_path(path, self);
        }
    }
}

fn write_proxy_with_explicit_path(path: &str, meta_config: &MetaConfig) {
    let file = create_file(path);
    let proxy_generator = ProxyGenerator::new(meta_config);
    proxy_generator.write_proxy_to_file(file);
}
