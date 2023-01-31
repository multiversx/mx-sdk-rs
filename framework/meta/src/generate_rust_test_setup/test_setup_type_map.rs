use std::collections::HashMap;

use crate::generate_snippets::snippet_type_map::RustTypeString;

// default_value_expr not used here
lazy_static! {
    pub(crate) static ref ABI_TYPES_TO_RUST_TEST_TYPES_MAP: HashMap<&'static str, RustTypeString> = {
        let mut m = HashMap::new();

        m.insert(
            "u8",
            RustTypeString {
                type_name: "u8".to_string(),
                default_value_expr: String::new(),
                contains_custom_types: false,
            },
        );
        m.insert(
            "u16",
            RustTypeString {
                type_name: "u16".to_string(),
                default_value_expr: String::new(),
                contains_custom_types: false,
            },
        );
        m.insert(
            "u32",
            RustTypeString {
                type_name: "u32".to_string(),
                default_value_expr: String::new(),
                contains_custom_types: false,
            },
        );
        m.insert(
            "u64",
            RustTypeString {
                type_name: "u64".to_string(),
                default_value_expr: String::new(),
                contains_custom_types: false,
            },
        );

        m.insert(
            "Address",
            RustTypeString {
                type_name: "ManagedAddress<DebugApi>".to_string(),
                default_value_expr: String::new(),
                contains_custom_types: false,
            },
        );
        m.insert(
            "BigUint",
            RustTypeString {
                type_name: "BigUint<DebugApi>".to_string(),
                default_value_expr: String::new(),
                contains_custom_types: false,
            },
        );

        m.insert(
            "bytes",
            RustTypeString {
                type_name: "ManagedBuffer<DebugApi>".to_string(),
                default_value_expr: String::new(),
                contains_custom_types: false,
            },
        );
        m.insert(
            "TokenIdentifier",
            RustTypeString {
                type_name: "TokenIdentifier<DebugApi>".to_string(),
                default_value_expr: String::new(),
                contains_custom_types: false,
            },
        );
        m.insert(
            "EgldOrEsdtTokenIdentifier",
            RustTypeString {
                type_name: "EgldOrEsdtTokenIdentifier<DebugApi>".to_string(),
                default_value_expr: String::new(),
                contains_custom_types: false,
            },
        );

        m.insert(
            "EsdtTokenPayment",
            RustTypeString {
                type_name: "EsdtTokenPayment<DebugApi>".to_string(),
                default_value_expr: String::new(),
                contains_custom_types: false,
            },
        );
        m.insert(
            "EgldOrEsdtTokenPayment",
            RustTypeString {
                type_name: "EgldOrEsdtTokenPayment<DebugApi>".to_string(),
                default_value_expr: String::new(),
                contains_custom_types: false,
            },
        );

        m
    };
}
