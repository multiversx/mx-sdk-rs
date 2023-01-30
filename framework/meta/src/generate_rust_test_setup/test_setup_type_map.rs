use std::collections::HashMap;

use crate::generate_snippets::snippet_type_map::{handle_abi_type, RustTypeString};

lazy_static! {
    static ref ABI_TYPES_TO_UNMANAGED_RUST_TYPES_MAP: HashMap<&'static str, RustTypeString> = {
        let mut m = HashMap::new();

        m.insert(
            "u8",
            RustTypeString {
                type_name: "u8".to_string(),
                default_value_expr: "0u8".to_string(),
                contains_custom_types: false,
            },
        );
        m.insert(
            "u16",
            RustTypeString {
                type_name: "u16".to_string(),
                default_value_expr: "0u16".to_string(),
                contains_custom_types: false,
            },
        );
        m.insert(
            "u32",
            RustTypeString {
                type_name: "u32".to_string(),
                default_value_expr: "0u32".to_string(),
                contains_custom_types: false,
            },
        );
        m.insert(
            "u64",
            RustTypeString {
                type_name: "u64".to_string(),
                default_value_expr: "0u64".to_string(),
                contains_custom_types: false,
            },
        );

        m.insert(
            "Address",
            RustTypeString {
                type_name: "Address".to_string(),
                default_value_expr: "Address::from_slice(&b\"\"[..])".to_string(),
                contains_custom_types: false,
            },
        );
        m.insert(
            "BigUint",
            RustTypeString {
                type_name: "RustBigUint".to_string(),
                default_value_expr: "RustBigUint::from(0u64)".to_string(),
                contains_custom_types: false,
            },
        );

        let bytes_type = RustTypeString {
            type_name: "&[u8]".to_string(),
            default_value_expr: "b\"\"".to_string(),
            contains_custom_types: false,
        };

        m.insert("bytes", bytes_type.clone());
        m.insert("TokenIdentifier", bytes_type.clone());
        m.insert("EgldOrEsdtTokenIdentifier", bytes_type);

        let payment_type = RustTypeString {
            type_name: "TxTokenTransfer".to_string(),
            default_value_expr: "TxTokenTransfer::new(
            &b\"\"[..],
            0u64,
            RustBigUint::from(0u64)
        )"
            .to_string(),
            contains_custom_types: false,
        };

        m.insert("EsdtTokenPayment", payment_type.clone());
        m.insert("EgldOrEsdtTokenPayment", payment_type);

        m
    };
}

pub(crate) fn map_abi_type_to_unmanaged_rust_type(abi_type: String) -> RustTypeString {
    let mut type_string = RustTypeString::default();
    handle_abi_type(
        &mut type_string,
        abi_type,
        &ABI_TYPES_TO_UNMANAGED_RUST_TYPES_MAP,
    );
    type_string
}
