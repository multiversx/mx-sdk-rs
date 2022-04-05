use std::rc::Rc;

use elrond_wasm::types::heap::Address;
use mandos::model::{ScDeployStep, Step};

use crate::{
    tx_execution::sc_create,
    tx_mock::{generate_tx_hash_dummy, TxInput},
    world_mock::BlockchainMock,
};

use super::check_tx_output;

impl BlockchainMock {
    pub fn mandos_sc_deploy(&mut self, sc_deploy_step: ScDeployStep) -> &mut Self {
        self.with_borrowed_rc(|rc| {
            execute_rc(rc, &sc_deploy_step);
        });
        self.mandos_trace.steps.push(Step::ScDeploy(sc_deploy_step));
        self
    }
}

fn execute_rc(state: &mut Rc<BlockchainMock>, sc_deploy_step: &ScDeployStep) {
    let tx = &sc_deploy_step.tx;
    let tx_input = TxInput {
        from: tx.from.value.into(),
        to: Address::zero(),
        egld_value: tx.egld_value.value.clone(),
        esdt_values: Vec::new(),
        func_name: b"init".to_vec(),
        args: tx
            .arguments
            .iter()
            .map(|scen_arg| scen_arg.value.clone())
            .collect(),
        gas_limit: tx.gas_limit.value,
        gas_price: tx.gas_price.value,
        tx_hash: generate_tx_hash_dummy(&sc_deploy_step.tx_id),
    };
    let tx_result = sc_create(tx_input, &tx.contract_code.value, state);
    if let Some(tx_expect) = &sc_deploy_step.expect {
        check_tx_output(&sc_deploy_step.tx_id, tx_expect, &tx_result);
    }
}
