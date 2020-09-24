#![allow(unused_variables)] // for now

use super::*;
use elrond_wasm::*;
use mandos_rs::*;
use std::path::Path;

pub fn parse_execute_mandos<P: AsRef<Path>>(mock_ref: &ArwenMockRef, path: P) {
    let scenario = mandos_rs::parse_scenario(path);
    execute_mandos_scenario(mock_ref, scenario);
}

pub fn execute_mandos_scenario(mock_ref: &ArwenMockRef, scenario: Scenario) {
    for step in scenario.steps.iter() {
        match step {
            Step::ExternalSteps {
                path,
            } => {},
            Step::SetState {
                comment,
                accounts,
                new_addresses,
                block_hashes,
                previous_block_info,
                current_block_info,
            } => {
                for (address, account) in accounts.iter() {
                    mock_ref.add_account(AccountData{
                        address: address.value.into(),
                        nonce: account.nonce.value,
                        balance: account.balance.value.clone(),
                        storage: HashMap::new(),
                        contract: account.code.as_ref().map(|bytes_value| bytes_value.value.clone()),
                    });
                }
                for new_address in new_addresses.iter() {
                    mock_ref.put_new_address(
                        new_address.creator_address.value.into(),
                        new_address.creator_nonce.value,
                        new_address.new_address.value.into())
                }
            },
            Step::ScCall {
                tx_id,
                comment,
                tx,
                expect,
            } => {
                let call_tx = TxData{
                    from: tx.from.value.into(),
                    to: tx.to.value.into(),
                    call_value: tx.value.value.clone(),
                    func_name: tx.function.as_bytes().to_vec(),
                    new_contract: None,
                    args: tx.arguments.iter().map(|scen_arg| scen_arg.value.clone()).collect(),
                };
                let _ = mock_ref.execute_tx(call_tx);
            },
            Step::ScDeploy {
                tx_id,
                comment,
                tx,
                expect,
            } => {
                let deploy_tx = TxData{
                    from: tx.from.value.into(),
                    to: H256::zero(),
                    call_value: tx.value.value.clone(),
                    func_name: b"init".to_vec(),
                    new_contract: Some(tx.contract_code.value.clone()),
                    args: tx.arguments.iter().map(|scen_arg| scen_arg.value.clone()).collect(),
                };
                let result = mock_ref.execute_tx(deploy_tx);
                if let Some(tx_expect) = expect {
                    if !tx_expect.status.check(result.result_status as u64) {
                        panic!("Bad tx result status");
                    }
                    if !tx_expect.out.check(result.result_values.as_slice()) {
                        panic!("Bad tx output");
                    }
                }
            },
            Step::Transfer {
                tx_id,
                comment,
                tx,
            } => {},
            Step::ValidatorReward {
                tx_id,
                comment,
                tx,
            } => {},
            Step::CheckState {
                comment,
                accounts,
            } => {},
            Step::DumpState {..} => {
                mock_ref.print_accounts();
            },
        }
    }
}
