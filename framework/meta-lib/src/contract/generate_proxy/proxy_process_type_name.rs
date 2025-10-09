use multiversx_sc::abi::EnumVariantDescription;

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

pub(super) fn explicit_discriminant(
    index: usize,
    variant: &EnumVariantDescription,
    variants: &[EnumVariantDescription],
) -> Option<usize> {
    if index == variant.discriminant {
        return None;
    }

    if index == 0 && variant.discriminant != 0 {
        return Some(variant.discriminant);
    }

    if variants[index - 1].discriminant != variant.discriminant - 1 {
        return Some(variant.discriminant);
    }

    None
}

pub(super) fn c_enum_representation(enum_variants: &[EnumVariantDescription]) -> Option<String> {
    enum_variants
        .iter()
        .enumerate()
        .find_map(|(i, variant)| explicit_discriminant(i, variant, enum_variants))?;

    let max_discriminant = max_discriminant(enum_variants)?;

    let ty = if max_discriminant <= u8::MAX as usize {
        "u8"
    } else if max_discriminant <= u16::MAX as usize {
        "u16"
    } else if max_discriminant <= u32::MAX as usize {
        "u32"
    } else if max_discriminant <= u64::MAX as usize {
        "u64"
    } else {
        "u128"
    };

    Some(ty.to_string())
}

fn max_discriminant(enum_variants: &[EnumVariantDescription]) -> Option<usize> {
    enum_variants
        .iter()
        .filter(|variant| !variant.fields.is_empty())
        .map(|variant| variant.discriminant)
        .max()
}
