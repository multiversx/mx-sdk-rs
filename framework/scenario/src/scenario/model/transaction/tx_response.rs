use crate::multiversx_sc::types::Address;
use multiversx_chain_vm::tx_mock::TxResult;
use multiversx_sdk::data::transaction::{
    ApiLogs, ApiSmartContractResult, Events, TransactionOnNetwork,
};

use super::{
    decode_scr_data_or_panic, is_out_scr, process_topics_error, Log, TxExpect, TxResponseStatus,
};

const LOG_IDENTIFIER_SC_DEPLOY: &str = "SCDeploy";
const LOG_IDENTIFIER_SIGNAL_ERROR: &str = "signalError";

const SYSTEM_SC_BECH32: &str = "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u";

#[derive(Debug, Default, Clone)]
/// The response of a transaction.
pub struct TxResponse {
    /// The output of the transaction.
    pub out: Vec<Vec<u8>>,
    /// The address of the newly deployed smart contract.
    pub new_deployed_address: Option<Address>,
    /// The identifier of the newly issued token.
    pub new_issued_token_identifier: Option<String>,
    /// The status of the transaction.
    pub tx_error: TxResponseStatus,
    /// The logs of the transaction.
    pub logs: Vec<Log>,
    /// The gas used by the transaction.
    pub gas: u64,
    /// The refund of the transaction.
    pub refund: u64,
    /// The smart contract results of the transaction.
    pub api_scrs: Vec<ApiSmartContractResult>,
    /// The api logs of the transaction.
    pub api_logs: Option<ApiLogs>,
}

impl TxResponse {
    /// Creates a [`TxResponse`] from a [`TxResult`].
    pub fn from_tx_result(tx_result: TxResult) -> Self {
        TxResponse {
            out: tx_result.result_values,
            tx_error: TxResponseStatus {
                status: tx_result.result_status,
                message: tx_result.result_message,
            },
            ..Default::default()
        }
    }

    /// Creates a [`TxResponse`] from a [`TransactionOnNetwork`].
    pub fn from_network_tx(tx: TransactionOnNetwork) -> Self {
        let mut response = Self {
            api_scrs: tx.smart_contract_results.unwrap_or_default(),
            api_logs: tx.logs,
            ..Default::default()
        };

        response.tx_error = response.process_signal_error();
        if !response.tx_error.is_success() {
            return response;
        }

        response.process()
    }

    /// Creates a [`TxResponse`] from raw results.
    pub fn from_raw_results(raw_results: Vec<Vec<u8>>) -> Self {
        TxResponse {
            out: raw_results,
            ..Default::default()
        }
    }

    /// Creates a scenario "expect" field based on the real response.
    ///
    /// Useful for creating traces that also check the results come out always the same.
    pub fn to_expect(&self) -> TxExpect {
        if self.tx_error.is_success() {
            let mut tx_expect = TxExpect::ok();
            if self.out.is_empty() {
                tx_expect = tx_expect.no_result();
            } else {
                for raw_result in &self.out {
                    let result_hex_string = format!("0x{}", hex::encode(raw_result));
                    tx_expect = tx_expect.result(result_hex_string.as_str());
                }
            }
            tx_expect
        } else {
            TxExpect::err(
                self.tx_error.status,
                format!("str:{}", self.tx_error.message),
            )
        }
    }

    /// Checks if the transaction was successful.
    pub fn is_success(&self) -> bool {
        self.tx_error.is_success()
    }

    fn process_signal_error(&self) -> TxResponseStatus {
        if let Some(event) = self.find_log(LOG_IDENTIFIER_SIGNAL_ERROR) {
            let topics = event.topics.as_ref();
            if let Some(error) = process_topics_error(topics) {
                return TxResponseStatus::signal_error(&error);
            }

            let error_raw = base64::decode(topics.unwrap().get(1).unwrap()).unwrap();
            let error = String::from_utf8(error_raw).unwrap();
            return TxResponseStatus::signal_error(&error);
        }

        TxResponseStatus::default()
    }

    fn process(self) -> Self {
        self.process_out()
            .process_new_deployed_address()
            .process_new_issued_token_identifier()
    }

    fn process_out(mut self) -> Self {
        let out_scr = self.api_scrs.iter().find(is_out_scr);

        if let Some(out_scr) = out_scr {
            self.out = decode_scr_data_or_panic(&out_scr.data);
        } else if let Some(data) = self.process_out_from_log() {
            self.out = data
        }

        self
    }

