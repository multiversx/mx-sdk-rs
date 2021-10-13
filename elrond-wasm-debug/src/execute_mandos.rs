#![allow(unused_variables)] // for now

use crate::{mandos_step, world_mock::BlockchainMock, ContractMap, DebugApi};

use mandos::model::Step;
use std::path::Path;

/// Runs mandos test using the Rust infrastructure and the debug mode.
/// Uses a contract map to replace the references to the wasm bytecode
/// with the contracts running in debug mode.
pub fn mandos_rs<P: AsRef<Path>>(relative_path: P, contract_map: &ContractMap<DebugApi>) {
    let mut absolute_path = std::env::current_dir().unwrap();
    absolute_path.push(relative_path);
    let mut state = BlockchainMock::new();
    parse_execute_mandos_steps(absolute_path.as_ref(), &mut state, contract_map);
}

fn parse_execute_mandos_steps(
    steps_path: &Path,
    state: &mut BlockchainMock,
    contract_map: &ContractMap<DebugApi>,
) {
    let scenario = mandos::parse_scenario(steps_path);

    for step in scenario.steps.iter() {
        match step {
            Step::ExternalSteps { path } => {
                let parent_path = steps_path.parent().unwrap();
                let new_path = parent_path.join(path);
                parse_execute_mandos_steps(new_path.as_path(), state, contract_map);
            },
            Step::SetState {
                comment,
                accounts,
                new_addresses,
                block_hashes,
                previous_block_info,
                current_block_info,
            } => mandos_step::set_state::execute(
                state,
                accounts,
                new_addresses,
                previous_block_info,
                current_block_info,
            ),
            Step::ScCall {
                tx_id,
                comment,
                tx,
                expect,
            } => mandos_step::sc_call::execute(state, contract_map, tx_id, tx, expect),
            Step::ScQuery {
                tx_id,
                comment,
                tx,
                expect,
            } => mandos_step::sc_query::execute(state, contract_map, tx_id, tx, expect),
            Step::ScDeploy {
                tx_id,
                comment,
                tx,
                expect,
            } => mandos_step::sc_deploy::execute(state, contract_map, tx_id, tx, expect),
            Step::Transfer { tx_id, comment, tx } => mandos_step::transfer::execute(state, tx),
            Step::ValidatorReward { tx_id, comment, tx } => {
                state.increase_validator_reward(&tx.to.value.into(), &tx.egld_value.value);
            },
            Step::CheckState { comment, accounts } => {
                mandos_step::check_state::execute(accounts, state);
            },
            Step::DumpState { .. } => {
                state.print_accounts();
            },
        }
    }
}
