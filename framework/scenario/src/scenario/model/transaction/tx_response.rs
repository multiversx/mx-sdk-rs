use core::panic;

use crate::multiversx_sc::types::Address;
use multiversx_chain_vm::tx_mock::TxResult;
use multiversx_sdk::data::transaction::{
    ApiLogs, ApiSmartContractResult, Events, TransactionOnNetwork,
};

use super::{decode_scr_data_or_panic, process_topics_error, Log, TxResponseStatus};

const LOG_IDENTIFIER_SC_DEPLOY: &str = "SCDeploy";
const LOG_IDENTIFIER_SIGNAL_ERROR: &str = "signalError";

const SYSTEM_SC_BECH32: &str = "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u";

#[derive(Debug, Default, Clone)]
pub struct TxResponse {
    pub out: Vec<Vec<u8>>,
    pub new_deployed_address: Option<Address>,
    pub new_issued_token_identifier: Option<String>,
    pub tx_error: TxResponseStatus,
    pub logs: Vec<Log>,
    pub gas: u64,
    pub refund: u64,
    pub api_scrs: Vec<ApiSmartContractResult>,
    pub api_logs: Option<ApiLogs>,
}

impl TxResponse {
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

    // Finds api logs matching the given log identifier.
    fn find_log(&self, log_identifier: &str) -> Option<&Events> {
        if let Some(logs) = &self.api_logs {
            logs.events
                .iter()
                .find(|event| event.identifier == log_identifier)
        } else {
            None
        }
    }

    fn process(self) -> Self {
        self.process_out()
            .process_new_deployed_address()
            .process_new_issued_token_identifier()
    }

    fn process_out(mut self) -> Self {
        if let Some(first_scr) = self.api_scrs.get(0) {
            self.out = decode_scr_data_or_panic(first_scr.data.as_str());
        } else {
            panic!("no smart contract results obtained");
        }

        self
    }

    fn process_new_deployed_address(mut self) -> Self {
        if let Some(event) = self.find_log(LOG_IDENTIFIER_SC_DEPLOY).cloned() {
            let topics = event.topics.as_ref();
            if let Some(error) = process_topics_error(topics) {
                panic!("{error}");
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
            panic!("no token identifier found in SCR");
        }

        self.new_issued_token_identifier =
            Some(String::from_utf8(hex::decode(encoded_tid.unwrap()).unwrap()).unwrap());
        self
    }

    #[deprecated(
        note = "used for consistency, will be removed soon"
    )]
    pub fn handle_signal_error_event(&self) -> Result<(), TxResponseStatus> {
        if !self.tx_error.is_success() {
            Err(self.tx_error.clone())
        } else {
            Ok(())
        }
    }

    #[deprecated(
        note = "used for consistency, will be removed soon"
    )]
    pub fn new_deployed_address(&self) -> Result<Address, TxResponseStatus> {
        if !self.tx_error.is_success() {
            Err(self.tx_error.clone())
        } else {
            Ok(self.new_deployed_address.clone().unwrap())
        }
    }

    #[deprecated(
        note = "used for consistency, will be removed soon"
    )]
    pub fn issue_non_fungible_new_token_identifier(&self) -> Result<String, TxResponseStatus> {
        if !self.tx_error.is_success() {
            Err(self.tx_error.clone())
        } else {
            Ok(self.new_issued_token_identifier.clone().unwrap())
        }
    }
}
