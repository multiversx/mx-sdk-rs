use crate::multiversx_sc::types::Address;
use multiversx_chain_vm::tx_mock::TxResult;
use multiversx_sdk::{
    data::transaction::{ApiLogs, ApiSmartContractResult, Events, TransactionOnNetwork},
    utils::base64_decode,
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

            let error_raw = base64_decode(topics.unwrap().get(1).unwrap());
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
            logs.events.iter().rev().find_map(|event| {
                if event.identifier == "writeLog" {
                    if let Some(data) = &event.data {
                        let decoded_data = String::from_utf8(base64_decode(data)).unwrap();

                        if decoded_data.starts_with('@') {
                            let out = decode_scr_data_or_panic(decoded_data.as_str());
                            return Some(out);
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

            let address_raw = base64_decode(topics.unwrap().first().unwrap());

            let address: Address = Address::from_slice(address_raw.as_slice());
            self.new_deployed_address = Some(address);
        }

        self
    }

    fn process_new_issued_token_identifier(mut self) -> Self {
        for scr in self.api_scrs.iter() {
            if scr.sender.to_string() != SYSTEM_SC_BECH32 {
                continue;
            }

            let Some(prev_tx) = self.api_scrs.iter().find(|e| e.hash == scr.prev_tx_hash) else {
                continue;
            };

            let is_issue_fungible = prev_tx.data.starts_with("issue@");
            let is_issue_semi_fungible = prev_tx.data.starts_with("issueSemiFungible@");
            let is_issue_non_fungible = prev_tx.data.starts_with("issueNonFungible@");
            let is_register_meta_esdt = prev_tx.data.starts_with("registerMetaESDT@");

            if !is_issue_fungible
                && !is_issue_semi_fungible
                && !is_issue_non_fungible
                && !is_register_meta_esdt
            {
                continue;
            }

            if scr.data.starts_with("ESDTTransfer@") {
                let encoded_tid = scr.data.split('@').nth(1);
                if encoded_tid.is_none() {
                    return self;
                }

                self.new_issued_token_identifier =
                    Some(String::from_utf8(hex::decode(encoded_tid.unwrap()).unwrap()).unwrap());

                break;
            } else if scr.data.starts_with("@00@") {
                let encoded_tid = scr.data.split('@').nth(2);
                if encoded_tid.is_none() {
                    return self;
                }

                self.new_issued_token_identifier =
                    Some(String::from_utf8(hex::decode(encoded_tid.unwrap()).unwrap()).unwrap());

                break;
            }
        }

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
    use crate::scenario_model::TxResponse;
    use multiversx_sdk::data::transaction::{TransactionInfo, TransactionOnNetwork};

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
        let tx_response = TxResponse::from_network_tx(tx_on_network);

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
        let tx_response = TxResponse::from_network_tx(tx_on_network);

        let expected: Vec<Vec<u8>> = vec![
            hex::decode("0a").unwrap(),
            hex::decode("0218711a00").unwrap(),
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

        let tx_on_network: TransactionOnNetwork = serde_json::from_str::<TransactionInfo>(data)
            .unwrap()
            .data
            .unwrap()
            .transaction;
        let tx_response = TxResponse::from_network_tx(tx_on_network);

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

        let tx_on_network: TransactionOnNetwork = serde_json::from_str::<TransactionInfo>(data)
            .unwrap()
            .data
            .unwrap()
            .transaction;
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

        let tx_on_network: TransactionOnNetwork = serde_json::from_str::<TransactionInfo>(data)
            .unwrap()
            .data
            .unwrap()
            .transaction;
        let tx_response = TxResponse::from_network_tx(tx_on_network);

        let expected: Vec<Vec<u8>> = vec![];

        assert_eq!(tx_response.out, expected)
    }

    #[test]
    fn test_process_issued_token_identifier_fungible() {
        let data = r#"
        {
  "data": {
    "transaction": {
      "type": "normal",
      "processingTypeOnSource": "SCInvoking",
      "processingTypeOnDestination": "SCInvoking",
      "hash": "b78170cc5ca5ba441ea46fe84540db9610ccab243ccd4cd3cd976e170c4864c8",
      "nonce": 61,
      "round": 173598,
      "epoch": 72,
      "value": "50000000000000000",
      "receiver": "erd1qqqqqqqqqqqqqpgqa7hv0nahgsl8tz0psat46x0tchm0wuyc0n4s6q28ad",
      "sender": "erd1x39tc3q3nn72ecjnmcz7x0qp09kp97t080x99dgyhx7zh95j0n4szskhlv",
      "gasPrice": 1000000000,
      "gasLimit": 100000000,
      "gasUsed": 100000000,
      "data": "aXNzdWVMcFRva2VuQDAwMDAwMDAwMDAwMDAwMDAwNTAwMTM5ZWQ3YWU0YWEwMzc5MmU2YmNiMzMyMzk0YTQwZmU3NDZlZWZhNDdjZWJANDU0NzRjNDQ0ZDQ1NTg0YzUwQDQ1NDc0YzQ0NGQ0NTU4",
      "signature": "b5049d2906adc1305a6a8d0f42749254ca6259c6996d9a35e7dc7528b3c87b48a421879aff70bc6d81483a7559b75e5dcf9be499dcb7d57aa9f25c79ac2ad40d",
      "sourceShard": 1,
      "destinationShard": 1,
      "blockNonce": 173354,
      "blockHash": "09d85ac264a54e12e7613395211c53fe0ee5a7d3b7111bf5fec1d02794caaacd",
      "notarizedAtSourceInMetaNonce": 173321,
      "NotarizedAtSourceInMetaHash": "64a83759da97fe8305cd4cda4b518f2d41ef0a8f3995d264460821edad45e09e",
      "notarizedAtDestinationInMetaNonce": 173321,
      "notarizedAtDestinationInMetaHash": "64a83759da97fe8305cd4cda4b518f2d41ef0a8f3995d264460821edad45e09e",
      "miniblockType": "TxBlock",
      "miniblockHash": "7f45eee4e35ffc1fbce66b92e4dd2aeae2acb092416aa5aa775b96493256b81d",
      "hyperblockNonce": 173321,
      "hyperblockHash": "64a83759da97fe8305cd4cda4b518f2d41ef0a8f3995d264460821edad45e09e",
      "timestamp": 1695041588,
      "smartContractResults": [
        {
          "hash": "bce3d0dceb0b3e5c8c5780d7da3755c3f7492d551685d493a73bf66ebd36754b",
          "nonce": 0,
          "value": 50000000000000000,
          "receiver": "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u",
          "sender": "erd1qqqqqqqqqqqqqpgqa7hv0nahgsl8tz0psat46x0tchm0wuyc0n4s6q28ad",
          "data": "issue@45474c444d45584c50@45474c444d4558@03e8@12@63616e467265657a65@74727565@63616e57697065@74727565@63616e5061757365@74727565@63616e4d696e74@74727565@63616e4275726e@74727565@63616e4368616e67654f776e6572@74727565@63616e55706772616465@74727565@63616e4164645370656369616c526f6c6573@74727565@65ba30",
          "prevTxHash": "b78170cc5ca5ba441ea46fe84540db9610ccab243ccd4cd3cd976e170c4864c8",
          "originalTxHash": "b78170cc5ca5ba441ea46fe84540db9610ccab243ccd4cd3cd976e170c4864c8",
          "gasLimit": 89624222,
          "gasPrice": 1000000000,
          "callType": 1,
          "originalSender": "erd1x39tc3q3nn72ecjnmcz7x0qp09kp97t080x99dgyhx7zh95j0n4szskhlv",
          "operation": "transfer",
          "function": "issue"
        },
        {
          "hash": "2a452ff652791d79be5f6933fb583cc5503e876893e54b3b51381a92aa2e904d",
          "nonce": 0,
          "value": 0,
          "receiver": "erd1llllllllllllllllllllllllllllllllllllllllllllllllluqsl6e366",
          "sender": "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u",
          "data": "ESDTSetBurnRoleForAll@45474c444d45582d393563366435",
          "prevTxHash": "bce3d0dceb0b3e5c8c5780d7da3755c3f7492d551685d493a73bf66ebd36754b",
          "originalTxHash": "b78170cc5ca5ba441ea46fe84540db9610ccab243ccd4cd3cd976e170c4864c8",
          "gasLimit": 0,
          "gasPrice": 1000000000,
          "callType": 0,
          "logs": {
            "address": "erd1llllllllllllllllllllllllllllllllllllllllllllllllluqsl6e366",
            "events": [
              {
                "address": "erd1llllllllllllllllllllllllllllllllllllllllllllllllluqsl6e366",
                "identifier": "completedTxEvent",
                "topics": [
                  "vOPQ3OsLPlyMV4DX2jdVw/dJLVUWhdSTpzv2br02dUs="
                ],
                "data": null,
                "additionalData": null
              }
            ]
          },
          "operation": "transfer"
        },
        {
          "hash": "2c84740ccb3376ea9fa00dab6c6c93fe7a35ee0a1d6dbfa0a1e61064853b0874",
          "nonce": 0,
          "value": 0,
          "receiver": "erd1qqqqqqqqqqqqqpgqa7hv0nahgsl8tz0psat46x0tchm0wuyc0n4s6q28ad",
          "sender": "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u",
          "data": "ESDTTransfer@45474c444d45582d393563366435@03e8@00",
          "prevTxHash": "bce3d0dceb0b3e5c8c5780d7da3755c3f7492d551685d493a73bf66ebd36754b",
          "originalTxHash": "b78170cc5ca5ba441ea46fe84540db9610ccab243ccd4cd3cd976e170c4864c8",
          "gasLimit": 39624222,
          "gasPrice": 1000000000,
          "callType": 2,
          "logs": {
            "address": "erd1qqqqqqqqqqqqqpgqa7hv0nahgsl8tz0psat46x0tchm0wuyc0n4s6q28ad",
            "events": [
              {
                "address": "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u",
                "identifier": "ESDTTransfer",
                "topics": [
                  "RUdMRE1FWC05NWM2ZDU=",
                  "",
                  "A+g=",
                  "AAAAAAAAAAAFAO+ux8+3RD51ieGHV10Z68X293CYfOs="
                ],
                "data": null,
                "additionalData": null
              },
              {
                "address": "erd1qqqqqqqqqqqqqpgqa7hv0nahgsl8tz0psat46x0tchm0wuyc0n4s6q28ad",
                "identifier": "completedTxEvent",
                "topics": [
                  "vOPQ3OsLPlyMV4DX2jdVw/dJLVUWhdSTpzv2br02dUs="
                ],
                "data": null,
                "additionalData": null
              }
            ]
          },
          "tokens": [
            "EGLDMEX-95c6d5"
          ],
          "esdtValues": [
            "1000"
          ],
          "operation": "ESDTTransfer",
          "function": "\u0000"
        },
        {
          "hash": "c9dfc4de3c3cee319123087a4f5dd03cc051e728ec6070707a63ea977b535227",
          "nonce": 0,
          "value": 0,
          "receiver": "erd1qqqqqqqqqqqqqpgqa7hv0nahgsl8tz0psat46x0tchm0wuyc0n4s6q28ad",
          "sender": "erd1qqqqqqqqqqqqqpgqa7hv0nahgsl8tz0psat46x0tchm0wuyc0n4s6q28ad",
          "data": "\u0000",
          "prevTxHash": "2c84740ccb3376ea9fa00dab6c6c93fe7a35ee0a1d6dbfa0a1e61064853b0874",
          "originalTxHash": "b78170cc5ca5ba441ea46fe84540db9610ccab243ccd4cd3cd976e170c4864c8",
          "gasLimit": 39424222,
          "gasPrice": 1000000000,
          "callType": 2,
          "operation": "transfer",
          "function": "\u0000"
        },
        {
          "hash": "609c3a8e1903680fef1f6d9e47527b1b5c1259664b868af600162120ce0b8192",
          "nonce": 1,
          "value": 300925400000000,
          "receiver": "erd1qqqqqqqqqqqqqpgqa7hv0nahgsl8tz0psat46x0tchm0wuyc0n4s6q28ad",
          "sender": "erd1qqqqqqqqqqqqqpgqa7hv0nahgsl8tz0psat46x0tchm0wuyc0n4s6q28ad",
          "data": "@6f6b",
          "prevTxHash": "2c84740ccb3376ea9fa00dab6c6c93fe7a35ee0a1d6dbfa0a1e61064853b0874",
          "originalTxHash": "b78170cc5ca5ba441ea46fe84540db9610ccab243ccd4cd3cd976e170c4864c8",
          "gasLimit": 0,
          "gasPrice": 1000000000,
          "callType": 0,
          "operation": "transfer",
          "isRefund": true
        }
      ],
      "logs": {
        "address": "erd1qqqqqqqqqqqqqpgqa7hv0nahgsl8tz0psat46x0tchm0wuyc0n4s6q28ad",
        "events": [
          {
            "address": "erd1qqqqqqqqqqqqqpgqa7hv0nahgsl8tz0psat46x0tchm0wuyc0n4s6q28ad",
            "identifier": "transferValueOnly",
            "topics": [
              "AAAAAAAAAAAFAO+ux8+3RD51ieGHV10Z68X293CYfOs=",
              "AAAAAAAAAAAAAQAAAAAAAAAAAAAAAAAAAAAAAAAC//8=",
              "saK8LsUAAA=="
            ],
            "data": null,
            "additionalData": null
          },
          {
            "address": "erd1qqqqqqqqqqqqqpgqa7hv0nahgsl8tz0psat46x0tchm0wuyc0n4s6q28ad",
            "identifier": "writeLog",
            "topics": [
              "NEq8RBGc/KziU94F4zwBeWwS+W87zFK1BLm8K5aSfOs="
            ],
            "data": "QDZmNmI=",
            "additionalData": null
          }
        ]
      },
      "status": "success",
      "operation": "transfer",
      "function": "issueLpToken",
      "initiallyPaidFee": "1214335000000000",
      "fee": "1214335000000000",
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
        let tx_response = TxResponse::from_network_tx(tx_on_network);

        let expected: Option<String> = Some("EGLDMEX-95c6d5".to_string());

        assert_eq!(tx_response.new_issued_token_identifier, expected)
    }

    #[test]
    fn test_process_issued_token_identifier_semi_fungible() {
        let data = r#"
        {
  "data": {
    "transaction": {
      "type": "normal",
      "processingTypeOnSource": "SCInvoking",
      "processingTypeOnDestination": "SCInvoking",
      "hash": "0634b9c1db9fd6bfa065fc937d51cec37958fd5d33d0c934a0da3d27776a33ae",
      "nonce": 65,
      "round": 8422527,
      "epoch": 584,
      "value": "50000000000000000",
      "receiver": "erd1qqqqqqqqqqqqqpgq06w7lq7relxyh2h6xzh98q8x24psf3fqssvqn4ptek",
      "sender": "erd1x3g000ew7zzv6kyqhj9jl2wy5g6cc72qahvvxz29zv76jwq6ssvqt0d998",
      "gasPrice": 1000000000,
      "gasLimit": 80000000,
      "gasUsed": 80000000,
      "data": "aXNzdWVUb2tlbkA0NDZmNzA2NTU0NjU3Mzc0QDQ0NGY1MDQ1NTQ0NTUzNTQ=",
      "signature": "0191848976e930996f6c62d4921e732f9b0ada8b41ca3b5b63d6bfd304fd44c2a1e8e6643479618ba4a764a36e87f53882b4f707600d5b7d476f2fdd2bac040e",
      "sourceShard": 0,
      "destinationShard": 0,
      "blockNonce": 8420241,
      "blockHash": "4d302220f6015876c95e7961b770cc67f8ab63c5f0ab69b4d6c2fb15c8bc23bd",
      "notarizedAtSourceInMetaNonce": 8403647,
      "NotarizedAtSourceInMetaHash": "f8b83b6d38fa45dacc167b15c93dd07ee5c40db906de34f26e11e7a24f539e30",
      "notarizedAtDestinationInMetaNonce": 8403647,
      "notarizedAtDestinationInMetaHash": "f8b83b6d38fa45dacc167b15c93dd07ee5c40db906de34f26e11e7a24f539e30",
      "miniblockType": "TxBlock",
      "miniblockHash": "b7b8fc9f3b81d7daae1113cbf73457e16ee31f3a864ef3729a1a21f3a929e112",
      "hyperblockNonce": 8403647,
      "hyperblockHash": "f8b83b6d38fa45dacc167b15c93dd07ee5c40db906de34f26e11e7a24f539e30",
      "timestamp": 1646652762,
      "smartContractResults": [
        {
          "hash": "9aecf3bd5dd5c706a28d1cc7059ac20db74340f136816f667dbefcc58daa3aba",
          "nonce": 0,
          "value": 50000000000000000,
          "receiver": "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u",
          "sender": "erd1qqqqqqqqqqqqqpgq06w7lq7relxyh2h6xzh98q8x24psf3fqssvqn4ptek",
          "data": "issueSemiFungible@446f706554657374@444f504554455354@63616e467265657a65@74727565@63616e57697065@74727565@63616e5061757365@74727565@63616e4368616e67654f776e6572@74727565@63616e55706772616465@74727565@63616e4164645370656369616c526f6c6573@74727565@5ca148",
          "prevTxHash": "0634b9c1db9fd6bfa065fc937d51cec37958fd5d33d0c934a0da3d27776a33ae",
          "originalTxHash": "0634b9c1db9fd6bfa065fc937d51cec37958fd5d33d0c934a0da3d27776a33ae",
          "gasLimit": 75958360,
          "gasPrice": 1000000000,
          "callType": 1,
          "originalSender": "erd1x3g000ew7zzv6kyqhj9jl2wy5g6cc72qahvvxz29zv76jwq6ssvqt0d998",
          "operation": "transfer",
          "function": "issueSemiFungible"
        },
        {
          "hash": "aacfe9088bb9d2d5b3fbe9cab2b2f1c6a7e9cbab2f1a41020e2c819fc9b43570",
          "nonce": 66,
          "value": 0,
          "receiver": "erd1x3g000ew7zzv6kyqhj9jl2wy5g6cc72qahvvxz29zv76jwq6ssvqt0d998",
          "sender": "erd1qqqqqqqqqqqqqpgq06w7lq7relxyh2h6xzh98q8x24psf3fqssvqn4ptek",
          "data": "@6f6b",
          "prevTxHash": "0634b9c1db9fd6bfa065fc937d51cec37958fd5d33d0c934a0da3d27776a33ae",
          "originalTxHash": "0634b9c1db9fd6bfa065fc937d51cec37958fd5d33d0c934a0da3d27776a33ae",
          "gasLimit": 0,
          "gasPrice": 1000000000,
          "callType": 0,
          "operation": "transfer"
        },
        {
          "hash": "3f6f0f3de9e942884e7e1592823a7db7ce935a3f9d3359d8c1ee98a5645332d8",
          "nonce": 0,
          "value": 0,
          "receiver": "erd1qqqqqqqqqqqqqpgq06w7lq7relxyh2h6xzh98q8x24psf3fqssvqn4ptek",
          "sender": "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u",
          "data": "@00@444f5045544553542d373732303063",
          "prevTxHash": "9aecf3bd5dd5c706a28d1cc7059ac20db74340f136816f667dbefcc58daa3aba",
          "originalTxHash": "0634b9c1db9fd6bfa065fc937d51cec37958fd5d33d0c934a0da3d27776a33ae",
          "gasLimit": 25958360,
          "gasPrice": 1000000000,
          "callType": 2,
          "originalSender": "erd1qqqqqqqqqqqqqpgq06w7lq7relxyh2h6xzh98q8x24psf3fqssvqn4ptek",
          "logs": {
            "address": "erd1qqqqqqqqqqqqqpgq06w7lq7relxyh2h6xzh98q8x24psf3fqssvqn4ptek",
            "events": [
              {
                "address": "erd1qqqqqqqqqqqqqpgq06w7lq7relxyh2h6xzh98q8x24psf3fqssvqn4ptek",
                "identifier": "completedTxEvent",
                "topics": [
                  "muzzvV3VxwaijRzHBZrCDbdDQPE2gW9mfb78xY2qOro="
                ],
                "data": null
              }
            ]
          },
          "operation": "transfer"
        },
        {
          "hash": "c6e4f7c5da455009fb4f6967ce8a273a97b826aa617fa798ffd0cf17bde6b97a",
          "nonce": 1,
          "value": 225516180000000,
          "receiver": "erd1qqqqqqqqqqqqqpgq06w7lq7relxyh2h6xzh98q8x24psf3fqssvqn4ptek",
          "sender": "erd1qqqqqqqqqqqqqpgq06w7lq7relxyh2h6xzh98q8x24psf3fqssvqn4ptek",
          "data": "@6f6b",
          "prevTxHash": "3f6f0f3de9e942884e7e1592823a7db7ce935a3f9d3359d8c1ee98a5645332d8",
          "originalTxHash": "0634b9c1db9fd6bfa065fc937d51cec37958fd5d33d0c934a0da3d27776a33ae",
          "gasLimit": 0,
          "gasPrice": 1000000000,
          "callType": 0,
          "operation": "transfer",
          "isRefund": true
        }
      ],
      "logs": {
        "address": "erd1qqqqqqqqqqqqqpgq06w7lq7relxyh2h6xzh98q8x24psf3fqssvqn4ptek",
        "events": [
          {
            "address": "erd1qqqqqqqqqqqqqpgq06w7lq7relxyh2h6xzh98q8x24psf3fqssvqn4ptek",
            "identifier": "transferValueOnly",
            "topics": [
              "AAAAAAAAAAAFAH6d74PDz8xLqvowrlOA5lVDBMUghBg=",
              "AAAAAAAAAAAAAQAAAAAAAAAAAAAAAAAAAAAAAAAC//8=",
              "saK8LsUAAA=="
            ],
            "data": null
          },
          {
            "address": "erd1qqqqqqqqqqqqqpgq06w7lq7relxyh2h6xzh98q8x24psf3fqssvqn4ptek",
            "identifier": "writeLog",
            "topics": [
              "NFD3vy7whM1YgLyLL6nEojWMeUDt2MMJRRM9qTgahBg="
            ],
            "data": "QDZmNmI="
          }
        ]
      },
      "status": "success",
      "operation": "transfer",
      "function": "issueToken",
      "initiallyPaidFee": "914840000000000",
      "fee": "914840000000000",
      "chainID": "1",
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
        let tx_response = TxResponse::from_network_tx(tx_on_network);

        let expected: Option<String> = Some("DOPETEST-77200c".to_string());

        assert_eq!(tx_response.new_issued_token_identifier, expected)
    }

    #[test]
    fn test_process_issued_token_identifier_non_fungible() {
        let data = r#"
        {
  "data": {
    "transaction": {
      "type": "normal",
      "processingTypeOnSource": "SCInvoking",
      "processingTypeOnDestination": "SCInvoking",
      "hash": "d296186b432d7e7937bde37d725cd52b765ef334c00b95adcb079933bc2277bb",
      "nonce": 16,
      "round": 820170,
      "epoch": 341,
      "value": "50000000000000000",
      "receiver": "erd1qqqqqqqqqqqqqpgqmegl4z2rcfhxjv6pn7dm0n4hndllfaam4fdqwqxld8",
      "sender": "erd162knt53z7m0f9jjms9wxphr3q9d7zu4ky85xs2cc0ekrqz7k4fdq85lkuc",
      "gasPrice": 1000000000,
      "gasLimit": 200000000,
      "gasUsed": 200000000,
      "data": "aXNzdWVUb2tlbkA2NzY1NmU2NTdhNzk3M0A0NzQ1NGU=",
      "signature": "e80d45f4de419799a2bbff1cae1235521c8eef1853ee45b02f95c2da74ce50d241bf75b6ab0c650245562700862ea9759caad40f3e381ac0c4d82cfe56e67c09",
      "sourceShard": 2,
      "destinationShard": 2,
      "blockNonce": 819313,
      "blockHash": "a1db4ef13f07b86678000df9cc78f244d83dcc35ae51de545f333bf616930d39",
      "notarizedAtSourceInMetaNonce": 819396,
      "NotarizedAtSourceInMetaHash": "6d9e511e46d318aa5b77cbfdfde14d2ce8515ce4e954b286f130a6b518ddf26a",
      "notarizedAtDestinationInMetaNonce": 819396,
      "notarizedAtDestinationInMetaHash": "6d9e511e46d318aa5b77cbfdfde14d2ce8515ce4e954b286f130a6b518ddf26a",
      "miniblockType": "TxBlock",
      "miniblockHash": "afdb278522181aeb9b12f08840e6c534e398e6af9c7f757548308e300e7ec4e9",
      "hyperblockNonce": 819396,
      "hyperblockHash": "6d9e511e46d318aa5b77cbfdfde14d2ce8515ce4e954b286f130a6b518ddf26a",
      "timestamp": 1698921020,
      "smartContractResults": [
        {
          "hash": "6fe0cc002802af1744f394eee4a69224b5e775961d8386e04e7a5b9242f7ff65",
          "nonce": 0,
          "value": 50000000000000000,
          "receiver": "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u",
          "sender": "erd1qqqqqqqqqqqqqpgqmegl4z2rcfhxjv6pn7dm0n4hndllfaam4fdqwqxld8",
          "data": "issueNonFungible@67656e657a7973@47454e@63616e467265657a65@74727565@63616e57697065@74727565@63616e5061757365@74727565@63616e5472616e736665724e4654437265617465526f6c65@74727565@63616e4368616e67654f776e6572@66616c7365@63616e55706772616465@66616c7365@63616e4164645370656369616c526f6c6573@74727565@5e30e4",
          "prevTxHash": "d296186b432d7e7937bde37d725cd52b765ef334c00b95adcb079933bc2277bb",
          "originalTxHash": "d296186b432d7e7937bde37d725cd52b765ef334c00b95adcb079933bc2277bb",
          "gasLimit": 196098365,
          "gasPrice": 1000000000,
          "callType": 1,
          "originalSender": "erd162knt53z7m0f9jjms9wxphr3q9d7zu4ky85xs2cc0ekrqz7k4fdq85lkuc",
          "operation": "transfer",
          "function": "issueNonFungible"
        },
        {
          "hash": "98afe82512c79f1caaf171bd5919ee469d11ba0c4f725aefcab834278c0f1e58",
          "nonce": 0,
          "value": 0,
          "receiver": "erd1lllllllllllllllllllllllllllllllllllllllllllllllllupq9x7ny0",
          "sender": "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u",
          "data": "ESDTSetBurnRoleForAll@47454e2d383638353933",
          "prevTxHash": "6fe0cc002802af1744f394eee4a69224b5e775961d8386e04e7a5b9242f7ff65",
          "originalTxHash": "d296186b432d7e7937bde37d725cd52b765ef334c00b95adcb079933bc2277bb",
          "gasLimit": 0,
          "gasPrice": 1000000000,
          "callType": 0,
          "logs": {
            "address": "erd1lllllllllllllllllllllllllllllllllllllllllllllllllupq9x7ny0",
            "events": [
              {
                "address": "erd1lllllllllllllllllllllllllllllllllllllllllllllllllupq9x7ny0",
                "identifier": "completedTxEvent",
                "topics": [
                  "b+DMACgCrxdE85Tu5KaSJLXndZYdg4bgTnpbkkL3/2U="
                ],
                "data": null,
                "additionalData": null
              }
            ]
          },
          "operation": "transfer"
        },
        {
          "hash": "83494ad9369738b574a7266cbfb12ce63ccf634950cd6b0ec16107b8fb42f8f6",
          "nonce": 0,
          "value": 0,
          "receiver": "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u",
          "sender": "erd1qqqqqqqqqqqqqpgqmegl4z2rcfhxjv6pn7dm0n4hndllfaam4fdqwqxld8",
          "data": "setSpecialRole@47454e2d383638353933@00000000000000000500de51fa8943c26e6933419f9bb7ceb79b7ff4f7bbaa5a@45534454526f6c654e4654437265617465@5e30e4",
          "prevTxHash": "112d18ec0364b4700b1bfb3df517c80dba547a53373ece2a9e71acd5266e7b64",
          "originalTxHash": "d296186b432d7e7937bde37d725cd52b765ef334c00b95adcb079933bc2277bb",
          "gasLimit": 142399698,
          "gasPrice": 1000000000,
          "callType": 1,
          "operation": "transfer",
          "function": "setSpecialRole"
        },
        {
          "hash": "112d18ec0364b4700b1bfb3df517c80dba547a53373ece2a9e71acd5266e7b64",
          "nonce": 0,
          "value": 0,
          "receiver": "erd1qqqqqqqqqqqqqpgqmegl4z2rcfhxjv6pn7dm0n4hndllfaam4fdqwqxld8",
          "sender": "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u",
          "data": "@00@47454e2d383638353933",
          "prevTxHash": "6fe0cc002802af1744f394eee4a69224b5e775961d8386e04e7a5b9242f7ff65",
          "originalTxHash": "d296186b432d7e7937bde37d725cd52b765ef334c00b95adcb079933bc2277bb",
          "gasLimit": 146098365,
          "gasPrice": 1000000000,
          "callType": 2,
          "originalSender": "erd1qqqqqqqqqqqqqpgqmegl4z2rcfhxjv6pn7dm0n4hndllfaam4fdqwqxld8",
          "logs": {
            "address": "erd1qqqqqqqqqqqqqpgqmegl4z2rcfhxjv6pn7dm0n4hndllfaam4fdqwqxld8",
            "events": [
              {
                "address": "erd1qqqqqqqqqqqqqpgqmegl4z2rcfhxjv6pn7dm0n4hndllfaam4fdqwqxld8",
                "identifier": "writeLog",
                "topics": [
                  "AAAAAAAAAAAFAN5R+olDwm5pM0Gfm7fOt5t/9Pe7qlo="
                ],
                "data": "QDZmNmI=",
                "additionalData": null
              }
            ]
          },
          "operation": "transfer"
        },
        {
          "hash": "db5d74970374337956fa61fb4fd90057b3f6a82ea3e259b389934b71a1652e5f",
          "nonce": 0,
          "value": 0,
          "receiver": "erd1qqqqqqqqqqqqqpgqmegl4z2rcfhxjv6pn7dm0n4hndllfaam4fdqwqxld8",
          "sender": "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u",
          "data": "ESDTSetRole@47454e2d383638353933@45534454526f6c654e4654437265617465",
          "prevTxHash": "83494ad9369738b574a7266cbfb12ce63ccf634950cd6b0ec16107b8fb42f8f6",
          "originalTxHash": "d296186b432d7e7937bde37d725cd52b765ef334c00b95adcb079933bc2277bb",
          "gasLimit": 0,
          "gasPrice": 1000000000,
          "callType": 0,
          "logs": {
            "address": "erd1qqqqqqqqqqqqqpgqmegl4z2rcfhxjv6pn7dm0n4hndllfaam4fdqwqxld8",
            "events": [
              {
                "address": "erd1qqqqqqqqqqqqqpgqmegl4z2rcfhxjv6pn7dm0n4hndllfaam4fdqwqxld8",
                "identifier": "ESDTSetRole",
                "topics": [
                  "R0VOLTg2ODU5Mw==",
                  "",
                  "",
                  "RVNEVFJvbGVORlRDcmVhdGU="
                ],
                "data": null,
                "additionalData": null
              },
              {
                "address": "erd1qqqqqqqqqqqqqpgqmegl4z2rcfhxjv6pn7dm0n4hndllfaam4fdqwqxld8",
                "identifier": "completedTxEvent",
                "topics": [
                  "g0lK2TaXOLV0pyZsv7Es5jzPY0lQzWsOwWEHuPtC+PY="
                ],
                "data": null,
                "additionalData": null
              }
            ]
          },
          "operation": "ESDTSetRole",
          "function": "ESDTSetRole"
        },
        {
          "hash": "a6a665f47977a59c4c2baf460281fc938e04ae0f87ac2e78040a14ae27822701",
          "nonce": 0,
          "value": 0,
          "receiver": "erd1qqqqqqqqqqqqqpgqmegl4z2rcfhxjv6pn7dm0n4hndllfaam4fdqwqxld8",
          "sender": "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u",
          "data": "@00",
          "prevTxHash": "83494ad9369738b574a7266cbfb12ce63ccf634950cd6b0ec16107b8fb42f8f6",
          "originalTxHash": "d296186b432d7e7937bde37d725cd52b765ef334c00b95adcb079933bc2277bb",
          "gasLimit": 92399698,
          "gasPrice": 1000000000,
          "callType": 2,
          "originalSender": "erd1qqqqqqqqqqqqqpgqmegl4z2rcfhxjv6pn7dm0n4hndllfaam4fdqwqxld8",
          "logs": {
            "address": "erd1qqqqqqqqqqqqqpgqmegl4z2rcfhxjv6pn7dm0n4hndllfaam4fdqwqxld8",
            "events": [
              {
                "address": "erd1qqqqqqqqqqqqqpgqmegl4z2rcfhxjv6pn7dm0n4hndllfaam4fdqwqxld8",
                "identifier": "writeLog",
                "topics": [
                  "AAAAAAAAAAAFAN5R+olDwm5pM0Gfm7fOt5t/9Pe7qlo=",
                  "QHRvbyBtdWNoIGdhcyBwcm92aWRlZCBmb3IgcHJvY2Vzc2luZzogZ2FzIHByb3ZpZGVkID0gOTIzOTk2OTgsIGdhcyB1c2VkID0gMzE0MTg4MA=="
                ],
                "data": "QDZmNmI=",
                "additionalData": null
              },
              {
                "address": "erd1qqqqqqqqqqqqqpgqmegl4z2rcfhxjv6pn7dm0n4hndllfaam4fdqwqxld8",
                "identifier": "completedTxEvent",
                "topics": [
                  "g0lK2TaXOLV0pyZsv7Es5jzPY0lQzWsOwWEHuPtC+PY="
                ],
                "data": null,
                "additionalData": null
              }
            ]
          },
          "operation": "transfer"
        }
      ],
      "logs": {
        "address": "erd1qqqqqqqqqqqqqpgqmegl4z2rcfhxjv6pn7dm0n4hndllfaam4fdqwqxld8",
        "events": [
          {
            "address": "erd1qqqqqqqqqqqqqpgqmegl4z2rcfhxjv6pn7dm0n4hndllfaam4fdqwqxld8",
            "identifier": "transferValueOnly",
            "topics": [
              "AAAAAAAAAAAFAN5R+olDwm5pM0Gfm7fOt5t/9Pe7qlo=",
              "AAAAAAAAAAAAAQAAAAAAAAAAAAAAAAAAAAAAAAAC//8=",
              "saK8LsUAAA=="
            ],
            "data": null,
            "additionalData": null
          },
          {
            "address": "erd1qqqqqqqqqqqqqpgqmegl4z2rcfhxjv6pn7dm0n4hndllfaam4fdqwqxld8",
            "identifier": "writeLog",
            "topics": [
              "0q010iL23pLKW4FcYNxxAVvhcrYh6GgrGH5sMAvWqlo="
            ],
            "data": "QDZmNmI=",
            "additionalData": null
          }
        ]
      },
      "status": "success",
      "operation": "transfer",
      "function": "issueToken",
      "initiallyPaidFee": "2097020000000000",
      "fee": "2097020000000000",
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
        let tx_response = TxResponse::from_network_tx(tx_on_network);

        let expected: Option<String> = Some("GEN-868593".to_string());

        assert_eq!(tx_response.new_issued_token_identifier, expected)
    }

    #[test]
    fn test_process_issued_token_identifier_meta_esdt() {
        let data = r#"
        {
  "data": {
    "transaction": {
      "type": "normal",
      "processingTypeOnSource": "SCInvoking",
      "processingTypeOnDestination": "SCInvoking",
      "hash": "408433c5db749f4666bee6a8b599944071bf493c43ff5f01282a74c22ea2ea43",
      "nonce": 419,
      "round": 1787093,
      "epoch": 744,
      "value": "50000000000000000",
      "receiver": "erd1qqqqqqqqqqqqqpgq0628nau8zydgwu96fn8ksqklzhrggkcfq33sm4vmwv",
      "sender": "erd1j6kua7p67qnaw3y4sudmk25xsuv4k8ws6pwvax8fd2vtmuc3q33s840l87",
      "gasPrice": 1000000000,
      "gasLimit": 600000000,
      "gasUsed": 157220928,
      "data": "ZGVwbG95QXNoc3dhcExQQUNTdHJhdGVneUA0MTRjNTAyZDYzNjE2NTYxNjMzNUA0MTU0NTMyZDM0NjMzMDM5MzIzMEAwM2U4QDAzZThAQDNiOWFjYTAwQDAwMDAwMDAwMDAwMDAwMDAwNTAwOTU3MzkwYWVkYTQzMmY1MmE0MTFkNTE5NzRmZTkzZDQwZDI3NzMzZTA0NjNAMDAwMDAwMDAwMDAwMDAwMDA1MDBkMTJjYzczY2JkYTZmMjY1OWM5NTllNWQ1NzU4YWY5MmNhMTM4NDg2NTIzM0AwMDAwMDAwMDAwMDAwMDAwMDUwMDUxZGY3MTc1OGNmMmFjYTViNDZkZWQ4MTU1OGI1NTE1ZGMyOWYzZjM1MjMzQEAwMDAwMDAwMDAwMDAwMDAwMDUwMDdlNGExZGZjNDM3Y2VkNDlkYjlmMTYzNzk4NDE2Yjg0YWMyMWQ0Yzk3Y2ViMDAwMDAwMGM1NzQ1NDc0YzQ0MmQ2MTMyMzg2MzM1MzkwMDAwMDAwMDAwMDAwMDAwMDUwMGE4YmE5ZTY4NjI2YmJjOTkzZmQ3OTVlOGJiNmY0Nzk0M2IyZjVmZmE3Y2ViMDAwMDAwMGE1NTU0NGIyZDMxMzQ2NDM1Mzc2NEAwMDAwMDAwMTAwMDAwMDAwMDAwMDAwMDAwNTAwNTFkZjcxNzU4Y2YyYWNhNWI0NmRlZDgxNTU4YjU1MTVkYzI5ZjNmMzUyMzMwMDAwMDAwYjQyNTU1MzQ0MmQ2NDM0NjMzMDMxMzQwMDAwMDAwMDAwQDAxODZhMEAyNzEw",
      "signature": "4648af0b96eb430e4986b9fb760549742de09c809b46b984e5d995c898d80c25bfc0717c30da34bd89cd3005d98ee895afa39ee588b7b74b4807c63cbeade807",
      "sourceShard": 1,
      "destinationShard": 1,
      "blockNonce": 1785520,
      "blockHash": "8f926a5d79fa84bc69949a21bfbba17447091a8a074ac172fa0b88e4475a1214",
      "notarizedAtSourceInMetaNonce": 1785568,
      "NotarizedAtSourceInMetaHash": "eebd1aa5c3dde083f9c367242c054affedd36bfc95f7bcc1d4e2d90beb5754e9",
      "notarizedAtDestinationInMetaNonce": 1785568,
      "notarizedAtDestinationInMetaHash": "eebd1aa5c3dde083f9c367242c054affedd36bfc95f7bcc1d4e2d90beb5754e9",
      "miniblockType": "TxBlock",
      "miniblockHash": "b85d82db6d69cbc1911b3455d2837eeb3170b391926efa2eacb4d9c8e3c96ee4",
      "hyperblockNonce": 1785568,
      "hyperblockHash": "eebd1aa5c3dde083f9c367242c054affedd36bfc95f7bcc1d4e2d90beb5754e9",
      "timestamp": 1704722558,
      "smartContractResults": [
        {
          "hash": "ea9a96c079e66249e6b73c0341991dad96ca81f855f2fc4abe0d432be117a882",
          "nonce": 420,
          "value": 4427790720000000,
          "receiver": "erd1j6kua7p67qnaw3y4sudmk25xsuv4k8ws6pwvax8fd2vtmuc3q33s840l87",
          "sender": "erd1qqqqqqqqqqqqqpgq0628nau8zydgwu96fn8ksqklzhrggkcfq33sm4vmwv",
          "data": "@6f6b",
          "prevTxHash": "408433c5db749f4666bee6a8b599944071bf493c43ff5f01282a74c22ea2ea43",
          "originalTxHash": "408433c5db749f4666bee6a8b599944071bf493c43ff5f01282a74c22ea2ea43",
          "gasLimit": 0,
          "gasPrice": 1000000000,
          "callType": 0,
          "operation": "transfer",
          "isRefund": true
        },
        {
          "hash": "6082975132a2c9d8197dfd0f9852b454ad344740eebdbdf93f620b2796ab723b",
          "nonce": 0,
          "value": 50000000000000000,
          "receiver": "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u",
          "sender": "erd1qqqqqqqqqqqqqpgq0628nau8zydgwu96fn8ksqklzhrggkcfq33sm4vmwv",
          "data": "registerMetaESDT@415453417368537761704c5041435661756c74@4156415348@12@63616e467265657a65@66616c7365@63616e57697065@66616c7365@63616e5061757365@66616c7365@63616e5472616e736665724e4654437265617465526f6c65@66616c7365@63616e4368616e67654f776e6572@66616c7365@63616e55706772616465@66616c7365@63616e4164645370656369616c526f6c6573@74727565@9eb30a87c92674ab1469700c0b385b3850e86de80f87dec6cf3213c7e379a646@408433c5db749f4666bee6a8b599944071bf493c43ff5f01282a74c22ea2ea43@03eb4a30",
          "prevTxHash": "408433c5db749f4666bee6a8b599944071bf493c43ff5f01282a74c22ea2ea43",
          "originalTxHash": "408433c5db749f4666bee6a8b599944071bf493c43ff5f01282a74c22ea2ea43",
          "gasLimit": 125751600,
          "gasPrice": 1000000000,
          "callType": 1,
          "originalSender": "erd1j6kua7p67qnaw3y4sudmk25xsuv4k8ws6pwvax8fd2vtmuc3q33s840l87",
          "operation": "transfer",
          "function": "registerMetaESDT"
        },
        {
          "hash": "290f85d7ec2f7d5797510290358e9e0f76bb880451efaacb0d69280b8d94c67a",
          "nonce": 0,
          "value": 0,
          "receiver": "erd1llllllllllllllllllllllllllllllllllllllllllllllllluqsl6e366",
          "sender": "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u",
          "data": "ESDTSetBurnRoleForAll@41564153482d376438623564",
          "prevTxHash": "6082975132a2c9d8197dfd0f9852b454ad344740eebdbdf93f620b2796ab723b",
          "originalTxHash": "408433c5db749f4666bee6a8b599944071bf493c43ff5f01282a74c22ea2ea43",
          "gasLimit": 0,
          "gasPrice": 1000000000,
          "callType": 0,
          "originalSender": "erd1j6kua7p67qnaw3y4sudmk25xsuv4k8ws6pwvax8fd2vtmuc3q33s840l87",
          "logs": {
            "address": "erd1llllllllllllllllllllllllllllllllllllllllllllllllluqsl6e366",
            "events": [
              {
                "address": "erd1llllllllllllllllllllllllllllllllllllllllllllllllluqsl6e366",
                "identifier": "completedTxEvent",
                "topics": [
                  "YIKXUTKiydgZff0PmFK0VK00R0Duvb35P2ILJ5arcjs="
                ],
                "data": null,
                "additionalData": null
              }
            ]
          },
          "operation": "transfer"
        },
        {
          "hash": "1aa62a6251edd216bd4e5ae59f7e676d5d2f88597685e0ec0e25ac4434bfccdb",
          "nonce": 0,
          "value": 0,
          "receiver": "erd1qqqqqqqqqqqqqpgq0628nau8zydgwu96fn8ksqklzhrggkcfq33sm4vmwv",
          "sender": "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u",
          "data": "@00@41564153482d376438623564@d0644194444642fd16ee156307f6fda0e8f8baf4c496e1a1dc85e027ecc08a4a@9eb30a87c92674ab1469700c0b385b3850e86de80f87dec6cf3213c7e379a646@408433c5db749f4666bee6a8b599944071bf493c43ff5f01282a74c22ea2ea43@00",
          "prevTxHash": "6082975132a2c9d8197dfd0f9852b454ad344740eebdbdf93f620b2796ab723b",
          "originalTxHash": "408433c5db749f4666bee6a8b599944071bf493c43ff5f01282a74c22ea2ea43",
          "gasLimit": 75751600,
          "gasPrice": 1000000000,
          "callType": 2,
          "originalSender": "erd1j6kua7p67qnaw3y4sudmk25xsuv4k8ws6pwvax8fd2vtmuc3q33s840l87",
          "logs": {
            "address": "erd1qqqqqqqqqqqqqpgq0628nau8zydgwu96fn8ksqklzhrggkcfq33sm4vmwv",
            "events": [
              {
                "address": "erd1qqqqqqqqqqqqqpgq0628nau8zydgwu96fn8ksqklzhrggkcfq33sm4vmwv",
                "identifier": "writeLog",
                "topics": [
                  "AAAAAAAAAAAFAH6UefeHERqHcLpMz2gC3xXGhFsJBGM=",
                  "QHRvbyBtdWNoIGdhcyBwcm92aWRlZCBmb3IgcHJvY2Vzc2luZzogZ2FzIHByb3ZpZGVkID0gNzU3NTE2MDAsIGdhcyB1c2VkID0gNDE3NjA1OQ=="
                ],
                "data": "QDZmNmI=",
                "additionalData": [
                  "QDZmNmI="
                ]
              },
              {
                "address": "erd1qqqqqqqqqqqqqpgq0628nau8zydgwu96fn8ksqklzhrggkcfq33sm4vmwv",
                "identifier": "completedTxEvent",
                "topics": [
                  "YIKXUTKiydgZff0PmFK0VK00R0Duvb35P2ILJ5arcjs="
                ],
                "data": null,
                "additionalData": null
              }
            ]
          },
          "operation": "transfer"
        }
      ],
      "logs": {
        "address": "erd1qqqqqqqqqqqqqpgq0628nau8zydgwu96fn8ksqklzhrggkcfq33sm4vmwv",
        "events": [
          {
            "address": "erd1qqqqqqqqqqqqqpgq0628nau8zydgwu96fn8ksqklzhrggkcfq33sm4vmwv",
            "identifier": "transferValueOnly",
            "topics": [
              "",
              "AAAAAAAAAAAFANAMwOY4h/9reS00I0pE56xrV11LBGM="
            ],
            "data": "RGVwbG95RnJvbVNvdXJjZQ==",
            "additionalData": [
              "RGVwbG95RnJvbVNvdXJjZQ==",
              "aW5pdA==",
              "QUxQLWNhZWFjNQ==",
              "QVRTLTRjMDkyMA==",
              "A+g=",
              "A+g=",
              "",
              "O5rKAA=="
            ]
          },
          {
            "address": "erd1qqqqqqqqqqqqqpgq0628nau8zydgwu96fn8ksqklzhrggkcfq33sm4vmwv",
            "identifier": "transferValueOnly",
            "topics": [
              "",
              "AAAAAAAAAAAFADJ0SE0vUW6bO5SurLeFIMfK/HtBBGM="
            ],
            "data": "RGVwbG95RnJvbVNvdXJjZQ==",
            "additionalData": [
              "RGVwbG95RnJvbVNvdXJjZQ==",
              "aW5pdA==",
              "AAAAAAAAAAAFANAMwOY4h/9reS00I0pE56xrV11LBGM=",
              "AAAAAAAAAAAFAJVzkK7aQy9SpBHVGXT+k9QNJ3M+BGM=",
              "AAAAAAAAAAAFANEsxzy9pvJlnJWeXVdYr5LKE4SGUjM=",
              "AAAAAAAAAAAFAFHfcXWM8qyltG3tgVWLVRXcKfPzUjM=",
              "",
              "AAAAAAAAAAAFAH5KHfxDfO1J258WN5hBa4SsIdTJfOsAAAAMV0VHTEQtYTI4YzU5AAAAAAAAAAAFAKi6nmhia7yZP9eV6LtvR5Q7L1/6fOsAAAAKVVRLLTE0ZDU3ZA==",
              "AAAAAQAAAAAAAAAABQBR33F1jPKspbRt7YFVi1UV3Cnz81IzAAAAC0JVU0QtZDRjMDE0AAAAAAA=",
              "AYag",
              "JxA="
            ]
          },
          {
            "address": "erd1qqqqqqqqqqqqqpgqxf6ysnf029hfkwu546kt0pfqcl90c76pq33s0a320f",
            "identifier": "transferValueOnly",
            "topics": [
              "",
              "AAAAAAAAAAAFANEsxzy9pvJlnJWeXVdYr5LKE4SGUjM="
            ],
            "data": "RXhlY3V0ZU9uRGVzdENvbnRleHQ=",
            "additionalData": [
              "RXhlY3V0ZU9uRGVzdENvbnRleHQ=",
              "Z2V0RmFybWluZ1Rva2VuSWQ="
            ]
          },
          {
            "address": "erd1qqqqqqqqqqqqqpgqxf6ysnf029hfkwu546kt0pfqcl90c76pq33s0a320f",
            "identifier": "transferValueOnly",
            "topics": [
              "",
              "AAAAAAAAAAAFANEsxzy9pvJlnJWeXVdYr5LKE4SGUjM="
            ],
            "data": "RXhlY3V0ZU9uRGVzdENvbnRleHQ=",
            "additionalData": [
              "RXhlY3V0ZU9uRGVzdENvbnRleHQ=",
              "Z2V0RmFybVRva2VuSWQ="
            ]
          },
          {
            "address": "erd1qqqqqqqqqqqqqpgqxf6ysnf029hfkwu546kt0pfqcl90c76pq33s0a320f",
            "identifier": "transferValueOnly",
            "topics": [
              "",
              "AAAAAAAAAAAFANEsxzy9pvJlnJWeXVdYr5LKE4SGUjM="
            ],
            "data": "RXhlY3V0ZU9uRGVzdENvbnRleHQ=",
            "additionalData": [
              "RXhlY3V0ZU9uRGVzdENvbnRleHQ=",
              "Z2V0UmV3YXJkVG9rZW5JZA=="
            ]
          },
          {
            "address": "erd1qqqqqqqqqqqqqpgq0628nau8zydgwu96fn8ksqklzhrggkcfq33sm4vmwv",
            "identifier": "transferValueOnly",
            "topics": [
              "saK8LsUAAA==",
              "AAAAAAAAAAAAAQAAAAAAAAAAAAAAAAAAAAAAAAAC//8="
            ],
            "data": "QXN5bmNDYWxs",
            "additionalData": [
              "QXN5bmNDYWxs",
              "cmVnaXN0ZXJNZXRhRVNEVA==",
              "QVRTQXNoU3dhcExQQUNWYXVsdA==",
              "QVZBU0g=",
              "Eg==",
              "Y2FuRnJlZXpl",
              "ZmFsc2U=",
              "Y2FuV2lwZQ==",
              "ZmFsc2U=",
              "Y2FuUGF1c2U=",
              "ZmFsc2U=",
              "Y2FuVHJhbnNmZXJORlRDcmVhdGVSb2xl",
              "ZmFsc2U=",
              "Y2FuQ2hhbmdlT3duZXI=",
              "ZmFsc2U=",
              "Y2FuVXBncmFkZQ==",
              "ZmFsc2U=",
              "Y2FuQWRkU3BlY2lhbFJvbGVz",
              "dHJ1ZQ=="
            ]
          },
          {
            "address": "erd1qqqqqqqqqqqqqpgqxf6ysnf029hfkwu546kt0pfqcl90c76pq33s0a320f",
            "identifier": "SCDeploy",
            "topics": [
              "AAAAAAAAAAAFADJ0SE0vUW6bO5SurLeFIMfK/HtBBGM=",
              "AAAAAAAAAAAFAH6UefeHERqHcLpMz2gC3xXGhFsJBGM=",
              "fvRqbue54Womde/CN2IkRGkrx8tsU+xkLvi3+uwMkhY="
            ],
            "data": null,
            "additionalData": null
          },
          {
            "address": "erd1qqqqqqqqqqqqqpgq6qxvpe3csllkk7fdxs3553884344wh2tq33sakulat",
            "identifier": "SCDeploy",
            "topics": [
              "AAAAAAAAAAAFANAMwOY4h/9reS00I0pE56xrV11LBGM=",
              "AAAAAAAAAAAFAH6UefeHERqHcLpMz2gC3xXGhFsJBGM=",
              "E3blQfRJfCKLWDr06Od703DSZenIzq8KND+xUjmGY/M="
            ],
            "data": null,
            "additionalData": null
          }
        ]
      },
      "status": "success",
      "operation": "transfer",
      "function": "deployAshswapLPACStrategy",
      "initiallyPaidFee": "6936045000000000",
      "fee": "2508254280000000",
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
        let tx_response = TxResponse::from_network_tx(tx_on_network);

        let expected: Option<String> = Some("AVASH-7d8b5d".to_string());

        assert_eq!(tx_response.new_issued_token_identifier, expected)
    }

    #[test]
    fn test_set_special_roles_should_not_process_issued_token_identifier() {
        let data = r#"
        {
  "data": {
    "transaction": {
      "type": "normal",
      "processingTypeOnSource": "SCInvoking",
      "processingTypeOnDestination": "SCInvoking",
      "hash": "cbb1f866da564a04332297dfc4f637be2e50e62bbf4441bf42247ad429747ce0",
      "nonce": 420,
      "round": 1787109,
      "epoch": 744,
      "value": "0",
      "receiver": "erd1qqqqqqqqqqqqqpgq0628nau8zydgwu96fn8ksqklzhrggkcfq33sm4vmwv",
      "sender": "erd1j6kua7p67qnaw3y4sudmk25xsuv4k8ws6pwvax8fd2vtmuc3q33s840l87",
      "gasPrice": 1000000000,
      "gasLimit": 600000000,
      "gasUsed": 129636807,
      "data": "ZmluaXNoVmF1bHREZXBsb3ltZW50cw==",
      "signature": "dca943ef1a788bfa6cb0e9aa3900b8340e4908075cbfefaa2a66688f6f0c0fed349edb2eb48eec427cd9098822fba875e4d66072fbdb44cb7f4c1a416736e20c",
      "sourceShard": 1,
      "destinationShard": 1,
      "blockNonce": 1785536,
      "blockHash": "93ca539e81612768b67a85b7135f7c104e76bec031a758a6b1782910ae49dd8f",
      "notarizedAtSourceInMetaNonce": 1785584,
      "NotarizedAtSourceInMetaHash": "71d17afe660282bb42de1ea3eec3e3534a179bd32aa1471c2861ce411bf30552",
      "notarizedAtDestinationInMetaNonce": 1785584,
      "notarizedAtDestinationInMetaHash": "71d17afe660282bb42de1ea3eec3e3534a179bd32aa1471c2861ce411bf30552",
      "miniblockType": "TxBlock",
      "miniblockHash": "f8c60565af746e92d2c9c09a92734e5eb8da7e42c67a86854c93b349bfe287eb",
      "hyperblockNonce": 1785584,
      "hyperblockHash": "71d17afe660282bb42de1ea3eec3e3534a179bd32aa1471c2861ce411bf30552",
      "timestamp": 1704722654,
      "smartContractResults": [
        {
          "hash": "c3ce9c364de3823ffae250c2bfb40aaf2b18f771ed4bd37bf788ad83a2c651f3",
          "nonce": 421,
          "value": 4703631930000000,
          "receiver": "erd1j6kua7p67qnaw3y4sudmk25xsuv4k8ws6pwvax8fd2vtmuc3q33s840l87",
          "sender": "erd1qqqqqqqqqqqqqpgq0628nau8zydgwu96fn8ksqklzhrggkcfq33sm4vmwv",
          "data": "@6f6b",
          "prevTxHash": "cbb1f866da564a04332297dfc4f637be2e50e62bbf4441bf42247ad429747ce0",
          "originalTxHash": "cbb1f866da564a04332297dfc4f637be2e50e62bbf4441bf42247ad429747ce0",
          "gasLimit": 0,
          "gasPrice": 1000000000,
          "callType": 0,
          "operation": "transfer",
          "isRefund": true
        },
        {
          "hash": "50f9c25a1402ce6d87ae9f890659c8a67462292e471e02c74d64ff7ba1995e60",
          "nonce": 0,
          "value": 0,
          "receiver": "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u",
          "sender": "erd1qqqqqqqqqqqqqpgq0628nau8zydgwu96fn8ksqklzhrggkcfq33sm4vmwv",
          "data": "setSpecialRole@41564153482d376438623564@00000000000000000500d00cc0e63887ff6b792d34234a44e7ac6b575d4b0463@45534454526f6c654e4654437265617465@45534454526f6c654e46544164645175616e74697479@45534454526f6c654e46544275726e@0192c6db2c69f50b6968fb22ac558337a851719519cfd1e6bbf79a07bbcf18bc@cbb1f866da564a04332297dfc4f637be2e50e62bbf4441bf42247ad429747ce0@03eb4a30",
          "prevTxHash": "cbb1f866da564a04332297dfc4f637be2e50e62bbf4441bf42247ad429747ce0",
          "originalTxHash": "cbb1f866da564a04332297dfc4f637be2e50e62bbf4441bf42247ad429747ce0",
          "gasLimit": 125751600,
          "gasPrice": 1000000000,
          "callType": 1,
          "originalSender": "erd1j6kua7p67qnaw3y4sudmk25xsuv4k8ws6pwvax8fd2vtmuc3q33s840l87",
          "operation": "transfer",
          "function": "setSpecialRole"
        },
        {
          "hash": "d6a5824a60b6c9050462c3f5a02ace00c36e8b4ba1958d132bd394e2ed1e7226",
          "nonce": 0,
          "value": 0,
          "receiver": "erd1qqqqqqqqqqqqqpgq6qxvpe3csllkk7fdxs3553884344wh2tq33sakulat",
          "sender": "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u",
          "data": "ESDTSetRole@41564153482d376438623564@45534454526f6c654e4654437265617465@45534454526f6c654e46544164645175616e74697479@45534454526f6c654e46544275726e",
          "prevTxHash": "50f9c25a1402ce6d87ae9f890659c8a67462292e471e02c74d64ff7ba1995e60",
          "originalTxHash": "cbb1f866da564a04332297dfc4f637be2e50e62bbf4441bf42247ad429747ce0",
          "gasLimit": 0,
          "gasPrice": 1000000000,
          "callType": 0,
          "originalSender": "erd1j6kua7p67qnaw3y4sudmk25xsuv4k8ws6pwvax8fd2vtmuc3q33s840l87",
          "logs": {
            "address": "erd1qqqqqqqqqqqqqpgq6qxvpe3csllkk7fdxs3553884344wh2tq33sakulat",
            "events": [
              {
                "address": "erd1qqqqqqqqqqqqqpgq6qxvpe3csllkk7fdxs3553884344wh2tq33sakulat",
                "identifier": "ESDTSetRole",
                "topics": [
                  "QVZBU0gtN2Q4YjVk",
                  "",
                  "",
                  "RVNEVFJvbGVORlRDcmVhdGU=",
                  "RVNEVFJvbGVORlRBZGRRdWFudGl0eQ==",
                  "RVNEVFJvbGVORlRCdXJu"
                ],
                "data": null,
                "additionalData": null
              },
              {
                "address": "erd1qqqqqqqqqqqqqpgq6qxvpe3csllkk7fdxs3553884344wh2tq33sakulat",
                "identifier": "completedTxEvent",
                "topics": [
                  "UPnCWhQCzm2Hrp+JBlnIpnRiKS5HHgLHTWT/e6GZXmA="
                ],
                "data": null,
                "additionalData": null
              }
            ]
          },
          "operation": "ESDTSetRole",
          "function": "ESDTSetRole"
        },
        {
          "hash": "bf1b8b4b301ff548368dfd972896489d5e2a088d5cbdfa1bfe2421cc7f641f7a",
          "nonce": 0,
          "value": 0,
          "receiver": "erd1qqqqqqqqqqqqqpgq0628nau8zydgwu96fn8ksqklzhrggkcfq33sm4vmwv",
          "sender": "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u",
          "data": "@00@a68d44c751eba85db0713db8dc9c10c78749189ec0d6f1af5fc67bb656c1254b@0192c6db2c69f50b6968fb22ac558337a851719519cfd1e6bbf79a07bbcf18bc@cbb1f866da564a04332297dfc4f637be2e50e62bbf4441bf42247ad429747ce0@00",
          "prevTxHash": "50f9c25a1402ce6d87ae9f890659c8a67462292e471e02c74d64ff7ba1995e60",
          "originalTxHash": "cbb1f866da564a04332297dfc4f637be2e50e62bbf4441bf42247ad429747ce0",
          "gasLimit": 75751600,
          "gasPrice": 1000000000,
          "callType": 2,
          "originalSender": "erd1j6kua7p67qnaw3y4sudmk25xsuv4k8ws6pwvax8fd2vtmuc3q33s840l87",
          "logs": {
            "address": "erd1qqqqqqqqqqqqqpgq0628nau8zydgwu96fn8ksqklzhrggkcfq33sm4vmwv",
            "events": [
              {
                "address": "erd1qqqqqqqqqqqqqpgq0628nau8zydgwu96fn8ksqklzhrggkcfq33sm4vmwv",
                "identifier": "transferValueOnly",
                "topics": [
                  "",
                  "AAAAAAAAAAAFANAMwOY4h/9reS00I0pE56xrV11LBGM="
                ],
                "data": "RXhlY3V0ZU9uRGVzdENvbnRleHQ=",
                "additionalData": [
                  "RXhlY3V0ZU9uRGVzdENvbnRleHQ=",
                  "c2V0U2hhcmVUb2tlbklkZW50aWZpZXI=",
                  "QVZBU0gtN2Q4YjVk"
                ]
              },
              {
                "address": "erd1qqqqqqqqqqqqqpgq0628nau8zydgwu96fn8ksqklzhrggkcfq33sm4vmwv",
                "identifier": "transferValueOnly",
                "topics": [
                  "",
                  "AAAAAAAAAAAFANAMwOY4h/9reS00I0pE56xrV11LBGM="
                ],
                "data": "RXhlY3V0ZU9uRGVzdENvbnRleHQ=",
                "additionalData": [
                  "RXhlY3V0ZU9uRGVzdENvbnRleHQ=",
                  "c2V0U3RyYXRlZ3lBZGRyZXNz",
                  "AAAAAAAAAAAFADJ0SE0vUW6bO5SurLeFIMfK/HtBBGM="
                ]
              },
              {
                "address": "erd1qqqqqqqqqqqqqpgq0628nau8zydgwu96fn8ksqklzhrggkcfq33sm4vmwv",
                "identifier": "completedTxEvent",
                "topics": [
                  "UPnCWhQCzm2Hrp+JBlnIpnRiKS5HHgLHTWT/e6GZXmA="
                ],
                "data": null,
                "additionalData": null
              }
            ]
          },
          "operation": "transfer"
        },
        {
          "hash": "9d75a398545f488d4764149245e6ec3101debfce99477c353ac11c3239acd897",
          "nonce": 1,
          "value": 648519550000000,
          "receiver": "erd1qqqqqqqqqqqqqpgq0628nau8zydgwu96fn8ksqklzhrggkcfq33sm4vmwv",
          "sender": "erd1qqqqqqqqqqqqqpgq0628nau8zydgwu96fn8ksqklzhrggkcfq33sm4vmwv",
          "data": "@6f6b",
          "prevTxHash": "bf1b8b4b301ff548368dfd972896489d5e2a088d5cbdfa1bfe2421cc7f641f7a",
          "originalTxHash": "cbb1f866da564a04332297dfc4f637be2e50e62bbf4441bf42247ad429747ce0",
          "gasLimit": 0,
          "gasPrice": 1000000000,
          "callType": 0,
          "operation": "transfer",
          "isRefund": true
        }
      ],
      "logs": {
        "address": "erd1qqqqqqqqqqqqqpgq0628nau8zydgwu96fn8ksqklzhrggkcfq33sm4vmwv",
        "events": [
          {
            "address": "erd1qqqqqqqqqqqqqpgq0628nau8zydgwu96fn8ksqklzhrggkcfq33sm4vmwv",
            "identifier": "transferValueOnly",
            "topics": [
              "",
              "AAAAAAAAAAAAAQAAAAAAAAAAAAAAAAAAAAAAAAAC//8="
            ],
            "data": "QXN5bmNDYWxs",
            "additionalData": [
              "QXN5bmNDYWxs",
              "c2V0U3BlY2lhbFJvbGU=",
              "QVZBU0gtN2Q4YjVk",
              "AAAAAAAAAAAFANAMwOY4h/9reS00I0pE56xrV11LBGM=",
              "RVNEVFJvbGVORlRDcmVhdGU=",
              "RVNEVFJvbGVORlRBZGRRdWFudGl0eQ==",
              "RVNEVFJvbGVORlRCdXJu"
            ]
          }
        ]
      },
      "status": "success",
      "operation": "transfer",
      "function": "finishVaultDeployments",
      "initiallyPaidFee": "6082170000000000",
      "fee": "1378538070000000",
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
        let tx_response = TxResponse::from_network_tx(tx_on_network);

        let expected: Option<String> = None;

        assert_eq!(tx_response.new_issued_token_identifier, expected)
    }
}
