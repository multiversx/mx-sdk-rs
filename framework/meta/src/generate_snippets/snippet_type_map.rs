use std::collections::HashMap;

const INNER_TYPE_SEPARATOR: char = '<';

#[derive(Clone, Default)]
pub struct RustTypeString {
    pub type_name: String,          // used for return types
    pub default_value_expr: String, // used for arguments
}

lazy_static! {
    static ref ABI_TYPES_TO_RUST_TYPES_MAP: HashMap<&'static str, RustTypeString> = {
        let mut m = HashMap::new();

        m.insert(
            "u8",
            RustTypeString {
                type_name: "u8".to_string(),
                default_value_expr: "0u8".to_string(),
            },
        );
        m.insert(
            "u16",
            RustTypeString {
                type_name: "u16".to_string(),
                default_value_expr: "0u16".to_string(),
            },
        );
        m.insert(
            "u32",
            RustTypeString {
                type_name: "u32".to_string(),
                default_value_expr: "0u32".to_string(),
            },
        );
        m.insert(
            "u64",
            RustTypeString {
                type_name: "u64".to_string(),
                default_value_expr: "0u64".to_string(),
            },
        );

        m.insert(
            "Address",
            RustTypeString {
                type_name: "ManagedAddress<DebugApi>".to_string(),
                default_value_expr: "bech32::decode(\"\")".to_string(),
            },
        );
        m.insert(
            "BigUint",
            RustTypeString {
                type_name: "BigUint<DebugApi>".to_string(),
                default_value_expr: "BigUint::from(0u64)".to_string(),
            },
        );
        m.insert(
            "TokenIdentifier",
            RustTypeString {
                type_name: "TokenIdentifier<DebugApi>".to_string(),
                default_value_expr: "TokenIdentifier::from_esdt_bytes(b\"\")".to_string(),
            },
        );
        m.insert(
            "EsdtTokenPayment",
            RustTypeString {
                type_name: "EsdtTokenPayment<DebugApi>".to_string(),
                default_value_expr: "EsdtTokenPayment::new(
                TokenIdentifier::from_esdt_bytes(b\"\"),
                0u64,
                BigUint::from(0u64)
            )"
                .to_string(),
            },
        );

        m.insert(
            "bytes",
            RustTypeString {
                type_name: "ManagedBuffer<DebugApi>".to_string(),
                default_value_expr: "ManagedBuffer::new_from_bytes(b\"\")".to_string(),
            },
        );

        m
    };
}

enum AbiType {
    UserDefined,
    Basic(RustTypeString),
    Variadic(String),
    Optional(String),
    Multi(String),
    List(String),
    Array(String, String),
    Option(String),
}

fn get_abi_type(abi_type: &str) -> AbiType {
    let opt_inner_type_start = abi_type.find(INNER_TYPE_SEPARATOR);
    if opt_inner_type_start.is_none() {
        let opt_basic_type = ABI_TYPES_TO_RUST_TYPES_MAP.get(abi_type);
        return match opt_basic_type {
            Some(basic_type) => AbiType::Basic(basic_type.clone()),
            None => AbiType::UserDefined,
        };
    }

    let inner_type_start = unsafe { opt_inner_type_start.unwrap_unchecked() };
    let (complex_type_name, inner_types) = abi_type.split_at(inner_type_start);
    if complex_type_name.starts_with("array") {
        let array_type_name_len = "array".len();
        let (_, array_size_str) = complex_type_name.split_at(array_type_name_len);
        return AbiType::Array(array_size_str.to_string(), inner_types.to_string());
    }

    match complex_type_name {
        "variadic" => AbiType::Variadic(inner_types.to_string()),
        "optional" => AbiType::Optional(inner_types.to_string()),
        "Option" => AbiType::Option(inner_types.to_string()),
        "multi" => AbiType::Multi(inner_types.to_string()),
        "List" => AbiType::List(inner_types.to_string()),
        _ => AbiType::UserDefined,
    }
}

fn handle_abi_type(type_string: &mut RustTypeString, abi_type: String) -> Result<(), ()> {
    let abi_type = get_abi_type(&abi_type);
    match abi_type {
        AbiType::UserDefined => Result::Err(()),
        AbiType::Basic(basic_type) => {
            type_string.type_name += &basic_type.type_name;
            type_string.default_value_expr += &basic_type.default_value_expr;

            Result::Ok(())
        },
        AbiType::Variadic(inner_types) => handle_variadic_type(type_string, inner_types),
        AbiType::Optional(inner_types) => handle_optional_type(type_string, inner_types),
        AbiType::Multi(inner_types) => handle_multi_type(type_string, inner_types),
        AbiType::List(inner_types) => handle_list_type(type_string, inner_types),
        AbiType::Array(array_size, inner_types) => {
            handle_array_type(type_string, array_size, inner_types)
        },
        AbiType::Option(inner_types) => handle_option_type(type_string, inner_types),
    }
}

fn handle_variadic_type(type_string: &mut RustTypeString, inner_types: String) -> Result<(), ()> {
    type_string.type_name += "MultiValueVec<";
    type_string.default_value_expr += "MultiValueVec::from(vec![";

    handle_abi_type(type_string, inner_types)?;

    type_string.type_name += ">";
    type_string.default_value_expr += "])";

    Result::Ok(())
}

fn handle_optional_type(type_string: &mut RustTypeString, inner_types: String) -> Result<(), ()> {
    type_string.type_name += "OptionalValue<";
    type_string.default_value_expr += "OptionalValue::Some(";

    handle_abi_type(type_string, inner_types)?;

    type_string.type_name += ">";
    type_string.default_value_expr += ")";

    Result::Ok(())
}

fn handle_multi_type(type_string: &mut RustTypeString, inner_types: String) -> Result<(), ()> {
    todo!();
}

fn handle_list_type(type_string: &mut RustTypeString, inner_types: String) -> Result<(), ()> {
    type_string.type_name += "ManagedVec<DebugApi, ";
    type_string.default_value_expr += "ManagedVec::from_single_item(";

    handle_abi_type(type_string, inner_types)?;

    type_string.type_name += ">";
    type_string.default_value_expr += "])";

    Result::Ok(())
}

fn handle_array_type(
    type_string: &mut RustTypeString,
    array_size: String,
    inner_types: String,
) -> Result<(), ()> {
    type_string.type_name += "[";
    type_string.default_value_expr += "[";

    handle_abi_type(type_string, inner_types)?;

    type_string.type_name += ";";
    type_string.type_name += &array_size;
    type_string.type_name += "]";

    type_string.default_value_expr += ";";
    type_string.default_value_expr += &array_size;
    type_string.default_value_expr += "]";

    Result::Ok(())
}

fn handle_option_type(type_string: &mut RustTypeString, inner_types: String) -> Result<(), ()> {
    type_string.type_name += "Option<";
    type_string.default_value_expr += "Option::Some(";

    handle_abi_type(type_string, inner_types)?;

    type_string.type_name += ">";
    type_string.default_value_expr += ")";

    Result::Ok(())
}

pub(crate) fn map_abi_type_to_rust_type(abi_type: String) -> Option<RustTypeString> {
    let mut type_string = RustTypeString::default();
    let handle_result = handle_abi_type(&mut type_string, abi_type);
    match handle_result {
        Ok(()) => Some(type_string),
        Err(()) => None,
    }
}
