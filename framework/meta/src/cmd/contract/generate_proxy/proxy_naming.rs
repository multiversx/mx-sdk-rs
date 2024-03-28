pub(super) fn proxy_type_name(contract_trait_name: &str) -> String {
    format!("{contract_trait_name}Proxy")
}

pub(super) fn proxy_methods_type_name(contract_trait_name: &str) -> String {
    format!("{contract_trait_name}ProxyMethods")
}
