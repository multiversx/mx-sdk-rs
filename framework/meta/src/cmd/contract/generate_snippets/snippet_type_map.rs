use std::collections::HashMap;

const INNER_TYPE_SEPARATOR: char = '<';
const INNER_TYPE_END: char = '>';
pub static STATIC_API_SUFFIX: &str = "<StaticApi>";
pub static PLACEHOLDER_INPUT_TYPE_NAME: &str = "PlaceholderInput";

#[derive(Clone, Default)]
pub struct RustTypeString {
    type_name: String,          // used for return types
    default_value_expr: String, // used for arguments
    contains_custom_types: bool,
}

impl RustTypeString {
    pub fn get_type_name(&self) -> &str {
        &self.type_name
    }

    pub fn get_default_value_expr(&self) -> &str {
        if !self.contains_custom_types {
            &self.default_value_expr
        } else {
            PLACEHOLDER_INPUT_TYPE_NAME
        }
    }
}

lazy_static! {
    static ref ABI_TYPES_TO_RUST_TYPES_MAP: HashMap<&'static str, RustTypeString> =
        init_rust_types_map();
}

fn init_rust_types_map() -> HashMap<&'static str, RustTypeString> {
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
            type_name: "ManagedAddress".to_string() + STATIC_API_SUFFIX,
            default_value_expr: "bech32::decode(\"\")".to_string(),
            contains_custom_types: false,
        },
    );
    m.insert(
        "BigUint",
        RustTypeString {
            type_name: "BigUint".to_string() + STATIC_API_SUFFIX,
            default_value_expr: "BigUint::<StaticApi>::from(0u128)".to_string(),
            contains_custom_types: false,
        },
    );
    m.insert(
        "bytes",
        RustTypeString {
            type_name: "ManagedBuffer".to_string() + STATIC_API_SUFFIX,
            default_value_expr: "ManagedBuffer::new_from_bytes(&b\"\"[..])".to_string(),
            contains_custom_types: false,
        },
    );
    m.insert(
        "TokenIdentifier",
        RustTypeString {
            type_name: "TokenIdentifier".to_string() + STATIC_API_SUFFIX,
            default_value_expr: "TokenIdentifier::from_esdt_bytes(&b\"\"[..])".to_string(),
            contains_custom_types: false,
        },
    );
    m.insert(
        "EgldOrEsdtTokenIdentifier",
        RustTypeString {
            type_name: "EgldOrEsdtTokenIdentifier".to_string() + STATIC_API_SUFFIX,
            default_value_expr: "EgldOrEsdtTokenIdentifier::esdt(&b\"\"[..])".to_string(),
            contains_custom_types: false,
        },
    );

    m.insert(
        "EsdtTokenPayment",
        RustTypeString {
            type_name: "EsdtTokenPayment".to_string() + STATIC_API_SUFFIX,
            default_value_expr: "EsdtTokenPayment::new(
            TokenIdentifier::from_esdt_bytes(&b\"\"[..]),
            0u64,
            BigUint::from(0u128),
        )"
            .to_string(),
            contains_custom_types: false,
        },
    );
    m.insert(
        "EgldOrEsdtTokenPayment",
        RustTypeString {
            type_name: "EgldOrEsdtTokenPayment".to_string() + STATIC_API_SUFFIX,
            default_value_expr: "EgldOrEsdtTokenPayment::new(
            EgldOrEsdtTokenIdentifier::esdt(&b\"\"[..]),
            0u64,
            BigUint::from(0u128),
        )"
            .to_string(),
            contains_custom_types: false,
        },
    );

    m
}

enum AbiType {
    UserDefined(String),
    Basic(RustTypeString),
    Variadic(String),
    Optional(String),
    Multi(String),
    List(String),
    Array(String, String),
    Option(String),
}

