use multiversx_sc_codec_human_readable::{
    AnyValue, EnumVariant, SingleValue, StructField, StructValue, decode_human_readable_value,
    default_value_for_abi_type, encode_human_readable_value, format::HumanReadableValue,
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
        "SimpleEnum": {
            "type": "enum",
            "variants": [
                {
                    "name": "First",
                    "discriminant": 0
                },
                {
                    "name": "Second",
                    "discriminant": 1
                }
            ]
        },
        "EnumWithStruct": {
            "type": "enum",
            "variants": [
                {
                    "name": "First",
                    "discriminant": 0
                },
                {
                    "name": "Second",
                    "discriminant": 1,
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
                }
            ]
        },
        "EnumWithTupleStruct": {
            "type": "enum",
            "variants": [
                {
                    "name": "First",
                    "discriminant": 0
                },
                {
                    "name": "Second",
                    "discriminant": 1,
                    "fields": [
                        {
                            "name": "0",
                            "type": "TwoU8s"
                        }
                    ]
                }
            ]
        },
        "EnumWithTupleValues": {
            "type": "enum",
            "variants": [
                {
                    "name": "First",
                    "discriminant": 0
                },
                {
                    "name": "Second",
                    "discriminant": 1,
                    "fields": [
                        {
                            "name": "0",
                            "type": "u8"
                        },
                        {
                            "name": "1",
                            "type": "u8"
                        }
                    ]
                }
            ]
        },
        "EnumWithTupleValuesAndStruct": {
            "type": "enum",
            "variants": [
                {
                    "name": "First",
                    "discriminant": 0
                },
                {
                    "name": "Second",
                    "discriminant": 1,
                    "fields": [
                        {
                            "name": "0",
                            "type": "u8"
                        },
                        {
                            "name": "1",
                            "type": "u8"
                        },
                        {
                            "name": "2",
                            "type": "TwoU8s"
                        }
                    ]
                }
            ]
        },
        "EnumWithDiscriminantOnlyDefault": {
            "type": "enum",
            "variants": [
                {
                    "name": "First",
                    "discriminant": 0
                },
                {
                    "name": "Second",
                    "discriminant": 1
                }
            ]
        },
        "EnumWithStructDefault": {
            "type": "enum",
            "variants": [
                {
                    "name": "First",
                    "discriminant": 0,
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
                {
                    "name": "Second",
                    "discriminant": 1
                }
            ]
        },
        "EnumWithTupleValuesDefault": {
            "type": "enum",
            "variants": [
                {
                    "name": "First",
                    "discriminant": 0,
                    "fields": [
                        {
                            "name": "0",
                            "type": "u8"
                        },
                        {
                            "name": "1",
                            "type": "u8"
                        }
                    ]
                },
                {
                    "name": "Second",
                    "discriminant": 1
                }
            ]
        }
    }
}"#;

#[test]
fn serialize_enum_only_discriminant() {
    let abi: ContractAbi = deserialize_abi_from_json(ABI_JSON).unwrap().into();

    let value = r#""Second""#.parse::<HumanReadableValue>().unwrap();

    let result = decode_human_readable_value(&value, "SimpleEnum", &abi).unwrap();
    let serialized = top_encode_to_vec_u8(&result).unwrap();
    assert_eq!(serialized, vec![1]);
}

#[test]
fn deserialize_enum_only_discriminant() {
    let abi: ContractAbi = deserialize_abi_from_json(ABI_JSON).unwrap().into();

    let value = AnyValue::Enum(Box::new(EnumVariant {
        discriminant: 1,
        value: AnyValue::None,
    }));

    let result = encode_human_readable_value(&value, "SimpleEnum", &abi).unwrap();
    assert_eq!(result.to_string(), "\"Second\"");
}

#[test]
fn serialize_enum_with_struct() {
    let abi: ContractAbi = deserialize_abi_from_json(ABI_JSON).unwrap().into();

    let value =
        r#"{ "Second": { "first": 1, "second": 2 } }"#.parse::<HumanReadableValue>().unwrap();

    let result = decode_human_readable_value(&value, "EnumWithStruct", &abi).unwrap();
    let serialized = top_encode_to_vec_u8(&result).unwrap();
    assert_eq!(
        serialized,
        vec![
            1, // discriminant
            0, 0, 0, 1, 1, // first
            0, 0, 0, 1, 2 // second
        ]
    );
}