    fn process_out_from_log(&self) -> Option<Vec<Vec<u8>>> {
        if let Some(logs) = &self.api_logs {
            logs.events
                .iter()
                .rev()
                .find_map(|event| {
                    if event.identifier == "writeLog" {
                        if let Some(data) = &event.data {
                            let decoded_data = String::from_utf8(base64::decode(data).unwrap()).unwrap();
                            if decoded_data.starts_with('@') {
                                let out = decode_scr_data_or_panic(decoded_data.as_str());
                                return Some(out)
                            }
                        }
                    }

                    None
                })
        } else {
            None
        }
    }

    fn process_new_deployed_address(mut self) -> Self {
        if let Some(event) = self.find_log(LOG_IDENTIFIER_SC_DEPLOY).cloned() {
            let topics = event.topics.as_ref();
            if process_topics_error(topics).is_some() {
                return self;
            }

            let address_raw = base64::decode(topics.unwrap().get(0).unwrap()).unwrap();
            let address: Address = Address::from_slice(address_raw.as_slice());
            self.new_deployed_address = Some(address);
        }

        self
    }

    fn process_new_issued_token_identifier(mut self) -> Self {
        let token_identifier_issue_scr: Option<&ApiSmartContractResult> = self
            .api_scrs
            .iter()
            .find(|scr| scr.sender.to_string() == SYSTEM_SC_BECH32 && scr.data.starts_with("@00@"));

        if token_identifier_issue_scr.is_none() {
            return self;
        }

        let token_identifier_issue_scr = token_identifier_issue_scr.unwrap();
        let encoded_tid = token_identifier_issue_scr.data.split('@').nth(2);
        if encoded_tid.is_none() {
            return self;
        }

        self.new_issued_token_identifier =
            Some(String::from_utf8(hex::decode(encoded_tid.unwrap()).unwrap()).unwrap());

        self
    }

    fn find_log(&self, log_identifier: &str) -> Option<&Events> {
        if let Some(logs) = &self.api_logs {
            logs.events
                .iter()
                .find(|event| event.identifier == log_identifier)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use multiversx_sdk::data::transaction::{TransactionInfo, TransactionOnNetwork};
    use crate::scenario_model::TxResponse;

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

        let tx_on_network: TransactionOnNetwork = serde_json::from_str::<TransactionInfo>(data).unwrap().data.unwrap().transaction;
        let tx_response = TxResponse::from_network_tx(tx_on_network);

        let expected: Vec<Vec<u8>> = vec![
            hex::decode("0000000c5745474c442d64376336626200000000000000000000000803856446ff9a304b").unwrap(),
            hex::decode("10").unwrap()
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

        let tx_on_network: TransactionOnNetwork = serde_json::from_str::<TransactionInfo>(data).unwrap().data.unwrap().transaction;
        let tx_response = TxResponse::from_network_tx(tx_on_network);

        let expected: Vec<Vec<u8>> = vec![
            hex::decode("0a").unwrap(),
            hex::decode("0218711a00").unwrap()
        ];

        assert_eq!(tx_response.out, expected)
    }

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

        let tx_on_network: TransactionOnNetwork = serde_json::from_str::<TransactionInfo>(data).unwrap().data.unwrap().transaction;
        let tx_response = TxResponse::from_network_tx(tx_on_network);

        let expected: Vec<Vec<u8>> = vec![
            hex::decode("0a").unwrap(),
            hex::decode("0218711a00").unwrap()
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

        let tx_on_network: TransactionOnNetwork = serde_json::from_str::<TransactionInfo>(data).unwrap().data.unwrap().transaction;
        let tx_response = TxResponse::from_network_tx(tx_on_network);

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

        let tx_on_network: TransactionOnNetwork = serde_json::from_str::<TransactionInfo>(data).unwrap().data.unwrap().transaction;
        let tx_response = TxResponse::from_network_tx(tx_on_network);

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

        let tx_on_network: TransactionOnNetwork = serde_json::from_str::<TransactionInfo>(data).unwrap().data.unwrap().transaction;
        let tx_response = TxResponse::from_network_tx(tx_on_network);

        let expected: Vec<Vec<u8>> = vec![];

        assert_eq!(tx_response.out, expected)
    }
}