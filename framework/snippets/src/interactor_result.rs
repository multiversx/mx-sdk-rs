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

    pub fn new_deployed_address(&self) -> Address {
        let event = self
            .find_log(LOG_IDENTIFIER_SC_DEPLOY)
            .expect("SCDeploy event log not found");
        let topics = event.topics.as_ref().expect("missing topics");
        assert_eq!(topics.len(), 2, "`SCDeploy` is expected to have 2 topics");
        let address_raw = base64::decode(topics.get(0).unwrap()).unwrap();
        let address = Address::from_slice(address_raw.as_slice());
        info!("new address: {}", bech32::encode(&address));
        address
    }

    pub fn issue_non_fungible_new_token_identifier(&self) -> String {
        let second_scr = self
            .scrs
            .get(1)
            .expect("no second smart contract results obtained");

        // TODO: error handling
        let mut split = second_scr.data.split('@');
        let _ = split.next().unwrap();
        let _ = split.next().unwrap();
        let encoded_tid = split
            .next()
            .unwrap_or_else(|| panic!("bad issue token SCR data: {}", second_scr.data));
        String::from_utf8(hex::decode(encoded_tid).unwrap()).unwrap()
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
