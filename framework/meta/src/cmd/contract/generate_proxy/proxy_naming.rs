pub(super) fn proxy_type_name(contract_trait_name: &str) -> String {
    format!("{contract_trait_name}Proxy")
}

pub(super) fn proxy_methods_type_name(contract_trait_name: &str) -> String {
    format!("{contract_trait_name}ProxyMethods")
}

pub(super) fn extract_struct_crate(struct_path: &str) -> String {
    let struct_crate_name = struct_path
        .replace('_', "-")
        .replace("multiversx_sc::api::uncallable::UncallableApi", "Api")
        .to_string();
    let crate_name = struct_crate_name
        .split("::")
        .next()
        .unwrap_or_else(|| &struct_crate_name);
    crate_name.to_string()
}
