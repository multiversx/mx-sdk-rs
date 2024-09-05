use bech32::FromBase32;
use multiversx_sc_codec_human_readable::{
    decode_human_readable_value, default_value_for_abi_type, encode_human_readable_value,
    format::HumanReadableValue, AnyValue, SingleValue,
};
use multiversx_sc_meta::abi_json::deserialize_abi_from_json;
use multiversx_sc_scenario::multiversx_sc::{abi::ContractAbi, codec::top_encode_to_vec_u8};

const EMPTY_ABI_JSON: &str = r#"{
    "name": "Test",
    "endpoints": [],
    "events": [],
    "esdtAttributes": [],
    "hasCallback": false,
    "types": {}
}"#;

const TEST_ADDRESS: &str = "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u";

#[test]
fn serialize_single_value_unsigned() {
    let abi: ContractAbi = deserialize_abi_from_json(EMPTY_ABI_JSON).unwrap().into();

    let value = "1234".parse::<HumanReadableValue>().unwrap();

    let result = decode_human_readable_value(&value, "u32", &abi).unwrap();
    let serialized = top_encode_to_vec_u8(&result).unwrap();
    assert_eq!(serialized, 1234u16.to_be_bytes().to_vec()); // should take only 2 bytes (top encoded)
}

#[test]
fn deserialize_single_value_unsigned() {
    let abi: ContractAbi = deserialize_abi_from_json(EMPTY_ABI_JSON).unwrap().into();

    let value = AnyValue::SingleValue(SingleValue::UnsignedNumber(1234u16.into()));
    let result = encode_human_readable_value(&value, "u32", &abi).unwrap();

    assert_eq!(result.to_string(), "1234");
}

#[test]
fn serialize_single_value_signed() {
    let abi: ContractAbi = deserialize_abi_from_json(EMPTY_ABI_JSON).unwrap().into();

    let value = "-1234".parse::<HumanReadableValue>().unwrap();

    let result = decode_human_readable_value(&value, "i32", &abi).unwrap();
    let serialized = top_encode_to_vec_u8(&result).unwrap();
    assert_eq!(serialized, (-1234 as i16).to_be_bytes().to_vec()); // should take only 2 bytes (top encoded)
}

#[test]
fn deserialize_single_value_signed() {
    let abi: ContractAbi = deserialize_abi_from_json(EMPTY_ABI_JSON).unwrap().into();

    let value = AnyValue::SingleValue(SingleValue::SignedNumber((-1234 as i16).into()));
    let result = encode_human_readable_value(&value, "i32", &abi).unwrap();

    assert_eq!(result.to_string(), "-1234");
}

#[test]
fn serialize_single_value_managed_buffer() {
    let abi: ContractAbi = deserialize_abi_from_json(EMPTY_ABI_JSON).unwrap().into();

    let value = "[12, 34]".parse::<HumanReadableValue>().unwrap();

    let result = decode_human_readable_value(&value, "ManagedBuffer", &abi).unwrap();
    let serialized = top_encode_to_vec_u8(&result).unwrap();
    assert_eq!(serialized, vec![12, 34]);
}

#[test]
fn deserialize_single_value_managed_buffer() {
    let abi: ContractAbi = deserialize_abi_from_json(EMPTY_ABI_JSON).unwrap().into();

    let value = AnyValue::SingleValue(SingleValue::Bytes(vec![0x1, 0x2, 0x3].into()));
    let result = encode_human_readable_value(&value, "ManagedBuffer", &abi).unwrap();

    assert_eq!(result.to_string(), "[1,2,3]");
}

#[test]
fn serialize_single_value_string() {
    let abi: ContractAbi = deserialize_abi_from_json(EMPTY_ABI_JSON).unwrap().into();

    let value = r#""hello""#.parse::<HumanReadableValue>().unwrap();

    let result = decode_human_readable_value(&value, "utf-8 string", &abi).unwrap();
    let serialized = top_encode_to_vec_u8(&result).unwrap();
    assert_eq!(serialized, "hello".as_bytes().to_vec());
}

#[test]
fn deserialize_single_value_string() {
    let abi: ContractAbi = deserialize_abi_from_json(EMPTY_ABI_JSON).unwrap().into();

    let value = AnyValue::SingleValue(SingleValue::String("hello".to_owned()));
    let result = encode_human_readable_value(&value, "utf-8 string", &abi).unwrap();

    assert_eq!(result.to_string(), "\"hello\"");
}

