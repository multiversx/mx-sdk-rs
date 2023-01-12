use std::{collections::BTreeMap, fs::File, io::Write};

use crate::scenario_format::serde_raw::{ScenarioRaw, StepRaw};
use serde::Serialize;

use super::{raw_converter::*, ScCallMandos, ScQueryMandos, TxExpectMandos};
use multiversx_chain_vm::world_mock::{AccountData, BlockInfo};

pub(crate) struct MandosGenerator {
    scenario: ScenarioRaw,
    current_tx_id: u64,
}

impl MandosGenerator {
    pub fn new() -> Self {
        Self {
            scenario: ScenarioRaw {
                check_gas: None,
                comment: None,
                gas_schedule: None,
                name: None,
                steps: Vec::new(),
            },
            current_tx_id: 0,
        }
    }

    pub fn write_mandos_output(self, file_path: &str) {
        let buf = Vec::new();
        let formatter = serde_json::ser::PrettyFormatter::with_indent(b"    ");
        let mut ser = serde_json::Serializer::with_formatter(buf, formatter);
        self.scenario.serialize(&mut ser).unwrap();
        let mut serialized = String::from_utf8(ser.into_inner()).unwrap();
        serialized.push('\n');

        let mut file = File::create(file_path).unwrap();
        file.write_all(serialized.as_bytes()).unwrap();
    }
}

impl MandosGenerator {
    fn add_step(&mut self, step: StepRaw) {
        self.scenario.steps.push(step);
    }

    pub fn set_account(&mut self, acc: &AccountData, sc_scenario_path_expr: Option<Vec<u8>>) {
        let mut accounts_raw = BTreeMap::new();

        let addr_as_str = bytes_to_hex(acc.address.as_bytes());
        let mut acc_clone = acc.clone();
        acc_clone.contract_path = sc_scenario_path_expr;

        let acc_raw = account_as_raw(&acc_clone);
        accounts_raw.insert(addr_as_str, acc_raw);

        let step = StepRaw::SetState {
            accounts: accounts_raw,
            block_hashes: Vec::new(),
            new_addresses: Vec::new(),
            comment: None,
            current_block_info: None,
            previous_block_info: None,
        };
        self.add_step(step);
    }

    pub fn set_block_info(&mut self, current_block_info: &BlockInfo, prev_block_info: &BlockInfo) {
        let current_raw = block_info_as_raw(current_block_info);
        let prev_raw = block_info_as_raw(prev_block_info);

        let step = StepRaw::SetState {
            accounts: BTreeMap::new(),
            block_hashes: Vec::new(),
            new_addresses: Vec::new(),
            comment: None,
            current_block_info: Some(current_raw),
            previous_block_info: Some(prev_raw),
        };
        self.add_step(step);
    }

    pub fn next_tx_id_string(&mut self) -> String {
        let id_str = self.current_tx_id.to_string();
        self.current_tx_id += 1;

        id_str
    }

    pub fn create_tx(&mut self, tx: &ScCallMandos, opt_expect: Option<&TxExpectMandos>) {
        let tx_raw = tx_call_as_raw(tx);
        let expect_raw = opt_expect.map(tx_expect_as_raw);

        let step = StepRaw::ScCall {
            comment: None,
            display_logs: None,
            id: self.next_tx_id_string(),
            tx: tx_raw,
            expect: expect_raw,
            tx_id: None,
        };
        self.add_step(step);
    }

    pub fn create_query(&mut self, query: &ScQueryMandos, opt_expect: Option<&TxExpectMandos>) {
        let query_raw = tx_query_as_raw(query);
        let expect_raw = opt_expect.map(tx_expect_as_raw);

        let step = StepRaw::ScQuery {
            comment: None,
            display_logs: None,
            id: self.next_tx_id_string(),
            tx_id: None,
            tx: query_raw,
            expect: expect_raw,
        };
        self.add_step(step);
    }

    pub fn check_account(&mut self, acc: &AccountData) {
        let check_raw = account_as_check_state_raw(acc);

        let step = StepRaw::CheckState {
            accounts: check_raw,
            comment: None,
        };
        self.add_step(step);
    }
}
