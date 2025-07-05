use multiversx_sc_codec_human_readable::interpret_value_according_to_abi;
use multiversx_sc_meta_lib::abi_json::{deserialize_abi_from_json, ContractAbiJson};
use multiversx_sc_scenario::multiversx_sc::codec::top_encode_to_vec_u8;

const TEST_ABI_JSON: &str = r#"{
    "name": "Test",
    "endpoints": [],
    "events": [],
    "esdtAttributes": [],
    "hasCallback": false,
    "types": {}
}"#;

#[test]
fn test_display_unsigned() {
    let abi_json: ContractAbiJson = deserialize_abi_from_json(TEST_ABI_JSON).unwrap();

    let result = interpret_value_according_to_abi("123", "BigUint", &abi_json).unwrap();
    let serialized = top_encode_to_vec_u8(&result).unwrap();
    assert_eq!(serialized, vec![123]);
}
