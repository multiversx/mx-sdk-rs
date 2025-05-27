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

pub(super) fn process_rust_type(
    rust_type: String,
    paths: Vec<String>,
    processed_paths: Vec<String>,
) -> String {
    let mut processed_rust_type: String = rust_type.to_string().clone();
    for index in 0..paths.len() {
        processed_rust_type = processed_rust_type.replace(
            paths.get(index).unwrap(),
            processed_paths.get(index).unwrap(),
        );
    }

    processed_rust_type
}

pub(super) fn extract_paths(rust_type: &str) -> Vec<String> {
    let delimiters = "<>,()[] ";
    rust_type
        .split(|c| delimiters.contains(c))
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect()
}
