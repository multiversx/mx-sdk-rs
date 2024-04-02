pub(super) fn proxy_type_name(contract_trait_name: &str) -> String {
    format!("{contract_trait_name}Proxy")
}

pub(super) fn proxy_methods_type_name(contract_trait_name: &str) -> String {
    format!("{contract_trait_name}ProxyMethods")
}

pub(super) fn extract_struct_crate(struct_path: &str) -> String {
    let crate_name = struct_path.split("::").next().unwrap_or(struct_path);
    crate_name.to_string()
}
