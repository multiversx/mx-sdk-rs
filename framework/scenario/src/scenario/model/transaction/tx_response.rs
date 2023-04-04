use std::error::Error;

use crate::{
    bech32,
    multiversx_sc::types::Address,
    scenario_model::{BytesValue, U64Value},
};
use log::info;
use multiversx_sdk::data::transaction::{
    ApiLogs, ApiSmartContractResult, Events, TransactionOnNetwork,
};

use super::Log;

const LOG_IDENTIFIER_SC_DEPLOY: &str = "SCDeploy";
const LOG_IDENTIFIER_SIGNAL_ERROR: &str = "signalError";

#[derive(Debug)]
pub struct TxError {
    pub message: String,
}

impl std::fmt::Display for TxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "transaction error: {}", self.message)
    }
}

impl Error for TxError {}

#[derive(Debug, Default, Clone)]
pub struct TxResponse {
    pub out: Vec<BytesValue>,
    pub status: U64Value,
    pub message: BytesValue,
    pub logs: Vec<Log>,
    pub gas: U64Value,
    pub refund: U64Value,
    pub api_scrs: Vec<ApiSmartContractResult>,
    pub api_logs: Option<ApiLogs>,
}

impl TxResponse {
    pub fn new(tx: TransactionOnNetwork) -> Self {
        Self {
            api_scrs: tx.smart_contract_results.unwrap_or_default(),
            api_logs: tx.logs,
            ..Default::default()
        }
    }

    pub fn raw_result(&self) -> Result<Vec<Vec<u8>>, TxError> {
        self.handle_signal_error_event()?;

        let first_scr = self.api_scrs.get(0);
        if first_scr.is_none() {
            return Err(TxError {
                message: "no smart contract results obtained".to_string(),
            });
        }

        Ok(decode_scr_data_or_panic(first_scr.unwrap().data.as_str()))
    }

    pub fn new_deployed_address(&self) -> Result<Address, TxError> {
        self.handle_signal_error_event()?;
        self.handle_sc_deploy_event()
    }

    // Returns the token identifier of the newly issued non-fungible token.
    pub fn issue_non_fungible_new_token_identifier(&self) -> Result<String, TxError> {
        self.handle_signal_error_event()?;

        let second_scr = self
            .api_scrs
            .iter()
            .find(|scr| scr.data.starts_with("@00@"));
        if second_scr.is_none() {
            return Err(TxError {
                message: "no token identifier SCR found".to_string(),
            });
        }

        let second_scr = second_scr.unwrap();
        let encoded_tid = second_scr.data.split('@').nth(2);
        if encoded_tid.is_none() {
            return Err(TxError {
                message: format!("bad issue token SCR data: {}", second_scr.data),
            });
        }

        Ok(String::from_utf8(hex::decode(encoded_tid.unwrap()).unwrap()).unwrap())
    }

    // Finds api logs matching the given log identifier.
    pub fn find_log(&self, log_identifier: &str) -> Option<&Events> {
        if let Some(logs) = &self.api_logs {
            logs.events
                .iter()
                .find(|event| event.identifier == log_identifier)
        } else {
            None
        }
    }

    // Handles a signalError event
    pub fn handle_signal_error_event(&self) -> Result<(), TxError> {
        if let Some(event) = self.find_log(LOG_IDENTIFIER_SIGNAL_ERROR) {
            let topics = self.handle_event_topics(event, LOG_IDENTIFIER_SIGNAL_ERROR)?;
            let error_raw = base64::decode(topics.get(1).unwrap()).unwrap();
            let error = String::from_utf8(error_raw).unwrap();

            return Err(TxError { message: error });
        }
        Ok(())
    }

    // Handles a scDeploy event
    fn handle_sc_deploy_event(&self) -> Result<Address, TxError> {
        let event = self.find_log(LOG_IDENTIFIER_SC_DEPLOY);
        if event.is_none() {
            return Err(TxError {
                message: format!("`{LOG_IDENTIFIER_SC_DEPLOY}` event not found"),
            });
        }
        let topics = self.handle_event_topics(event.unwrap(), LOG_IDENTIFIER_SC_DEPLOY)?;
        let address_raw = base64::decode(topics.get(0).unwrap()).unwrap();
        let address = Address::from_slice(address_raw.as_slice());

        info!("new address: {}", bech32::encode(&address));
        Ok(address)
    }

    // Handles the topics of an event and returns them.
    fn handle_event_topics<'a, 'b: 'a>(
        &'a self,
        event: &'b Events,
        log_identifier: &str,
    ) -> Result<&Vec<String>, TxError> {
        let option = event.topics.as_ref();
        if option.is_none() {
            return Err(TxError {
                message: "missing topics".to_string(),
            });
        }

        let topics = option.unwrap();
        if topics.len() != 2 {
            return Err(TxError {
                message: format!(
                    "`{log_identifier}` is expected to have 2 topics, found {}",
                    topics.len()
                ),
            });
        }
        Ok(topics)
    }
}

fn decode_scr_data_or_panic(data: &str) -> Vec<Vec<u8>> {
    let mut split = data.split('@');
    let _ = split.next().expect("SCR data should start with '@'");
    let result_code = split.next().expect("missing result code");
    assert_eq!(result_code, "6f6b", "result code is not 'ok'");

    split
        .map(|encoded_arg| hex::decode(encoded_arg).expect("error hex-decoding result"))
        .collect()
}
