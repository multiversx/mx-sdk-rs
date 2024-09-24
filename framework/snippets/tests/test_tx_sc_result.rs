use multiversx_sc_snippets::network_response;
use multiversx_sdk_wbg::data::transaction::{TransactionInfo, TransactionOnNetwork};

#[test]
fn test_with_tx_that_has_sc_result() {
    // transaction data from the devnet, an artificial "10" result has been appended on the original result
    let data = r#"
            {
              "data": {
                "transaction": {
                  "type": "normal",
                  "processingTypeOnSource": "BuiltInFunctionCall",
                  "processingTypeOnDestination": "SCInvoking",
                  "hash": "d4058bd3c13db0b14832c7c58fe163db2f579ff6544380586b8a27d88b4a5070",
                  "nonce": 30,
                  "round": 7639115,
                  "epoch": 6333,
                  "value": "0",
                  "receiver": "erd1qqqqqqqqqqqqqpgq5400a82at6ttplyrdhyn8kk9lhxaed5d0n4s9s77kz",
                  "sender": "erd14r7m6drneg69jyxvxxnrsss6x5gg2cqqwreyhdwanj0fcza0ynnq5jmy4g",
                  "gasPrice": 1000000000,
                  "gasLimit": 25500000,
                  "gasUsed": 15297149,
                  "data": "RVNEVFRyYW5zZmVyQDQ4NTQ0ZDJkNjY2NTMxNjYzNjM5QDBkZTBiNmIzYTc2NDAwMDBANzM3NzYxNzA1NDZmNmI2NTZlNzM0NjY5Nzg2NTY0NDk2ZTcwNzU3NEA1NzQ1NDc0YzQ0MmQ2NDM3NjMzNjYyNjJAMDM3Yzc3OGZjY2U5YzU1Yg==",
                  "signature": "e912fae4b7a9e51ddf316a5e82a0f457d453a62e3c17477f5d6175e1b33c5e92ddb187d65f54cf3131a0603321290279a0456c20778039f2ab09b54e33c60f0d",
                  "sourceShard": 2,
                  "destinationShard": 1,
                  "blockNonce": 7585351,
                  "blockHash": "e456f38f11fec78ed26d5fda068e912739dceedb2e5ce559bf17614b8386c039",
                  "notarizedAtSourceInMetaNonce": 7601495,
                  "NotarizedAtSourceInMetaHash": "e28c6011d4b3f73f3945cae70ff251e675dfea331a70077c5ab3310e3101af17",
                  "notarizedAtDestinationInMetaNonce": 7601499,
                  "notarizedAtDestinationInMetaHash": "333d4266614e981cc1c5654f85ef496038a8cddac46dfc0ad0b7c44c37ab489d",
                  "miniblockType": "TxBlock",
                  "miniblockHash": "13e041f32fde79ebf1abdcfe692e99516f9ec6778dcb917251b440daa7f1210a",
                  "hyperblockNonce": 7601499,
                  "hyperblockHash": "333d4266614e981cc1c5654f85ef496038a8cddac46dfc0ad0b7c44c37ab489d",
                  "timestamp": 1694386290,
                  "smartContractResults": [
                    {
                      "hash": "a23faa3c80bae0b968f007ff0fad3afdec05b4e71d749c3d583dec10c6eb05a2",
                      "nonce": 0,
                      "value": 0,
                      "receiver": "erd14r7m6drneg69jyxvxxnrsss6x5gg2cqqwreyhdwanj0fcza0ynnq5jmy4g",
                      "sender": "erd1qqqqqqqqqqqqqpgq5400a82at6ttplyrdhyn8kk9lhxaed5d0n4s9s77kz",
                      "data": "ESDTTransfer@5745474c442d643763366262@03856446ff9a304b",
                      "prevTxHash": "d4058bd3c13db0b14832c7c58fe163db2f579ff6544380586b8a27d88b4a5070",
                      "originalTxHash": "d4058bd3c13db0b14832c7c58fe163db2f579ff6544380586b8a27d88b4a5070",
                      "gasLimit": 0,
                      "gasPrice": 1000000000,
                      "callType": 0,
                      "logs": {
                        "address": "erd14r7m6drneg69jyxvxxnrsss6x5gg2cqqwreyhdwanj0fcza0ynnq5jmy4g",
                        "events": [
                          {
                            "address": "erd1qqqqqqqqqqqqqpgq5400a82at6ttplyrdhyn8kk9lhxaed5d0n4s9s77kz",
                            "identifier": "ESDTTransfer",
                            "topics": [
                              "V0VHTEQtZDdjNmJi",
                              "",
                              "A4VkRv+aMEs=",
                              "qP29NHPKNFkQzDGmOEIaNRCFYABw8ku13ZyenAuvJOY="
                            ],
                            "data": null
                          },
                          {
                            "address": "erd14r7m6drneg69jyxvxxnrsss6x5gg2cqqwreyhdwanj0fcza0ynnq5jmy4g",
                            "identifier": "writeLog",
                            "topics": [
                              "AAAAAAAAAAAFAKVe/p1dXpaw/INtyTPaxf3N3LaNfOs="
                            ],
                            "data": "QDZmNmI="
                          },
                          {
                            "address": "erd14r7m6drneg69jyxvxxnrsss6x5gg2cqqwreyhdwanj0fcza0ynnq5jmy4g",
                            "identifier": "completedTxEvent",
                            "topics": [
                              "1AWL08E9sLFIMsfFj+Fj2y9Xn/ZUQ4BYa4on2ItKUHA="
                            ],
                            "data": null
                          }
                        ]
                      },
                      "tokens": [
                        "WEGLD-d7c6bb"
                      ],
                      "esdtValues": [
                        "253719210115084363"
                      ],
                      "operation": "ESDTTransfer"
                    },
                    {
                      "hash": "b7b4d15917fd215399d8e772c3c4e732008baaedc2b8172f71c91708ba7523f0",
                      "nonce": 31,
                      "value": 102028510000000,
                      "receiver": "erd14r7m6drneg69jyxvxxnrsss6x5gg2cqqwreyhdwanj0fcza0ynnq5jmy4g",
                      "sender": "erd1qqqqqqqqqqqqqpgq5400a82at6ttplyrdhyn8kk9lhxaed5d0n4s9s77kz",
                      "data": "@6f6b@0000000c5745474c442d64376336626200000000000000000000000803856446ff9a304b@10",
                      "prevTxHash": "d4058bd3c13db0b14832c7c58fe163db2f579ff6544380586b8a27d88b4a5070",
                      "originalTxHash": "d4058bd3c13db0b14832c7c58fe163db2f579ff6544380586b8a27d88b4a5070",
                      "gasLimit": 0,
                      "gasPrice": 1000000000,
                      "callType": 0,
                      "logs": {
                        "address": "erd14r7m6drneg69jyxvxxnrsss6x5gg2cqqwreyhdwanj0fcza0ynnq5jmy4g",
                        "events": [
                          {
                            "address": "erd14r7m6drneg69jyxvxxnrsss6x5gg2cqqwreyhdwanj0fcza0ynnq5jmy4g",
                            "identifier": "completedTxEvent",
                            "topics": [
                              "1AWL08E9sLFIMsfFj+Fj2y9Xn/ZUQ4BYa4on2ItKUHA="
                            ],
                            "data": null
                          }
                        ]
                      },
                      "operation": "transfer",
                      "isRefund": true
                    },
                    {
                      "hash": "05a766ca05d2053d1c0fbeb1797116474a06c86402a3bfd6c132c9a24cfa1bb0",
                      "nonce": 0,
                      "value": 0,
                      "receiver": "erd1qqqqqqqqqqqqqpgq5400a82at6ttplyrdhyn8kk9lhxaed5d0n4s9s77kz",
                      "sender": "erd1qqqqqqqqqqqqqpgq5400a82at6ttplyrdhyn8kk9lhxaed5d0n4s9s77kz",
                      "data": "swapTokensFixedInput@5745474c442d643763366262@037c778fcce9c55b",
                      "prevTxHash": "d4058bd3c13db0b14832c7c58fe163db2f579ff6544380586b8a27d88b4a5070",
                      "originalTxHash": "d4058bd3c13db0b14832c7c58fe163db2f579ff6544380586b8a27d88b4a5070",
                      "gasLimit": 25050500,
                      "gasPrice": 1000000000,
                      "callType": 0,
                      "operation": "transfer",
                      "function": "swapTokensFixedInput"
                    },
                    {
                      "hash": "4e639c80822d5d7780c8326d683fa9cd6d59649d14122dfabc5a96dda36da527",
                      "nonce": 0,
                      "value": 0,
                      "receiver": "erd1qqqqqqqqqqqqqpgquu5rsa4ee6l4azz6vdu4hjp8z4p6tt8m0n4suht3dy",
                      "sender": "erd1qqqqqqqqqqqqqpgq5400a82at6ttplyrdhyn8kk9lhxaed5d0n4s9s77kz",
                      "data": "ESDTTransfer@5745474c442d643763366262@e7730d1ef1b0@737761704e6f466565416e64466f7277617264@4d45582d646332383963@0000000000000000000000000000000000000000000000000000000000000000",
                      "prevTxHash": "d4058bd3c13db0b14832c7c58fe163db2f579ff6544380586b8a27d88b4a5070",
                      "originalTxHash": "d4058bd3c13db0b14832c7c58fe163db2f579ff6544380586b8a27d88b4a5070",
                      "gasLimit": 0,
                      "gasPrice": 1000000000,
                      "callType": 0,
                      "tokens": [
                        "WEGLD-d7c6bb"
                      ],
                      "esdtValues": [
                        "254481327387056"
                      ],
                      "operation": "ESDTTransfer",
                      "function": "swapNoFeeAndForward"
                    }
                  ],
                  "logs": {
                    "address": "erd1qqqqqqqqqqqqqpgq5400a82at6ttplyrdhyn8kk9lhxaed5d0n4s9s77kz",
                    "events": [
                      {
                        "address": "erd14r7m6drneg69jyxvxxnrsss6x5gg2cqqwreyhdwanj0fcza0ynnq5jmy4g",
                        "identifier": "ESDTTransfer",
                        "topics": [
                          "SFRNLWZlMWY2OQ==",
                          "",
                          "DeC2s6dkAAA=",
                          "AAAAAAAAAAAFAKVe/p1dXpaw/INtyTPaxf3N3LaNfOs="
                        ],
                        "data": null
                      },
                      {
                        "address": "erd1qqqqqqqqqqqqqpgq5400a82at6ttplyrdhyn8kk9lhxaed5d0n4s9s77kz",
                        "identifier": "ESDTTransfer",
                        "topics": [
                          "V0VHTEQtZDdjNmJi",
                          "",
                          "53MNHvGw",
                          "AAAAAAAAAAAFAOcoOHa5zr9eiFpjeVvIJxVDpaz7fOs="
                        ],
                        "data": null
                      },
                      {
                        "address": "erd1qqqqqqqqqqqqqpgquu5rsa4ee6l4azz6vdu4hjp8z4p6tt8m0n4suht3dy",
                        "identifier": "ESDTLocalBurn",
                        "topics": [
                          "TUVYLWRjMjg5Yw==",
                          "",
                          "AuMDPq1jy03x"
                        ],
                        "data": null
                      },
                      {
                        "address": "erd1qqqqqqqqqqqqqpgquu5rsa4ee6l4azz6vdu4hjp8z4p6tt8m0n4suht3dy",
                        "identifier": "swapNoFeeAndForward",
                        "topics": [
                          "c3dhcF9ub19mZWVfYW5kX2ZvcndhcmQ=",
                          "TUVYLWRjMjg5Yw==",
                          "AAAAAAAAAAAFAKVe/p1dXpaw/INtyTPaxf3N3LaNfOs=",
                          "GL0="
                        ],
                        "data": "AAAAAAAAAAAFAKVe/p1dXpaw/INtyTPaxf3N3LaNfOsAAAAMV0VHTEQtZDdjNmJiAAAABudzDR7xsAAAAApNRVgtZGMyODljAAAACQLjAz6tY8tN8QAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABzvkcAAAAAAAAYvQAAAABk/khy"
                      },
                      {
                        "address": "erd1qqqqqqqqqqqqqpgq5400a82at6ttplyrdhyn8kk9lhxaed5d0n4s9s77kz",
                        "identifier": "ESDTTransfer",
                        "topics": [
                          "V0VHTEQtZDdjNmJi",
                          "",
                          "A4VkRv+aMEs=",
                          "qP29NHPKNFkQzDGmOEIaNRCFYABw8ku13ZyenAuvJOY="
                        ],
                        "data": null
                      },
                      {
                        "address": "erd1qqqqqqqqqqqqqpgq5400a82at6ttplyrdhyn8kk9lhxaed5d0n4s9s77kz",
                        "identifier": "swapTokensFixedInput",
                        "topics": [
                          "c3dhcA==",
                          "SFRNLWZlMWY2OQ==",
                          "V0VHTEQtZDdjNmJi",
                          "qP29NHPKNFkQzDGmOEIaNRCFYABw8ku13ZyenAuvJOY=",
                          "GL0="
                        ],
                        "data": "qP29NHPKNFkQzDGmOEIaNRCFYABw8ku13ZyenAuvJOYAAAAKSFRNLWZlMWY2OQAAAAgN4Lazp2QAAAAAAAxXRUdMRC1kN2M2YmIAAAAIA4VkRv+aMEsAAAAHA41+pMaAAAAAAAoofxtJRPkr8X9kAAAACgpOPCsHUu261HUAAAAAAHO+RwAAAAAAABi9AAAAAGT+SHI="
                      }
                    ]
                  },
                  "status": "success",
                  "tokens": [
                    "HTM-fe1f69"
                  ],
                  "esdtValues": [
                    "1000000000000000000"
                  ],
                  "operation": "ESDTTransfer",
                  "function": "swapTokensFixedInput",
                  "initiallyPaidFee": "502005000000000",
                  "fee": "399976490000000",
                  "chainID": "D",
                  "version": 1,
                  "options": 0
                }
              },
              "error": "",
              "code": "successful"
            }
        "#;

    let tx_on_network: TransactionOnNetwork = serde_json::from_str::<TransactionInfo>(data)
        .unwrap()
        .data
        .unwrap()
        .transaction;
    let tx_response = network_response::parse_tx_response(tx_on_network);

    let expected: Vec<Vec<u8>> = vec![
        hex::decode("0000000c5745474c442d64376336626200000000000000000000000803856446ff9a304b")
            .unwrap(),
        hex::decode("10").unwrap(),
    ];

    assert_eq!(tx_response.out, expected)
}

