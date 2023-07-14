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
        }

        self
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