fn get_abi_type(abi_type_str: &str) -> AbiType {
    let opt_inner_type_start = abi_type_str.find(INNER_TYPE_SEPARATOR);
    if opt_inner_type_start.is_none() {
        let separated_str: Vec<&str> = abi_type_str.split(INNER_TYPE_END).collect();
        let type_name = separated_str[0];
        let opt_basic_type = ABI_TYPES_TO_RUST_TYPES_MAP.get(type_name);
        return match opt_basic_type {
            Some(basic_type) => AbiType::Basic(basic_type.clone()),
            None => AbiType::UserDefined(type_name.to_string()),
        };
    }

    let inner_type_start = unsafe { opt_inner_type_start.unwrap_unchecked() };
    let (complex_type_name, inner_types) = abi_type_str.split_at(inner_type_start);

    // skip the '<' character
    let inner_type_str = inner_types[1..].to_string();
    if complex_type_name.starts_with("array") {
        let array_type_name_len = "array".len();
        let (_, array_size_str) = complex_type_name.split_at(array_type_name_len);

        return AbiType::Array(array_size_str.to_string(), inner_type_str);
    }

    match complex_type_name {
        "variadic" => AbiType::Variadic(inner_type_str),
        "optional" => AbiType::Optional(inner_type_str),
        "Option" => AbiType::Option(inner_type_str),
        "multi" => AbiType::Multi(inner_type_str),
        "List" => AbiType::List(inner_type_str),
        _ => AbiType::UserDefined(inner_type_str),
    }
}

fn handle_abi_type(type_string: &mut RustTypeString, abi_type_str: String) {
    let abi_type = get_abi_type(&abi_type_str);
    match abi_type {
        AbiType::UserDefined(user_type) => {
            // most user-defined types contain managed types
            type_string.type_name += &(user_type + STATIC_API_SUFFIX);
            type_string.contains_custom_types = true;
        },
        AbiType::Basic(basic_type) => {
            type_string.type_name += &basic_type.type_name;
            type_string.default_value_expr += &basic_type.default_value_expr;
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

fn handle_variadic_type(type_string: &mut RustTypeString, inner_types: String) {
    type_string.type_name += "MultiValueVec<";
    type_string.default_value_expr += "MultiValueVec::from(vec![";

    handle_abi_type(type_string, inner_types);

    type_string.type_name += ">";
    type_string.default_value_expr += "])";
}

fn handle_optional_type(type_string: &mut RustTypeString, inner_types: String) {
    type_string.type_name += "OptionalValue<";
    type_string.default_value_expr += "OptionalValue::Some(";

    handle_abi_type(type_string, inner_types);

    type_string.type_name += ">";
    type_string.default_value_expr += ")";
}

fn handle_multi_type(type_string: &mut RustTypeString, inner_types: String) {
    let multi_type_end_index = inner_types.find(INNER_TYPE_END).unwrap();
    let mut inner_multi_types: Vec<&str> = inner_types[..multi_type_end_index].split(',').collect();
    let inner_multi_types_len = inner_multi_types.len();

    type_string.type_name += "MultiValue";
    type_string.type_name += &inner_multi_types_len.to_string();
    type_string.type_name += "<";

    // "MultiValueN::from((x, y, z))"
    type_string.default_value_expr += "MultiValue";
    type_string.default_value_expr += &inner_multi_types_len.to_string();
    type_string.default_value_expr += "::from((";

    for (i, multi_type) in inner_multi_types.iter_mut().enumerate() {
        let trimmed = multi_type.trim();
        handle_abi_type(type_string, trimmed.to_string());

        if i < inner_multi_types_len - 1 {
            type_string.type_name += ", ";
            type_string.default_value_expr += ", ";
        }
    }

    type_string.type_name += ">";
    type_string.default_value_expr += "))";
}

fn handle_list_type(type_string: &mut RustTypeString, inner_types: String) {
    type_string.type_name += "ManagedVec<StaticApi, ";
    type_string.default_value_expr += "ManagedVec::from_single_item(";

    handle_abi_type(type_string, inner_types);

    type_string.type_name += ">";
    type_string.default_value_expr += ")";
}

fn handle_array_type(type_string: &mut RustTypeString, array_size: String, inner_types: String) {
    type_string.type_name += "[";
    type_string.default_value_expr += "[";

    handle_abi_type(type_string, inner_types);

    type_string.type_name += ";";
    type_string.type_name += &array_size;
    type_string.type_name += "]";

    type_string.default_value_expr += ";";
    type_string.default_value_expr += &array_size;
    type_string.default_value_expr += "]";
}

fn handle_option_type(type_string: &mut RustTypeString, inner_types: String) {
    type_string.type_name += "Option<";
    type_string.default_value_expr += "Option::Some(";

    handle_abi_type(type_string, inner_types);

    type_string.type_name += ">";
    type_string.default_value_expr += ")";
}

pub(crate) fn map_abi_type_to_rust_type(abi_type: String) -> RustTypeString {
    let mut type_string = RustTypeString::default();
    handle_abi_type(&mut type_string, abi_type);
    type_string
}