#[test]
fn deserialize_enum_with_struct() {
    let abi: ContractAbi = deserialize_abi_from_json(ABI_JSON).unwrap().into();

    let value = AnyValue::Enum(Box::new(EnumVariant {
        discriminant: 1,
        value: AnyValue::Struct(StructValue(vec![
            StructField {
                name: "first".to_owned(),
                value: AnyValue::SingleValue(SingleValue::UnsignedNumber(1u8.into())),
            },
            StructField {
                name: "second".to_owned(),
                value: AnyValue::SingleValue(SingleValue::UnsignedNumber(2u8.into())),
            },
        ])),
    }));

    let result = encode_human_readable_value(&value, "EnumWithStruct", &abi).unwrap();
    assert_eq!(result.to_string(), r#"{"Second":{"first":1,"second":2}}"#);
}

#[test]
fn serialize_enum_tuple_with_struct() {
    let abi: ContractAbi = deserialize_abi_from_json(ABI_JSON).unwrap().into();

    let value =
        r#"{ "Second": { "first": 1, "second": 2 } }"#.parse::<HumanReadableValue>().unwrap();

    let result = decode_human_readable_value(&value, "EnumWithTupleStruct", &abi).unwrap();
    let serialized = top_encode_to_vec_u8(&result).unwrap();
    assert_eq!(
        serialized,
        vec![
            1, // discriminant
            0, 0, 0, 1, 1, // first
            0, 0, 0, 1, 2 // second
        ]
    );
}

#[test]
fn deserialize_enum_tuple_with_struct() {
    let abi: ContractAbi = deserialize_abi_from_json(ABI_JSON).unwrap().into();

    let value = AnyValue::Enum(Box::new(EnumVariant {
        discriminant: 1,
        value: AnyValue::Struct(StructValue(vec![
            StructField {
                name: "first".to_owned(),
                value: AnyValue::SingleValue(SingleValue::UnsignedNumber(1u8.into())),
            },
            StructField {
                name: "second".to_owned(),
                value: AnyValue::SingleValue(SingleValue::UnsignedNumber(2u8.into())),
            },
        ])),
    }));

    let result = encode_human_readable_value(&value, "EnumWithTupleStruct", &abi).unwrap();
    assert_eq!(result.to_string(), r#"{"Second":{"first":1,"second":2}}"#);
}

#[test]
fn serialize_enum_tuple_with_values() {
    let abi: ContractAbi = deserialize_abi_from_json(ABI_JSON).unwrap().into();

    let value = r#"{ "Second": [1, 2] }"#.parse::<HumanReadableValue>().unwrap();

    let result = decode_human_readable_value(&value, "EnumWithTupleValues", &abi).unwrap();
    let serialized = top_encode_to_vec_u8(&result).unwrap();
    assert_eq!(
        serialized,
        vec![
            1, // discriminant
            0, 0, 0, 1, 1, // 0
            0, 0, 0, 1, 2 // 1
        ]
    );
}

#[test]
fn deserialize_enum_tuple_with_values() {
    let abi: ContractAbi = deserialize_abi_from_json(ABI_JSON).unwrap().into();

    let value = AnyValue::Enum(Box::new(EnumVariant {
        discriminant: 1,
        value: AnyValue::Struct(StructValue(vec![
            StructField {
                name: "0".to_owned(),
                value: AnyValue::SingleValue(SingleValue::UnsignedNumber(1u8.into())),
            },
            StructField {
                name: "1".to_owned(),
                value: AnyValue::SingleValue(SingleValue::UnsignedNumber(2u8.into())),
            },
        ])),
    }));

    let result = encode_human_readable_value(&value, "EnumWithTupleValues", &abi).unwrap();
    assert_eq!(result.to_string(), r#"{"Second":[1,2]}"#);
}

#[test]
fn serialize_enum_tuple_with_values_and_struct() {
    let abi: ContractAbi = deserialize_abi_from_json(ABI_JSON).unwrap().into();

    let value = r#"{ "Second": [1, 2, { "first": 1, "second": 2 }] }"#
        .parse::<HumanReadableValue>()
        .unwrap();

    let result = decode_human_readable_value(&value, "EnumWithTupleValuesAndStruct", &abi).unwrap();
    let serialized = top_encode_to_vec_u8(&result).unwrap();
    assert_eq!(
        serialized,
        vec![
            1, // discriminant
            0, 0, 0, 1, 1, // 0
            0, 0, 0, 1, 2, // 1
            0, 0, 0, 1, 1, // 2.first
            0, 0, 0, 1, 2 // 2.second
        ]
    );
}

#[test]
fn deserialize_enum_tuple_with_values_and_struct() {
    let abi: ContractAbi = deserialize_abi_from_json(ABI_JSON).unwrap().into();

    let value = AnyValue::Enum(Box::new(EnumVariant {
        discriminant: 1,
        value: AnyValue::Struct(StructValue(vec![
            StructField {
                name: "0".to_owned(),
                value: AnyValue::SingleValue(SingleValue::UnsignedNumber(1u8.into())),
            },
            StructField {
                name: "1".to_owned(),
                value: AnyValue::SingleValue(SingleValue::UnsignedNumber(2u8.into())),
            },
            StructField {
                name: "2".to_owned(),
                value: AnyValue::Struct(StructValue(vec![
                    StructField {
                        name: "first".to_owned(),
                        value: AnyValue::SingleValue(SingleValue::UnsignedNumber(1u8.into())),
                    },
                    StructField {
                        name: "second".to_owned(),
                        value: AnyValue::SingleValue(SingleValue::UnsignedNumber(2u8.into())),
                    },
                ])),
            },
        ])),
    }));

    let result = encode_human_readable_value(&value, "EnumWithTupleValuesAndStruct", &abi).unwrap();
    assert_eq!(
        result.to_string(),
        r#"{"Second":[1,2,{"first":1,"second":2}]}"#
    );
}

#[test]
fn default_enum_discriminant_only() {
    let abi: ContractAbi = deserialize_abi_from_json(ABI_JSON).unwrap().into();

    let value = default_value_for_abi_type("EnumWithDiscriminantOnlyDefault", &abi).unwrap();

    let AnyValue::Enum(variant) = value else {
        panic!("Expected enum variant");
    };
    assert_eq!(variant.discriminant, 0);
    match variant.value {
        AnyValue::None => {}
        _ => panic!("Expected value none"),
    };
}

#[test]
fn default_enum_with_struct() {
    let abi: ContractAbi = deserialize_abi_from_json(ABI_JSON).unwrap().into();

    let value = default_value_for_abi_type("EnumWithStructDefault", &abi).unwrap();

    let AnyValue::Enum(variant) = value else {
        panic!("Expected enum variant");
    };
    assert_eq!(variant.discriminant, 0);
    let AnyValue::Struct(StructValue(fields)) = variant.value else {
        panic!("Expected struct value");
    };
    assert_eq!(fields.len(), 2);

    assert_eq!(fields[0].name, "first");
    let AnyValue::SingleValue(SingleValue::UnsignedNumber(num)) = &fields[0].value else {
        panic!("Expected unsigned number");
    };
    assert_eq!(*num, 0u8.into());

    assert_eq!(fields[1].name, "second");
    let AnyValue::SingleValue(SingleValue::UnsignedNumber(num)) = &fields[1].value else {
        panic!("Expected unsigned number");
    };
    assert_eq!(*num, 0u8.into());
}

#[test]
fn default_enum_with_tuple_values() {
    let abi: ContractAbi = deserialize_abi_from_json(ABI_JSON).unwrap().into();

    let value = default_value_for_abi_type("EnumWithTupleValuesDefault", &abi).unwrap();

    let AnyValue::Enum(variant) = value else {
        panic!("Expected enum variant");
    };
    assert_eq!(variant.discriminant, 0);
    let AnyValue::Struct(StructValue(fields)) = variant.value else {
        panic!("Expected struct value");
    };
    assert_eq!(fields.len(), 2);

    assert_eq!(fields[0].name, "0");
    let AnyValue::SingleValue(SingleValue::UnsignedNumber(num)) = &fields[0].value else {
        panic!("Expected unsigned number");
    };
    assert_eq!(*num, 0u8.into());

    assert_eq!(fields[1].name, "1");
    let AnyValue::SingleValue(SingleValue::UnsignedNumber(num)) = &fields[1].value else {
        panic!("Expected unsigned number");
    };
    assert_eq!(*num, 0u8.into());
}
