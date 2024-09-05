use multiversx_sc_codec_human_readable::{
    decode_human_readable_value, default_value_for_abi_type, encode_human_readable_value,
};
use multiversx_sc_meta::abi_json::deserialize_abi_from_json;
use multiversx_sc_scenario::multiversx_sc::{abi::ContractAbi, codec::top_encode_to_vec_u8};

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
        "EnumWithTupleValuesAndStruct": {
            "type": "enum",
            "variants": [
                {
                    "name": "First",
                    "discriminant": 0,
                    "fields": [
                        {
                            "name": "first",
                            "type": "utf-8 string"
                        },
                        {
                            "name": "second",
                            "type": "TwoU8s"
                        }
                    ]
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
        "Integrated": {
            "type": "struct",
            "fields": [
                {
                    "name": "first",
                    "type": "EnumWithTupleValuesAndStruct"
                },
                {
                    "name": "second",
                    "type": "EnumWithTupleValuesAndStruct"
                }
            ]
        }
    }
}"#;

#[test]
fn integrated_test() {
    let abi: ContractAbi = deserialize_abi_from_json(ABI_JSON).unwrap().into();

    let default_value = default_value_for_abi_type("Integrated", &abi).unwrap();
    let default_value_human =
        encode_human_readable_value(&default_value, "Integrated", &abi).unwrap();
    let default_value_decoded =
        decode_human_readable_value(&default_value_human, "Integrated", &abi).unwrap();

    let default_bytes = top_encode_to_vec_u8(&default_value).unwrap();
    let processed_bytes = top_encode_to_vec_u8(&default_value_decoded).unwrap();

    assert_eq!(default_bytes, processed_bytes);
}