#[test]
fn test_with_tx_that_has_no_sc_result() {
    // transaction data from the devnet
    let data = r#"
            {
              "data": {
                "transaction": {
                  "type": "normal",
                  "processingTypeOnSource": "SCInvoking",
                  "processingTypeOnDestination": "SCInvoking",
                  "hash": "6afac3ec13c89cc56154d06efdb457a24f58361699eee00a48202a8f8adc8c8a",
                  "nonce": 17,
                  "round": 7548071,
                  "epoch": 6257,
                  "value": "0",
                  "receiver": "erd1qqqqqqqqqqqqqpgq4nlkk7jwhqgp4r08lal46tqt70jdv0685u7qrr3l2d",
                  "sender": "erd1uh67c2lkhyj4vh73akv7jky9sfgvus8awwcj64uju69mmfne5u7q299t7g",
                  "gasPrice": 1000000000,
                  "gasLimit": 600000000,
                  "gasUsed": 600000000,
                  "data": "cmV0dXJuVHdvVTY0",
                  "signature": "f3a3ca96a78c90c9cf1b08541e1777010f0176a5e1e525e631155b2784932cbfd74c9168d03ba201fd5434d1a1b4789895ddade9883eca2ee9e0bce18468fb00",
                  "sourceShard": 0,
                  "destinationShard": 0,
                  "blockNonce": 7502091,
                  "blockHash": "5ec66c651cb1514cba200e7e80a4491880f0db678ce7631c397872e3842f0aa2",
                  "notarizedAtSourceInMetaNonce": 7510505,
                  "NotarizedAtSourceInMetaHash": "8410309ec5b988af79b4dcfb44fd4729d46874ebd796672c78e417e314409051",
                  "notarizedAtDestinationInMetaNonce": 7510505,
                  "notarizedAtDestinationInMetaHash": "8410309ec5b988af79b4dcfb44fd4729d46874ebd796672c78e417e314409051",
                  "miniblockType": "TxBlock",
                  "miniblockHash": "fb150e515449c9b658879ed06f256b429239cbe78ec2c2821deb4b283ff21554",
                  "hyperblockNonce": 7510505,
                  "hyperblockHash": "8410309ec5b988af79b4dcfb44fd4729d46874ebd796672c78e417e314409051",
                  "timestamp": 1693840026,
                  "logs": {
                    "address": "erd1qqqqqqqqqqqqqpgq4nlkk7jwhqgp4r08lal46tqt70jdv0685u7qrr3l2d",
                    "events": [
                      {
                        "address": "erd1qqqqqqqqqqqqqpgq4nlkk7jwhqgp4r08lal46tqt70jdv0685u7qrr3l2d",
                        "identifier": "writeLog",
                        "topics": [
                          "5fXsK/a5JVZf0e2Z6ViFglDOQP1zsS1XkuaLvaZ5pzw=",
                          "QHRvbyBtdWNoIGdhcyBwcm92aWRlZCBmb3IgcHJvY2Vzc2luZzogZ2FzIHByb3ZpZGVkID0gNTk5OTMyMDAwLCBnYXMgdXNlZCA9IDE4NDE2NjU="
                        ],
                        "data": "QDZmNmJAMGFAMDIxODcxMWEwMA=="
                      },
                      {
                        "address": "erd1qqqqqqqqqqqqqpgq4nlkk7jwhqgp4r08lal46tqt70jdv0685u7qrr3l2d",
                        "identifier": "completedTxEvent",
                        "topics": [
                          "avrD7BPInMVhVNBu/bRXok9YNhaZ7uAKSCAqj4rcjIo="
                        ],
                        "data": null
                      }
                    ]
                  },
                  "status": "success",
                  "operation": "transfer",
                  "function": "returnTwoU64",
                  "initiallyPaidFee": "6067320000000000",
                  "fee": "6067320000000000",
                  "chainID": "D",
                  "version": 1,
                  "options": 0
                }
              },
              "error": "",
              "code": "successful"
            }
        "#;

    let tx_on_network: TransactionOnNetwork = serde_json::from_str::<TransactionInfo>(data)
        .unwrap()
        .data
        .unwrap()
        .transaction;
    let tx_response = network_response::parse_tx_response(tx_on_network);

    let expected: Vec<Vec<u8>> = vec![
        hex::decode("0a").unwrap(),
        hex::decode("0218711a00").unwrap(),
    ];

    assert_eq!(tx_response.out, expected)
}
