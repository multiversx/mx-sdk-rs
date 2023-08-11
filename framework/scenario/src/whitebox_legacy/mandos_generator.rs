use std::collections::BTreeMap;

use crate::{
    scenario_format::serde_raw::StepRaw,
    scenario_model::{Scenario, Step},
};
use multiversx_chain_scenario_format::interpret_trait::{InterpretableFrom, InterpreterContext};

use super::{raw_converter::*, ScCallMandos, ScQueryMandos, TxExpectMandos};
use multiversx_chain_vm::world_mock::AccountData;

pub(crate) struct MandosGenerator<'a> {
    scenario: &'a mut Scenario,
    current_tx_id: &'a mut u64,
}

impl<'a> MandosGenerator<'a> {
    pub fn new(scenario: &'a mut Scenario, current_tx_id: &'a mut u64) -> Self {
        Self {
            scenario,
            current_tx_id,
        }
    }

    fn add_step(&mut self, step_raw: StepRaw) {
        self.scenario.steps.push(Step::interpret_from(
            step_raw,
            &InterpreterContext::default().with_allowed_missing_files(),
        ));
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
            new_token_identifiers: Vec::new(),
            comment: None,
            current_block_info: None,
            previous_block_info: None,
        };
        self.add_step(step);
    }

    pub fn next_tx_id_string(&mut self) -> String {
        let id_str = self.current_tx_id.to_string();
        *self.current_tx_id += 1;

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
