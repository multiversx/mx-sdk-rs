use multiversx_sc_codec_human_readable::interpret_value_according_to_abi;
use multiversx_sc_meta::abi_json::{deserialize_abi_from_json, ContractAbiJson};
use multiversx_sc_scenario::multiversx_sc::codec::top_encode_to_vec_u8;

const TEST_ABI_JSON: &str = r#"{
    "buildInfo": {
        "rustc": {
            "version": "1.62.0-nightly",
            "commitHash": "306ba8357fb36212b7d30efb9eb9e41659ac1445",
            "commitDate": "2022-04-05",
            "channel": "Nightly",
            "short": "rustc 1.62.0-nightly (306ba8357 2022-04-05)"
        },
        "contractCrate": {
            "name": "adder",
            "version": "0.0.0"
        },
        "framework": {
            "name": "elrond-wasm",
            "version": "0.30.0"
        }
    },
    "name": "Test",
    "endpoints": [],
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
