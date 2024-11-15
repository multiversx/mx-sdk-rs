use multiversx_sc_scenario::imports::ReturnCode;
use multiversx_sc_snippets::network_response;
use multiversx_sc_snippets::sdk::data::transaction::{TransactionInfo, TransactionOnNetwork};

#[test]
fn test_with_multi_contract_same_shard_tx_that_has_no_sc_result() {
    // transaction data from the devnet
    // context : user -> A --call--> B, B returns a MultiValue2<u64, u64>, A returns the B's returned value
    let data = r#"
            {
              "data": {
                "transaction": {
                  "type": "normal",
                  "processingTypeOnSource": "SCInvoking",
                  "processingTypeOnDestination": "SCInvoking",
                  "hash": "e914857f1bfd003ba411bae372266703e5f706fa412c378feb37faa5e18c3d73",
                  "nonce": 49,
                  "round": 7646960,
                  "epoch": 6339,
                  "value": "0",
                  "receiver": "erd1qqqqqqqqqqqqqpgqshqmekudxlxwp0d9j368etjamr5dw7k45u7qx40w6h",
                  "sender": "erd1uh67c2lkhyj4vh73akv7jky9sfgvus8awwcj64uju69mmfne5u7q299t7g",
                  "gasPrice": 1000000000,
                  "gasLimit": 600000000,
                  "gasUsed": 600000000,
                  "data": "Y2FsbEFub3RoZXJDb250cmFjdFJldHVyblR3b1U2NEAwMDAwMDAwMDAwMDAwMDAwMDUwMEFDRkY2QjdBNEVCODEwMUE4REU3RkY3RjVEMkMwQkYzRTRENjNGNDdBNzND",
                  "signature": "53cc6496647287d735bd7950f4ec79d7b51f884defda1d6d840d722b7d0d869900ccecc01602da7a7c717955e8d4ed0711b92acd980d64ed6eebd6eaed0c4608",
                  "sourceShard": 0,
                  "destinationShard": 0,
                  "blockNonce": 7600794,
                  "blockHash": "77eb0904e56d6dd596c0d72821cf33b326fde383e72903ca4df5c2f200b0ea75",
                  "notarizedAtSourceInMetaNonce": 7609344,
                  "NotarizedAtSourceInMetaHash": "12df3fe65cacde2c9742b9506ac2261d34f3c72d690301192fd8016430d51913",
                  "notarizedAtDestinationInMetaNonce": 7609344,
                  "notarizedAtDestinationInMetaHash": "12df3fe65cacde2c9742b9506ac2261d34f3c72d690301192fd8016430d51913",
                  "miniblockType": "TxBlock",
                  "miniblockHash": "03219ac7427f7511687f0768c722c759c1b1428b2664b44a0cbe2072154851ee",
                  "hyperblockNonce": 7609344,
                  "hyperblockHash": "12df3fe65cacde2c9742b9506ac2261d34f3c72d690301192fd8016430d51913",
                  "timestamp": 1694433360,
                  "logs": {
                    "address": "erd1qqqqqqqqqqqqqpgqshqmekudxlxwp0d9j368etjamr5dw7k45u7qx40w6h",
                    "events": [
                      {
                        "address": "erd1qqqqqqqqqqqqqpgqshqmekudxlxwp0d9j368etjamr5dw7k45u7qx40w6h",
                        "identifier": "writeLog",
                        "topics": [
                          "5fXsK/a5JVZf0e2Z6ViFglDOQP1zsS1XkuaLvaZ5pzw=",
                          "QHRvbyBtdWNoIGdhcyBwcm92aWRlZCBmb3IgcHJvY2Vzc2luZzogZ2FzIHByb3ZpZGVkID0gNTk5ODA2MDAwLCBnYXMgdXNlZCA9IDM4NDcyNDA="
                        ],
                        "data": "QDZmNmJAMGFAMDIxODcxMWEwMA=="
                      },
                      {
                        "address": "erd1qqqqqqqqqqqqqpgqshqmekudxlxwp0d9j368etjamr5dw7k45u7qx40w6h",
                        "identifier": "completedTxEvent",
                        "topics": [
                          "6RSFfxv9ADukEbrjciZnA+X3BvpBLDeP6zf6peGMPXM="
                        ],
                        "data": null
                      }
                    ]
                  },
                  "status": "success",
                  "operation": "transfer",
                  "function": "callAnotherContractReturnTwoU64",
                  "initiallyPaidFee": "6192060000000000",
                  "fee": "6192060000000000",
                  "chainID": "D",
                  "version": 2,
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
    let tx_response = network_response::parse_tx_response(tx_on_network, ReturnCode::Success);

    let expected: Vec<Vec<u8>> = vec![
        hex::decode("0a").unwrap(),
        hex::decode("0218711a00").unwrap(),
    ];

    assert_eq!(tx_response.out, expected)
}

#[test]
fn test_with_multi_contract_cross_shard_tx_that_has_no_callback() {
    // transaction data from the devnet
    // context : user -> A --async call--> B, no callback
    let data = r#"
            {
              "data": {
                "transaction": {
                  "type": "normal",
                  "processingTypeOnSource": "SCInvoking",
                  "processingTypeOnDestination": "SCInvoking",
                  "hash": "4d50a055663dfee2479851684d7fb83cf00695b6f03f4dbbdf0f9232477cafc4",
                  "nonce": 51,
                  "round": 7647523,
                  "epoch": 6340,
                  "value": "0",
                  "receiver": "erd1qqqqqqqqqqqqqpgqllqglpjdrz5kn3m0k9uf9hdqjmg3xdhk6r7se3wvlk",
                  "sender": "erd1uh67c2lkhyj4vh73akv7jky9sfgvus8awwcj64uju69mmfne5u7q299t7g",
                  "gasPrice": 1000000000,
                  "gasLimit": 600000000,
                  "gasUsed": 600000000,
                  "data": "YXN5bmNDYWxsQW5vdGhlckNvbnRyYWN0UmV0dXJuVHdvVTY0Tm9DYWxsYmFja0AwMDAwMDAwMDAwMDAwMDAwMDUwMEFDRkY2QjdBNEVCODEwMUE4REU3RkY3RjVEMkMwQkYzRTRENjNGNDdBNzND",
                  "signature": "0fc30cddaa8e5365662a14344e3434cbccf287f357be99b3ed4add182f64dded774ec0d095ab1589e7c6c07e00de3b7239efc96eb2e0e97b48c1ef87084cec01",
                  "sourceShard": 0,
                  "destinationShard": 1,
                  "blockNonce": 7593758,
                  "blockHash": "a828c0ca58ef1c8aff60e512ab59f18204f1915d4a6c8285cfceb1c5725b88e8",
                  "notarizedAtSourceInMetaNonce": 7609903,
                  "NotarizedAtSourceInMetaHash": "4e90fe45c2fdccd5cf6977c1422e5f4ffa41c4e9f31fb4a50c20455f87df1e99",
                  "notarizedAtDestinationInMetaNonce": 7609907,
                  "notarizedAtDestinationInMetaHash": "10b8666a44411c3babbe20af7154fb3d47efcb1cb10d955523ec68fece26e517",
                  "miniblockType": "TxBlock",
                  "miniblockHash": "4ff4bb1ac88911d617c9b0342aeb5158db78490c2fe414cad08adcc584a77be7",
                  "hyperblockNonce": 7609907,
                  "hyperblockHash": "10b8666a44411c3babbe20af7154fb3d47efcb1cb10d955523ec68fece26e517",
                  "timestamp": 1694436738,
                  "smartContractResults": [
                    {
                      "hash": "462b56a1530e6070dc7c15f755e51a97a6972c8cd7891f3be4635b93211890c5",
                      "nonce": 0,
                      "value": 0,
                      "receiver": "erd1qqqqqqqqqqqqqpgqllqglpjdrz5kn3m0k9uf9hdqjmg3xdhk6r7se3wvlk",
                      "sender": "erd1qqqqqqqqqqqqqpgq4nlkk7jwhqgp4r08lal46tqt70jdv0685u7qrr3l2d",
                      "data": "@00@0a@0218711a00",
                      "prevTxHash": "41d56fdacf3e14de67e821427c732b62ebfa07c82d2e5db6de75fe3a1c828d9b",
                      "originalTxHash": "4d50a055663dfee2479851684d7fb83cf00695b6f03f4dbbdf0f9232477cafc4",
                      "gasLimit": 595637825,
                      "gasPrice": 1000000000,
                      "callType": 2,
                      "logs": {
                        "address": "erd1qqqqqqqqqqqqqpgqllqglpjdrz5kn3m0k9uf9hdqjmg3xdhk6r7se3wvlk",
                        "events": [
                          {
                            "address": "erd1qqqqqqqqqqqqqpgqllqglpjdrz5kn3m0k9uf9hdqjmg3xdhk6r7se3wvlk",
                            "identifier": "writeLog",
                            "topics": [
                              "AAAAAAAAAAAFAP/Aj4ZNGKlpx2+xeJLdoJbREzb20P0=",
                              "QHRvbyBtdWNoIGdhcyBwcm92aWRlZCBmb3IgcHJvY2Vzc2luZzogZ2FzIHByb3ZpZGVkID0gNTk1NjM3ODI1LCBnYXMgdXNlZCA9IDIxNjE3NzA="
                            ],
                            "data": "QDZmNmI="
                          },
                          {
                            "address": "erd1qqqqqqqqqqqqqpgqllqglpjdrz5kn3m0k9uf9hdqjmg3xdhk6r7se3wvlk",
                            "identifier": "completedTxEvent",
                            "topics": [
                              "QdVv2s8+FN5n6CFCfHMrYuv6B8gtLl223nX+OhyCjZs="
                            ],
                            "data": null
                          }
                        ]
                      },
                      "operation": "transfer"
                    },
                    {
                      "hash": "41d56fdacf3e14de67e821427c732b62ebfa07c82d2e5db6de75fe3a1c828d9b",
                      "nonce": 0,
                      "value": 0,
                      "receiver": "erd1qqqqqqqqqqqqqpgq4nlkk7jwhqgp4r08lal46tqt70jdv0685u7qrr3l2d",
                      "sender": "erd1qqqqqqqqqqqqqpgqllqglpjdrz5kn3m0k9uf9hdqjmg3xdhk6r7se3wvlk",
                      "data": "returnTwoU64@4f3c60",
                      "prevTxHash": "4d50a055663dfee2479851684d7fb83cf00695b6f03f4dbbdf0f9232477cafc4",
                      "originalTxHash": "4d50a055663dfee2479851684d7fb83cf00695b6f03f4dbbdf0f9232477cafc4",
                      "gasLimit": 597479490,
                      "gasPrice": 1000000000,
                      "callType": 1,
                      "operation": "transfer",
                      "function": "returnTwoU64"
                    }
                  ],
                  "logs": {
                    "address": "erd1qqqqqqqqqqqqqpgqllqglpjdrz5kn3m0k9uf9hdqjmg3xdhk6r7se3wvlk",
                    "events": [
                      {
                        "address": "erd1qqqqqqqqqqqqqpgqllqglpjdrz5kn3m0k9uf9hdqjmg3xdhk6r7se3wvlk",
                        "identifier": "writeLog",
                        "topics": [
                          "5fXsK/a5JVZf0e2Z6ViFglDOQP1zsS1XkuaLvaZ5pzw="
                        ],
                        "data": "QDZmNmI="
                      }
                    ]
                  },
                  "status": "success",
                  "operation": "transfer",
                  "function": "asyncCallAnotherContractReturnTwoU64NoCallback",
                  "initiallyPaidFee": "6214335000000000",
                  "fee": "6214335000000000",
                  "chainID": "D",
                  "version": 2,
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
    let tx_response = network_response::parse_tx_response(tx_on_network, ReturnCode::Success);

    let expected: Vec<Vec<u8>> = vec![];

    assert_eq!(tx_response.out, expected)
}

#[test]
fn test_with_multi_contract_cross_shard_tx_that_has_non_returning_callback() {
    // transaction data from the devnet
    // context : user -> A --async call--> B --callback--> A, the callback returns ()
    let data = r#"
            {
              "data": {
                "transaction": {
                  "type": "normal",
                  "processingTypeOnSource": "SCInvoking",
                  "processingTypeOnDestination": "SCInvoking",
                  "hash": "4f7f19e448176e4d47a0f844cbd6bdb1b6c68035dafe927e8249ed60af1c3b17",
                  "nonce": 52,
                  "round": 7647560,
                  "epoch": 6340,
                  "value": "0",
                  "receiver": "erd1qqqqqqqqqqqqqpgqllqglpjdrz5kn3m0k9uf9hdqjmg3xdhk6r7se3wvlk",
                  "sender": "erd1uh67c2lkhyj4vh73akv7jky9sfgvus8awwcj64uju69mmfne5u7q299t7g",
                  "gasPrice": 1000000000,
                  "gasLimit": 600000000,
                  "gasUsed": 600000000,
                  "data": "YXN5bmNDYWxsQW5vdGhlckNvbnRyYWN0UmV0dXJuVHdvVTY0V2l0aE5vblJldHVybmluZ0NhbGxiYWNrQDAwMDAwMDAwMDAwMDAwMDAwNTAwQUNGRjZCN0E0RUI4MTAxQThERTdGRjdGNUQyQzBCRjNFNEQ2M0Y0N0E3M0M=",
                  "signature": "3918fce429b2059b2321b709011079755dbb835e12839278ee510e4741180540e80c6111eea1d3312b2c63446de08b20e01f6040358fa94d1633c355bb65bc0f",
                  "sourceShard": 0,
                  "destinationShard": 1,
                  "blockNonce": 7593795,
                  "blockHash": "c17e727f90025225670b7852ea9807c67753c9b3f21b6ec7cc40077e3849a8b7",
                  "notarizedAtSourceInMetaNonce": 7609940,
                  "NotarizedAtSourceInMetaHash": "c67b5c550986cfd6c94d00f4b90234eb38ee196ff0d79a00d916f3bd24be272c",
                  "notarizedAtDestinationInMetaNonce": 7609944,
                  "notarizedAtDestinationInMetaHash": "d59b7398d962ce3119679af59d5d74e10083e62c3ee2b15421cc0d607979ca18",
                  "miniblockType": "TxBlock",
                  "miniblockHash": "2977affeffeb6cf41117bed442662021cb713528cdb1d0dce4537b01caeb8e0b",
                  "hyperblockNonce": 7609944,
                  "hyperblockHash": "d59b7398d962ce3119679af59d5d74e10083e62c3ee2b15421cc0d607979ca18",
                  "timestamp": 1694436960,
                  "smartContractResults": [
                    {
                      "hash": "fe7474188d5ca4b84c7577f03fc778d22d53c070dfcb05a9cda840229d30e4d3",
                      "nonce": 0,
                      "value": 0,
                      "receiver": "erd1qqqqqqqqqqqqqpgq4nlkk7jwhqgp4r08lal46tqt70jdv0685u7qrr3l2d",
                      "sender": "erd1qqqqqqqqqqqqqpgqllqglpjdrz5kn3m0k9uf9hdqjmg3xdhk6r7se3wvlk",
                      "data": "returnTwoU64@4f3c60",
                      "prevTxHash": "4f7f19e448176e4d47a0f844cbd6bdb1b6c68035dafe927e8249ed60af1c3b17",
                      "originalTxHash": "4f7f19e448176e4d47a0f844cbd6bdb1b6c68035dafe927e8249ed60af1c3b17",
                      "gasLimit": 596979545,
                      "gasPrice": 1000000000,
                      "callType": 1,
                      "operation": "transfer",
                      "function": "returnTwoU64"
                    },
                    {
                      "hash": "948dc6702b376d1e043db8de2f87ca12907c342f54cfad7dfebadf59145ca3ac",
                      "nonce": 0,
                      "value": 0,
                      "receiver": "erd1qqqqqqqqqqqqqpgqllqglpjdrz5kn3m0k9uf9hdqjmg3xdhk6r7se3wvlk",
                      "sender": "erd1qqqqqqqqqqqqqpgq4nlkk7jwhqgp4r08lal46tqt70jdv0685u7qrr3l2d",
                      "data": "@00@0a@0218711a00",
                      "prevTxHash": "fe7474188d5ca4b84c7577f03fc778d22d53c070dfcb05a9cda840229d30e4d3",
                      "originalTxHash": "4f7f19e448176e4d47a0f844cbd6bdb1b6c68035dafe927e8249ed60af1c3b17",
                      "gasLimit": 595137880,
                      "gasPrice": 1000000000,
                      "callType": 2,
                      "logs": {
                        "address": "erd1qqqqqqqqqqqqqpgqllqglpjdrz5kn3m0k9uf9hdqjmg3xdhk6r7se3wvlk",
                        "events": [
                          {
                            "address": "erd1qqqqqqqqqqqqqpgqllqglpjdrz5kn3m0k9uf9hdqjmg3xdhk6r7se3wvlk",
                            "identifier": "writeLog",
                            "topics": [
                              "AAAAAAAAAAAFAP/Aj4ZNGKlpx2+xeJLdoJbREzb20P0=",
                              "QHRvbyBtdWNoIGdhcyBwcm92aWRlZCBmb3IgcHJvY2Vzc2luZzogZ2FzIHByb3ZpZGVkID0gNTk1MTM3ODgwLCBnYXMgdXNlZCA9IDIyODg1NTA="
                            ],
                            "data": "QDZmNmJAMGFAMDIxODcxMWEwMA=="
                          },
                          {
                            "address": "erd1qqqqqqqqqqqqqpgqllqglpjdrz5kn3m0k9uf9hdqjmg3xdhk6r7se3wvlk",
                            "identifier": "completedTxEvent",
                            "topics": [
                              "/nR0GI1cpLhMdXfwP8d40i1TwHDfywWpzahAIp0w5NM="
                            ],
                            "data": null
                          }
                        ]
                      },
                      "operation": "transfer"
                    }
                  ],
                  "logs": {
                    "address": "erd1qqqqqqqqqqqqqpgqllqglpjdrz5kn3m0k9uf9hdqjmg3xdhk6r7se3wvlk",
                    "events": [
                      {
                        "address": "erd1qqqqqqqqqqqqqpgqllqglpjdrz5kn3m0k9uf9hdqjmg3xdhk6r7se3wvlk",
                        "identifier": "writeLog",
                        "topics": [
                          "5fXsK/a5JVZf0e2Z6ViFglDOQP1zsS1XkuaLvaZ5pzw="
                        ],
                        "data": "QDZmNmI="
                      }
                    ]
                  },
                  "status": "success",
                  "operation": "transfer",
                  "function": "asyncCallAnotherContractReturnTwoU64WithNonReturningCallback",
                  "initiallyPaidFee": "6235125000000000",
                  "fee": "6235125000000000",
                  "chainID": "D",
                  "version": 2,
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
    let tx_response = network_response::parse_tx_response(tx_on_network, ReturnCode::Success);

    let expected: Vec<Vec<u8>> = vec![];

    assert_eq!(tx_response.out, expected)
}

#[test]
fn test_with_multi_contract_cross_shard_tx_that_has_returning_callback() {
    // transaction data from the devnet
    // context : user -> A --async call--> B --callback--> A, the callback returns a MultiValue2<u64, u64>
    let data = r#"
            {
              "data": {
                "transaction": {
                  "type": "normal",
                  "processingTypeOnSource": "SCInvoking",
                  "processingTypeOnDestination": "SCInvoking",
                  "hash": "f34e136ca81c0e32f6fb532b753612715675073f3718b5db009bb275d246fd7a",
                  "nonce": 53,
                  "round": 7647583,
                  "epoch": 6340,
                  "value": "0",
                  "receiver": "erd1qqqqqqqqqqqqqpgqllqglpjdrz5kn3m0k9uf9hdqjmg3xdhk6r7se3wvlk",
                  "sender": "erd1uh67c2lkhyj4vh73akv7jky9sfgvus8awwcj64uju69mmfne5u7q299t7g",
                  "gasPrice": 1000000000,
                  "gasLimit": 600000000,
                  "gasUsed": 600000000,
                  "data": "YXN5bmNDYWxsQW5vdGhlckNvbnRyYWN0UmV0dXJuVHdvVTY0V2l0aFJldHVybmluZ0NhbGxiYWNrQDAwMDAwMDAwMDAwMDAwMDAwNTAwQUNGRjZCN0E0RUI4MTAxQThERTdGRjdGNUQyQzBCRjNFNEQ2M0Y0N0E3M0M=",
                  "signature": "858958d4aaf9cb0220ab2933edad3f65e1cb4c58aa7940cb0f40b489d0bd9fdf5c4736a40d6e813743ee622bb91e9f86eacf01b9a31e0ff53f9c84f13c500304",
                  "sourceShard": 0,
                  "destinationShard": 1,
                  "blockNonce": 7593818,
                  "blockHash": "b19f97110ca38d3cb15f802a00ab403491b0e5804ebc701527ab50064dc06825",
                  "notarizedAtSourceInMetaNonce": 7609963,
                  "NotarizedAtSourceInMetaHash": "4d9db6de610ca778114d44fe91dd036fac7c375c373ae9e77130d3fb9efc8391",
                  "notarizedAtDestinationInMetaNonce": 7609967,
                  "notarizedAtDestinationInMetaHash": "a4573d388c31860f9bd6f9507b65d1b3130e445abcada538f10704feba4614e7",
                  "miniblockType": "TxBlock",
                  "miniblockHash": "530f5fa3c7af474a187caca8dcea02a7a155017414147871d083bed5c49ec8f5",
                  "hyperblockNonce": 7609967,
                  "hyperblockHash": "a4573d388c31860f9bd6f9507b65d1b3130e445abcada538f10704feba4614e7",
                  "timestamp": 1694437098,
                  "smartContractResults": [
                    {
                      "hash": "065291164a8acd27c26b5a8f09664810081fda18cd54fca635196cf9b200297a",
                      "nonce": 0,
                      "value": 0,
                      "receiver": "erd1qqqqqqqqqqqqqpgq4nlkk7jwhqgp4r08lal46tqt70jdv0685u7qrr3l2d",
                      "sender": "erd1qqqqqqqqqqqqqpgqllqglpjdrz5kn3m0k9uf9hdqjmg3xdhk6r7se3wvlk",
                      "data": "returnTwoU64@4f3c60",
                      "prevTxHash": "f34e136ca81c0e32f6fb532b753612715675073f3718b5db009bb275d246fd7a",
                      "originalTxHash": "f34e136ca81c0e32f6fb532b753612715675073f3718b5db009bb275d246fd7a",
                      "gasLimit": 596994205,
                      "gasPrice": 1000000000,
                      "callType": 1,
                      "operation": "transfer",
                      "function": "returnTwoU64"
                    },
                    {
                      "hash": "bc31cb153ae615204625df84fe9ae3a159aa412b7342f3dca958dd5517a08197",
                      "nonce": 0,
                      "value": 0,
                      "receiver": "erd1qqqqqqqqqqqqqpgqllqglpjdrz5kn3m0k9uf9hdqjmg3xdhk6r7se3wvlk",
                      "sender": "erd1qqqqqqqqqqqqqpgq4nlkk7jwhqgp4r08lal46tqt70jdv0685u7qrr3l2d",
                      "data": "@00@0a@0218711a00",
                      "prevTxHash": "065291164a8acd27c26b5a8f09664810081fda18cd54fca635196cf9b200297a",
                      "originalTxHash": "f34e136ca81c0e32f6fb532b753612715675073f3718b5db009bb275d246fd7a",
                      "gasLimit": 595152540,
                      "gasPrice": 1000000000,
                      "callType": 2,
                      "logs": {
                        "address": "erd1qqqqqqqqqqqqqpgqllqglpjdrz5kn3m0k9uf9hdqjmg3xdhk6r7se3wvlk",
                        "events": [
                          {
                            "address": "erd1qqqqqqqqqqqqqpgqllqglpjdrz5kn3m0k9uf9hdqjmg3xdhk6r7se3wvlk",
                            "identifier": "writeLog",
                            "topics": [
                              "AAAAAAAAAAAFAP/Aj4ZNGKlpx2+xeJLdoJbREzb20P0=",
                              "QHRvbyBtdWNoIGdhcyBwcm92aWRlZCBmb3IgcHJvY2Vzc2luZzogZ2FzIHByb3ZpZGVkID0gNTk1MTUyNTQwLCBnYXMgdXNlZCA9IDIyODgwMTU="
                            ],
                            "data": "QDZmNmJAMGFAMDIxODcxMWEwMA=="
                          },
                          {
                            "address": "erd1qqqqqqqqqqqqqpgqllqglpjdrz5kn3m0k9uf9hdqjmg3xdhk6r7se3wvlk",
                            "identifier": "completedTxEvent",
                            "topics": [
                              "BlKRFkqKzSfCa1qPCWZIEAgf2hjNVPymNRls+bIAKXo="
                            ],
                            "data": null
                          }
                        ]
                      },
                      "operation": "transfer"
                    }
                  ],
                  "logs": {
                    "address": "erd1qqqqqqqqqqqqqpgqllqglpjdrz5kn3m0k9uf9hdqjmg3xdhk6r7se3wvlk",
                    "events": [
                      {
                        "address": "erd1qqqqqqqqqqqqqpgqllqglpjdrz5kn3m0k9uf9hdqjmg3xdhk6r7se3wvlk",
                        "identifier": "writeLog",
                        "topics": [
                          "5fXsK/a5JVZf0e2Z6ViFglDOQP1zsS1XkuaLvaZ5pzw="
                        ],
                        "data": "QDZmNmI="
                      }
                    ]
                  },
                  "status": "success",
                  "operation": "transfer",
                  "function": "asyncCallAnotherContractReturnTwoU64WithReturningCallback",
                  "initiallyPaidFee": "6230670000000000",
                  "fee": "6230670000000000",
                  "chainID": "D",
                  "version": 2,
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
    let tx_response = network_response::parse_tx_response(tx_on_network, ReturnCode::Success);

    let expected: Vec<Vec<u8>> = vec![];

    assert_eq!(tx_response.out, expected)
}
