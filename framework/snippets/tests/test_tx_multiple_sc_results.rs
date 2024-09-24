use multiversx_sc_snippets::network_response::{self, is_out_scr};
use multiversx_sdk_wbg::data::transaction::{TransactionInfo, TransactionOnNetwork};

#[test]
fn test_transaction_multiple_sc_results() {
    let data = r#"
        {
          "data": {
            "transaction": {
              "type": "normal",
              "processingTypeOnSource": "BuiltInFunctionCall",
              "processingTypeOnDestination": "SCInvoking",
              "hash": "c6dc718c56c8795156d847f4f9768f2b1976a516d390fdce0f5b8bad1ac3cce5",
              "nonce": 236,
              "round": 3353069,
              "epoch": 1371,
              "value": "0",
              "receiver": "erd1qqqqqqqqqqqqqpgq4zafu6rzdw7fj07hjh5tkm68jsaj7hl60n4s8py4ra",
              "sender": "erd1uv40ahysflse896x4ktnh6ecx43u7cmy9wnxnvcyp7deg299a4sq6vaywa",
              "gasPrice": 1000000000,
              "gasLimit": 100000000,
              "gasUsed": 12767998,
              "data": "RVNEVFRyYW5zZmVyQDU1NTQ0YjJkMzEzNDY0MzUzNzY0QDhhYzcyMzA0ODllODAwMDBANzM3NzYxNzA1NDZmNmI2NTZlNzM0NjY5Nzg2NTY0NDk2ZTcwNzU3NEA1NzQ1NDc0YzQ0MmQ2MTMyMzg2MzM1MzlAZThkNGE1MTAwMA==",
              "signature": "caed340339e3ae17a92783f5f08f96ac875885e44c25510cd11251ce23f22994985a6605c4d36f841b7110288a5e928f624f150a66a9de8ade36b68028a9af09",
              "sourceShard": 0,
              "destinationShard": 1,
              "blockNonce": 3288476,
              "blockHash": "0e70ea5fb26c58b1029c84e24eb9a661272b6253d30c668af91f167bfd67b2b0",
              "notarizedAtSourceInMetaNonce": 3290316,
              "NotarizedAtSourceInMetaHash": "8200662ca3ade8fa8e1dd3a4184b0a74d4c43de8f4153170a506f60c94ad3e8b",
              "notarizedAtDestinationInMetaNonce": 3290320,
              "notarizedAtDestinationInMetaHash": "e5f332a8f2070fd1c4ff90f5dc1ee691f36e4ecb9cb5c37e8e7c8595036c3792",
              "miniblockType": "TxBlock",
              "miniblockHash": "d271ad87c6cf8653cc950272f3ee5e976820ada80468518fa35fe45b6e33dca8",
              "hyperblockNonce": 3290320,
              "hyperblockHash": "e5f332a8f2070fd1c4ff90f5dc1ee691f36e4ecb9cb5c37e8e7c8595036c3792",
              "timestamp": 1714118414,
              "smartContractResults": [
                {
                  "hash": "c0e63f1018ece1036e3e6dc405553e5f6badfe0f5d2a104f4cd4457a872d02f9",
                  "nonce": 0,
                  "value": 0,
                  "receiver": "erd1qqqqqqqqqqqqqpgq4zafu6rzdw7fj07hjh5tkm68jsaj7hl60n4s8py4ra",
                  "sender": "erd1qqqqqqqqqqqqqpgq4zafu6rzdw7fj07hjh5tkm68jsaj7hl60n4s8py4ra",
                  "data": "swapTokensFixedInput@5745474c442d613238633539@e8d4a51000",
                  "prevTxHash": "c6dc718c56c8795156d847f4f9768f2b1976a516d390fdce0f5b8bad1ac3cce5",
                  "originalTxHash": "c6dc718c56c8795156d847f4f9768f2b1976a516d390fdce0f5b8bad1ac3cce5",
                  "gasLimit": 99559500,
                  "gasPrice": 1000000000,
                  "callType": 0,
                  "originalSender": "erd1uv40ahysflse896x4ktnh6ecx43u7cmy9wnxnvcyp7deg299a4sq6vaywa",
                  "operation": "transfer",
                  "function": "swapTokensFixedInput"
                },
                {
                  "hash": "40078cec63b6e0d0d9522ea5e6d2d0cb6f21ebae981f354de3dc3545ac2928ad",
                  "nonce": 0,
                  "value": 0,
                  "receiver": "erd1uv40ahysflse896x4ktnh6ecx43u7cmy9wnxnvcyp7deg299a4sq6vaywa",
                  "sender": "erd1qqqqqqqqqqqqqpgq4zafu6rzdw7fj07hjh5tkm68jsaj7hl60n4s8py4ra",
                  "data": "ESDTTransfer@5745474c442d613238633539@9b35e4dd3902b9",
                  "prevTxHash": "c6dc718c56c8795156d847f4f9768f2b1976a516d390fdce0f5b8bad1ac3cce5",
                  "originalTxHash": "c6dc718c56c8795156d847f4f9768f2b1976a516d390fdce0f5b8bad1ac3cce5",
                  "gasLimit": 0,
                  "gasPrice": 1000000000,
                  "callType": 0,
                  "originalSender": "erd1uv40ahysflse896x4ktnh6ecx43u7cmy9wnxnvcyp7deg299a4sq6vaywa",
                  "logs": {
                    "address": "erd1uv40ahysflse896x4ktnh6ecx43u7cmy9wnxnvcyp7deg299a4sq6vaywa",
                    "events": [
                      {
                        "address": "erd1qqqqqqqqqqqqqpgq4zafu6rzdw7fj07hjh5tkm68jsaj7hl60n4s8py4ra",
                        "identifier": "ESDTTransfer",
                        "topics": [
                          "V0VHTEQtYTI4YzU5",
                          "",
                          "mzXk3TkCuQ==",
                          "4yr+3JBP4ZOXRq2XO+s4NWPPY2QrpmmzBA+blCil7WA="
                        ],
                        "data": null,
                        "additionalData": [
                          "",
                          "RVNEVFRyYW5zZmVy",
                          "V0VHTEQtYTI4YzU5",
                          "mzXk3TkCuQ=="
                        ]
                      },
                      {
                        "address": "erd1uv40ahysflse896x4ktnh6ecx43u7cmy9wnxnvcyp7deg299a4sq6vaywa",
                        "identifier": "writeLog",
                        "topics": [
                          "AAAAAAAAAAAFAKi6nmhia7yZP9eV6LtvR5Q7L1/6fOs="
                        ],
                        "data": "QDZmNmI=",
                        "additionalData": [
                          "QDZmNmI="
                        ]
                      },
                      {
                        "address": "erd1uv40ahysflse896x4ktnh6ecx43u7cmy9wnxnvcyp7deg299a4sq6vaywa",
                        "identifier": "completedTxEvent",
                        "topics": [
                          "xtxxjFbIeVFW2Ef0+XaPKxl2pRbTkP3OD1uLrRrDzOU="
                        ],
                        "data": null,
                        "additionalData": null
                      }
                    ]
                  },
                  "tokens": [
                    "WEGLD-a28c59"
                  ],
                  "esdtValues": [
                    "43687878470468281"
                  ],
                  "operation": "ESDTTransfer"
                },
                {
                  "hash": "26487a550721b8282ceafe603bb4d53ee93929ffd9ded39b08e7422adb4d8795",
                  "nonce": 237,
                  "value": 872320020000000,
                  "receiver": "erd1uv40ahysflse896x4ktnh6ecx43u7cmy9wnxnvcyp7deg299a4sq6vaywa",
                  "sender": "erd1qqqqqqqqqqqqqpgq4zafu6rzdw7fj07hjh5tkm68jsaj7hl60n4s8py4ra",
                  "data": "@6f6b@0000000c5745474c442d6132386335390000000000000000000000079b35e4dd3902b9",
                  "prevTxHash": "c6dc718c56c8795156d847f4f9768f2b1976a516d390fdce0f5b8bad1ac3cce5",
                  "originalTxHash": "c6dc718c56c8795156d847f4f9768f2b1976a516d390fdce0f5b8bad1ac3cce5",
                  "gasLimit": 0,
                  "gasPrice": 1000000000,
                  "callType": 0,
                  "logs": {
                    "address": "erd1uv40ahysflse896x4ktnh6ecx43u7cmy9wnxnvcyp7deg299a4sq6vaywa",
                    "events": [
                      {
                        "address": "erd1uv40ahysflse896x4ktnh6ecx43u7cmy9wnxnvcyp7deg299a4sq6vaywa",
                        "identifier": "completedTxEvent",
                        "topics": [
                          "xtxxjFbIeVFW2Ef0+XaPKxl2pRbTkP3OD1uLrRrDzOU="
                        ],
                        "data": null,
                        "additionalData": null
                      }
                    ]
                  },
                  "operation": "transfer",
                  "isRefund": true
                },
                {
                  "hash": "798ba4333a7cedb62f811d942dedb8c0c09bf9945a0d2ccede2eaed967eba135",
                  "nonce": 0,
                  "value": 0,
                  "receiver": "erd1qqqqqqqqqqqqqpgqw88ux2l44eufvwz2uhvduhq03g8pxc4j0n4s0frzjz",
                  "sender": "erd1qqqqqqqqqqqqqpgq4zafu6rzdw7fj07hjh5tkm68jsaj7hl60n4s8py4ra",
                  "data": "ESDTTransfer@55544b2d313464353764@2d79883d2000@6465706f7369745377617046656573",
                  "prevTxHash": "c6dc718c56c8795156d847f4f9768f2b1976a516d390fdce0f5b8bad1ac3cce5",
                  "originalTxHash": "c6dc718c56c8795156d847f4f9768f2b1976a516d390fdce0f5b8bad1ac3cce5",
                  "gasLimit": 0,
                  "gasPrice": 1000000000,
                  "callType": 0,
                  "originalSender": "erd1uv40ahysflse896x4ktnh6ecx43u7cmy9wnxnvcyp7deg299a4sq6vaywa",
                  "tokens": [
                    "UTK-14d57d"
                  ],
                  "esdtValues": [
                    "50000000000000"
                  ],
                  "operation": "ESDTTransfer",
                  "function": "depositSwapFees"
                }
              ],
              "logs": {
                "address": "erd1qqqqqqqqqqqqqpgq4zafu6rzdw7fj07hjh5tkm68jsaj7hl60n4s8py4ra",
                "events": [
                  {
                    "address": "erd1uv40ahysflse896x4ktnh6ecx43u7cmy9wnxnvcyp7deg299a4sq6vaywa",
                    "identifier": "ESDTTransfer",
                    "topics": [
                      "VVRLLTE0ZDU3ZA==",
                      "",
                      "iscjBInoAAA=",
                      "AAAAAAAAAAAFAKi6nmhia7yZP9eV6LtvR5Q7L1/6fOs="
                    ],
                    "data": null,
                    "additionalData": [
                      "",
                      "RVNEVFRyYW5zZmVy",
                      "VVRLLTE0ZDU3ZA==",
                      "iscjBInoAAA=",
                      "c3dhcFRva2Vuc0ZpeGVkSW5wdXQ=",
                      "V0VHTEQtYTI4YzU5",
                      "6NSlEAA="
                    ]
                  },
                  {
                    "address": "erd1qqqqqqqqqqqqqpgq4zafu6rzdw7fj07hjh5tkm68jsaj7hl60n4s8py4ra",
                    "identifier": "ESDTTransfer",
                    "topics": [
                      "VVRLLTE0ZDU3ZA==",
                      "",
                      "LXmIPSAA",
                      "AAAAAAAAAAAFAHHPwyv1rniWOErl2N5cD4oOE2KyfOs="
                    ],
                    "data": "RXhlY3V0ZU9uRGVzdENvbnRleHQ=",
                    "additionalData": [
                      "RXhlY3V0ZU9uRGVzdENvbnRleHQ=",
                      "RVNEVFRyYW5zZmVy",
                      "VVRLLTE0ZDU3ZA==",
                      "LXmIPSAA",
                      "ZGVwb3NpdFN3YXBGZWVz"
                    ]
                  },
                  {
                    "address": "erd1qqqqqqqqqqqqqpgqw88ux2l44eufvwz2uhvduhq03g8pxc4j0n4s0frzjz",
                    "identifier": "depositSwapFees",
                    "topics": [
                      "ZGVwb3NpdF9zd2FwX2ZlZXNfZXZlbnQ=",
                      "AAAAAAAAAAAFAKi6nmhia7yZP9eV6LtvR5Q7L1/6fOs=",
                      "ug==",
                      "AAAAClVUSy0xNGQ1N2QAAAAAAAAAAAAAAAYteYg9IAA="
                    ],
                    "data": null,
                    "additionalData": [
                      ""
                    ]
                  },
                  {
                    "address": "erd1qqqqqqqqqqqqqpgq4zafu6rzdw7fj07hjh5tkm68jsaj7hl60n4s8py4ra",
                    "identifier": "ESDTTransfer",
                    "topics": [
                      "V0VHTEQtYTI4YzU5",
                      "",
                      "mzXk3TkCuQ==",
                      "4yr+3JBP4ZOXRq2XO+s4NWPPY2QrpmmzBA+blCil7WA="
                    ],
                    "data": "RGlyZWN0Q2FsbA==",
                    "additionalData": [
                      "RGlyZWN0Q2FsbA==",
                      "RVNEVFRyYW5zZmVy",
                      "V0VHTEQtYTI4YzU5",
                      "mzXk3TkCuQ=="
                    ]
                  },
                  {
                    "address": "erd1qqqqqqqqqqqqqpgq4zafu6rzdw7fj07hjh5tkm68jsaj7hl60n4s8py4ra",
                    "identifier": "swapTokensFixedInput",
                    "topics": [
                      "c3dhcA==",
                      "VVRLLTE0ZDU3ZA==",
                      "V0VHTEQtYTI4YzU5",
                      "4yr+3JBP4ZOXRq2XO+s4NWPPY2QrpmmzBA+blCil7WA=",
                      "BVs="
                    ],
                    "data": "4yr+3JBP4ZOXRq2XO+s4NWPPY2QrpmmzBA+blCil7WAAAAAKVVRLLTE0ZDU3ZAAAAAiKxyMEiegAAAAAAAxXRUdMRC1hMjhjNTkAAAAHmzXk3TkCuQAAAAcjhvJvwQAAAAAACwGBykedC25GCD5kAAAACgGwxHNBlOj27dQAAAAAADItnAAAAAAAAAVbAAAAAGYrXw4=",
                    "additionalData": [
                      "4yr+3JBP4ZOXRq2XO+s4NWPPY2QrpmmzBA+blCil7WAAAAAKVVRLLTE0ZDU3ZAAAAAiKxyMEiegAAAAAAAxXRUdMRC1hMjhjNTkAAAAHmzXk3TkCuQAAAAcjhvJvwQAAAAAACwGBykedC25GCD5kAAAACgGwxHNBlOj27dQAAAAAADItnAAAAAAAAAVbAAAAAGYrXw4="
                    ]
                  }
                ]
              },
              "status": "success",
              "tokens": [
                "UTK-14d57d"
              ],
              "esdtValues": [
                "10000000000000000000"
              ],
              "operation": "ESDTTransfer",
              "function": "swapTokensFixedInput",
              "initiallyPaidFee": "1238095000000000",
              "fee": "365774980000000",
              "chainID": "D",
              "version": 1,
              "options": 0
            }
          },
          "error": "",
          "code": "successful"
        }"#;

    let tx_on_network: TransactionOnNetwork = serde_json::from_str::<TransactionInfo>(data)
        .unwrap()
        .data
        .unwrap()
        .transaction;
    assert_eq!(tx_on_network.smart_contract_results.len(), 4usize);
    assert!(is_out_scr(
        &tx_on_network.smart_contract_results.get(2).unwrap()
    ));
    let _ = network_response::parse_tx_response(tx_on_network);
}
