use std::marker::PhantomData;

use log::info;
use multiversx_sc_scenario::{
    bech32,
    multiversx_sc::{
        codec::{PanicErrorHandler, TopDecodeMulti},
        types::Address,
    },
};
use multiversx_sdk::data::transaction::{
    ApiLogs, ApiSmartContractResult, Events, TransactionOnNetwork,
};

const LOG_IDENTIFIER_SC_DEPLOY: &str = "SCDeploy";
const LOG_IDENTIFIER_SIGNAL_ERROR: &str = "signalError";

pub struct InteractorResult<T: TopDecodeMulti> {
    pub scrs: Vec<ApiSmartContractResult>,
    pub logs: Option<ApiLogs>,
    _phantom: PhantomData<T>,
}

impl<T: TopDecodeMulti> InteractorResult<T> {
    pub fn new(tx: TransactionOnNetwork) -> Self {
        Self {
            logs: tx.logs,
            scrs: tx.smart_contract_results.unwrap_or_default(),
            _phantom: PhantomData,
        }
    }

    pub fn value(&self) -> T {
        let first_scr = self
            .scrs
            .get(0)
            .expect("no smart contract results obtained");

        let mut raw_result = decode_scr_data_or_panic(first_scr.data.as_str());
        T::multi_decode_or_handle_err(&mut raw_result, PanicErrorHandler).unwrap()
    }

    pub fn find_log(&self, log_identifier: &str) -> Option<&Events> {
        if let Some(logs) = &self.logs {
            logs.events
                .iter()
                .find(|event| event.identifier == log_identifier)
        } else {
            None
        }
    }

    // Returns the address of the newly deployed smart contract.
    pub fn new_deployed_address(&self) -> Address {
        self.handle_signal_error_event();

        let topics = self.handle_sc_deploy_event();
        let address_raw = base64::decode(topics.get(0).unwrap()).unwrap();
        let address = Address::from_slice(address_raw.as_slice());

        info!("new address: {}", bech32::encode(&address));
        address
    }

    // Returns the token identifier of the newly issued non-fungible token.
    pub fn issue_non_fungible_new_token_identifier(&self) -> String {
        self.handle_signal_error_event();

        let second_scr = self
            .scrs
            .iter()
            .find(|scr| scr.data.starts_with("@00@"))
            .unwrap_or_else(|| panic!("no token identifier SCR found"));
        let encoded_tid = second_scr
            .data
            .split('@')
            .nth(3)
            .unwrap_or_else(|| panic!("bad issue token SCR data: {}", second_scr.data));

        String::from_utf8(hex::decode(encoded_tid).unwrap()).unwrap()
    }

    // Handles a signalError event, if present.
    fn handle_signal_error_event(&self) {
        if let Some(event) = self.find_log(LOG_IDENTIFIER_SIGNAL_ERROR) {
            let topics = event
                .topics
                .as_ref()
                .unwrap_or_else(|| panic!("missing topics"));
            assert_eq!(
                topics.len(),
                2,
                "`{LOG_IDENTIFIER_SIGNAL_ERROR}` is expected to have 2 topics"
            );
            let error_raw = base64::decode(topics.get(1).unwrap()).unwrap();
            let error = String::from_utf8(error_raw).unwrap();
            panic!("error: {error:#?}");
        }
    }

    // Handles a scDeploy event, if present and returns topics.
    fn handle_sc_deploy_event(&self) -> &Vec<String> {
        let event = self
            .find_log(LOG_IDENTIFIER_SC_DEPLOY)
            .unwrap_or_else(|| panic!("`{LOG_IDENTIFIER_SC_DEPLOY}` event log not found"));
        let topics = event
            .topics
            .as_ref()
            .unwrap_or_else(|| panic!("missing topics"));
        assert_eq!(
            topics.len(),
            2,
            "`{LOG_IDENTIFIER_SC_DEPLOY}` is expected to have 2 topics"
        );
        topics
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