#[test]
fn serialize_single_value_bool() {
    let abi: ContractAbi = deserialize_abi_from_json(EMPTY_ABI_JSON).unwrap().into();

    let value = "true".parse::<HumanReadableValue>().unwrap();

    let result = decode_human_readable_value(&value, "bool", &abi).unwrap();
    let serialized = top_encode_to_vec_u8(&result).unwrap();
    assert_eq!(serialized, vec![1]);
}

#[test]
fn deserialize_single_value_bool() {
    let abi: ContractAbi = deserialize_abi_from_json(EMPTY_ABI_JSON).unwrap().into();

    let value = AnyValue::SingleValue(SingleValue::Bool(true.into()));
    let result = encode_human_readable_value(&value, "bool", &abi).unwrap();

    assert_eq!(result.to_string(), "true");
}

#[test]
fn serialize_single_value_address() {
    let abi: ContractAbi = deserialize_abi_from_json(EMPTY_ABI_JSON).unwrap().into();

    let value = format!("\"{}\"", TEST_ADDRESS)
        .parse::<HumanReadableValue>()
        .unwrap();

    let result = decode_human_readable_value(&value, "Address", &abi).unwrap();
    let serialized = top_encode_to_vec_u8(&result).unwrap();

    let (_, address_bytes, _) = bech32::decode(TEST_ADDRESS).unwrap();
    let address_bytes = Vec::<u8>::from_base32(&address_bytes).unwrap();

    assert_eq!(serialized, address_bytes);
}

#[test]
fn deserialize_single_value_address() {
    let abi: ContractAbi = deserialize_abi_from_json(EMPTY_ABI_JSON).unwrap().into();
    let (_, address_bytes, _) = bech32::decode(TEST_ADDRESS).unwrap();
    let address_bytes = Vec::<u8>::from_base32(&address_bytes).unwrap();

    let value = AnyValue::SingleValue(SingleValue::Bytes(address_bytes.into()));
    let result = encode_human_readable_value(&value, "Address", &abi).unwrap();

    assert_eq!(result.to_string(), format!("\"{}\"", TEST_ADDRESS));
}

#[test]
fn default_single_values() {
    let abi: ContractAbi = deserialize_abi_from_json(EMPTY_ABI_JSON).unwrap().into();

    let AnyValue::SingleValue(SingleValue::UnsignedNumber(default_u32)) =
        default_value_for_abi_type("u32", &abi).unwrap()
    else {
        panic!("Expected default value to be a SingleValue::UnsignedNumber")
    };
    assert_eq!(default_u32, 0u32.into());

    let AnyValue::SingleValue(SingleValue::SignedNumber(default_i32)) =
        default_value_for_abi_type("i32", &abi).unwrap()
    else {
        panic!("Expected default value to be a SingleValue::SignedNumber")
    };
    assert_eq!(default_i32, 0u32.into());

    let AnyValue::SingleValue(SingleValue::Bytes(default_buffer)) =
        default_value_for_abi_type("ManagedBuffer", &abi).unwrap()
    else {
        panic!("Expected default value to be a SingleValue::Bytes")
    };
    assert_eq!(default_buffer.len(), 0);

    let AnyValue::SingleValue(SingleValue::String(default_string)) =
        default_value_for_abi_type("utf-8 string", &abi).unwrap()
    else {
        panic!("Expected default value to be a SingleValue::String")
    };
    assert_eq!(default_string, "".to_string());

    let AnyValue::SingleValue(SingleValue::Bytes(default_address)) =
        default_value_for_abi_type("Address", &abi).unwrap()
    else {
        panic!("Expected default value to be a SingleValue::Bytes")
    };
    assert_eq!(default_address.len(), 32);
    for byte in default_address.iter() {
        assert_eq!(*byte, 0);
    }

    let AnyValue::SingleValue(SingleValue::Bool(default_bool)) =
        default_value_for_abi_type("bool", &abi).unwrap()
    else {
        panic!("Expected default value to be a SingleValue::Bool")
    };
    assert_eq!(default_bool, false);
}
