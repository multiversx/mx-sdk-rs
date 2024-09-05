use multiversx_sc_codec_human_readable::{
    decode_human_readable_value, default_value_for_abi_type, encode_human_readable_value,
    format::HumanReadableValue, AnyValue, SingleValue, StructField, StructValue,
};
use multiversx_sc_scenario::{
    meta::abi_json::deserialize_abi_from_json,
    multiversx_sc::{abi::ContractAbi, codec::top_encode_to_vec_u8},
};

const ABI_JSON: &str = r#"{
    "name": "Test",
    "endpoints": [],
    "events": [],
    "esdtAttributes": [],
    "hasCallback": false,
    "types": {
        "TwoU8s": {
            "type": "struct",
            "fields": [
                {
                    "name": "first",
                    "type": "u8"
                },
                {
                    "name": "second",
                    "type": "u8"
                }
            ]
        },
        "NestedStruct": {
            "type": "struct",
            "fields": [
                {
                    "name": "first",
                    "type": "u8"
                },
                {
                    "name": "second",
                    "type": "TwoU8s"
                }
            ]
        }
    }
}"#;

#[test]
fn serialize_struct_two_u8s() {
    let abi: ContractAbi = deserialize_abi_from_json(ABI_JSON).unwrap().into();

    let value = r#"{ "first": 1, "second": 2 }"#.parse::<HumanReadableValue>().unwrap();

    let result = decode_human_readable_value(&value, "TwoU8s", &abi).unwrap();
    let serialized = top_encode_to_vec_u8(&result).unwrap();
    assert_eq!(
        serialized,
        vec![
            0, 0, 0, 1, 1, // first
            0, 0, 0, 1, 2 // second
        ]
    );
}

#[test]
fn deserialize_struct_two_u8s() {
    let abi: ContractAbi = deserialize_abi_from_json(ABI_JSON).unwrap().into();

    let value = AnyValue::Struct(StructValue(vec![
        StructField {
            name: "first".to_string(),
            value: AnyValue::SingleValue(SingleValue::UnsignedNumber(1u8.into())),
        },
        StructField {
            name: "second".to_string(),
            value: AnyValue::SingleValue(SingleValue::UnsignedNumber(2u8.into())),
        },
    ]));

    let result = encode_human_readable_value(&value, "TwoU8s", &abi).unwrap();
    assert_eq!(result.to_string(), r#"{"first":1,"second":2}"#.to_string());
}

#[test]
fn default_struct_simple() {
    let abi: ContractAbi = deserialize_abi_from_json(ABI_JSON).unwrap().into();

    let AnyValue::Struct(struct_value) = default_value_for_abi_type("TwoU8s", &abi).unwrap() else {
        panic!("Expected default value to be a SingleValue::UnsignedNumber")
    };
    assert_eq!(struct_value.0.len(), 2);

    let first_field = struct_value.0.first().unwrap();
    assert_eq!(first_field.name, "first");
    let AnyValue::SingleValue(SingleValue::UnsignedNumber(first_value)) = &first_field.value else {
        panic!("Expected default value to be a SingleValue::UnsignedNumber")
    };
    assert_eq!(*first_value, 0u8.into());

    let second_field = struct_value.0.get(1).unwrap();
    assert_eq!(second_field.name, "second");
    let AnyValue::SingleValue(SingleValue::UnsignedNumber(second_value)) = &second_field.value
    else {
        panic!("Expected default value to be a SingleValue::UnsignedNumber")
    };
    assert_eq!(*second_value, 0u8.into());
}

#[test]
fn serialize_struct_nested() {
    let abi: ContractAbi = deserialize_abi_from_json(ABI_JSON).unwrap().into();

    let value = r#"{
        "first": 1,
        "second": {
            "first": 1,
            "second": 2
        }
    }"#
    .parse::<HumanReadableValue>()
    .unwrap();

    let result = decode_human_readable_value(&value, "NestedStruct", &abi).unwrap();
    let serialized = top_encode_to_vec_u8(&result).unwrap();
    assert_eq!(
        serialized,
        vec![
            0, 0, 0, 1, 1, // first
            0, 0, 0, 1, 1, // second.first
            0, 0, 0, 1, 2 // second.second
        ]
    );
}

#[test]
fn deserialize_struct_nested() {
    let abi: ContractAbi = deserialize_abi_from_json(ABI_JSON).unwrap().into();

    let value = AnyValue::Struct(StructValue(vec![
        StructField {
            name: "first".to_string(),
            value: AnyValue::SingleValue(SingleValue::UnsignedNumber(1u8.into())),
        },
        StructField {
            name: "second".to_string(),
            value: AnyValue::Struct(StructValue(vec![
                StructField {
                    name: "first".to_string(),
                    value: AnyValue::SingleValue(SingleValue::UnsignedNumber(1u8.into())),
                },
                StructField {
                    name: "second".to_string(),
                    value: AnyValue::SingleValue(SingleValue::UnsignedNumber(2u8.into())),
                },
            ])),
        },
    ]));

    let result = encode_human_readable_value(&value, "NestedStruct", &abi).unwrap();
    assert_eq!(
        result.to_string(),
        r#"{"first":1,"second":{"first":1,"second":2}}"#.to_string()
    );
}

#[test]
fn default_struct_nested() {
    let abi: ContractAbi = deserialize_abi_from_json(ABI_JSON).unwrap().into();

    let AnyValue::Struct(struct_value) = default_value_for_abi_type("NestedStruct", &abi).unwrap()
    else {
        panic!("Expected default value to be a SingleValue::UnsignedNumber")
    };
    assert_eq!(struct_value.0.len(), 2);

    let first_field = struct_value.0.first().unwrap();
    assert_eq!(first_field.name, "first");
    let AnyValue::SingleValue(SingleValue::UnsignedNumber(first_value)) = &first_field.value else {
        panic!("Expected default value to be a SingleValue::UnsignedNumber")
    };
    assert_eq!(*first_value, 0u8.into());

    let second_field = struct_value.0.get(1).unwrap();
    assert_eq!(second_field.name, "second");
    let AnyValue::Struct(nested_struct_value) = &second_field.value else {
        panic!("Expected default value to be a SingleValue::Struct")
    };

    assert_eq!(nested_struct_value.0.len(), 2);

    let first_nested_field = nested_struct_value.0.first().unwrap();
    let AnyValue::SingleValue(SingleValue::UnsignedNumber(first_nested_value)) =
        &first_nested_field.value
    else {
        panic!("Expected default value to be a SingleValue::UnsignedNumber")
    };
    assert_eq!(*first_nested_value, 0u8.into());

    let second_nested_field = nested_struct_value.0.get(1).unwrap();
    let AnyValue::SingleValue(SingleValue::UnsignedNumber(second_nested_value)) =
        &second_nested_field.value
    else {
        panic!("Expected default value to be a SingleValue::UnsignedNumber")
    };
    assert_eq!(*second_nested_value, 0u8.into());
}
