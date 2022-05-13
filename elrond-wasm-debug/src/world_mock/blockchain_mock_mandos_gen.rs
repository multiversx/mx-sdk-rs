use elrond_wasm::types::Address;
use mandos::{
    interpret_trait::IntoRaw,
    model::{AddressKey, AddressValue},
    serde_raw::ValueSubTree,
};
use std::{collections::HashMap, path::Path};

use crate::BlockchainMock;

impl BlockchainMock {
    pub fn write_mandos_trace<P: AsRef<Path>>(&mut self, file_path: P) {
        self.mandos_trace_prettify();

        let mandos_trace = core::mem::take(&mut self.mandos_trace);
        let mandos_trace_raw = mandos_trace.into_raw();
        mandos_trace_raw.save_to_file(file_path);
    }

    fn mandos_trace_prettify(&mut self) {
        for step in &mut self.mandos_trace.steps {
            match step {
                mandos::model::Step::ExternalSteps(_) => {},
                mandos::model::Step::SetState(set_state_step) => {
                    let acc_map_keys = set_state_step
                        .accounts
                        .keys()
                        .cloned()
                        .collect::<Vec<AddressKey>>();
                    let mut accounts = Vec::new();

                    for addr_key in &acc_map_keys {
                        let acc = set_state_step.accounts.remove(addr_key).unwrap();
                        accounts.push(acc);
                    }

                    for (addr_key, acc) in acc_map_keys.into_iter().zip(accounts.into_iter()) {
                        let pretty_addr_key =
                            addr_key_to_pretty(&self.addr_to_mandos_string_map, addr_key);
                        set_state_step.accounts.insert(pretty_addr_key, acc);
                    }
                },
                mandos::model::Step::ScCall(sc_call_step) => {
                    sc_call_step.tx.from = addr_value_to_pretty(
                        &self.addr_to_mandos_string_map,
                        sc_call_step.tx.from.clone(),
                    );
                    sc_call_step.tx.to = addr_value_to_pretty(
                        &self.addr_to_mandos_string_map,
                        sc_call_step.tx.to.clone(),
                    );
                },
                mandos::model::Step::ScQuery(sc_query_step) => {
                    sc_query_step.tx.to = addr_value_to_pretty(
                        &self.addr_to_mandos_string_map,
                        sc_query_step.tx.to.clone(),
                    );
                },
                mandos::model::Step::ScDeploy(sc_deploy_step) => {
                    sc_deploy_step.tx.from = addr_value_to_pretty(
                        &self.addr_to_mandos_string_map,
                        sc_deploy_step.tx.from.clone(),
                    );
                },
                mandos::model::Step::Transfer(transfer_step) => {
                    transfer_step.tx.from = addr_value_to_pretty(
                        &self.addr_to_mandos_string_map,
                        transfer_step.tx.from.clone(),
                    );
                    transfer_step.tx.to = addr_value_to_pretty(
                        &self.addr_to_mandos_string_map,
                        transfer_step.tx.to.clone(),
                    );
                },
                mandos::model::Step::ValidatorReward(_) => todo!(),
                mandos::model::Step::CheckState(check_state_step) => {
                    let acc_map_keys = check_state_step
                        .accounts
                        .accounts
                        .keys()
                        .cloned()
                        .collect::<Vec<AddressKey>>();
                    let mut check_accounts = Vec::new();

                    for addr_key in &acc_map_keys {
                        let acc = check_state_step.accounts.accounts.remove(addr_key).unwrap();
                        check_accounts.push(acc);
                    }

                    for (addr_key, acc) in acc_map_keys.into_iter().zip(check_accounts.into_iter())
                    {
                        let pretty_addr_key =
                            addr_key_to_pretty(&self.addr_to_mandos_string_map, addr_key);
                        check_state_step
                            .accounts
                            .accounts
                            .insert(pretty_addr_key, acc);
                    }
                },
                mandos::model::Step::DumpState(_) => {},
            }
        }
    }
}

fn addr_key_to_pretty(
    addr_to_mandos_string_map: &HashMap<Address, String>,
    addr_key: AddressKey,
) -> AddressKey {
    match addr_to_mandos_string_map.get(&addr_key.value) {
        Some(pretty_addr) => AddressKey {
            value: addr_key.value,
            original: pretty_addr.clone(),
        },
        None => addr_key,
    }
}

fn addr_value_to_pretty(
    addr_to_mandos_string_map: &HashMap<Address, String>,
    addr_val: AddressValue,
) -> AddressValue {
    match addr_to_mandos_string_map.get(&addr_val.value) {
        Some(pretty_addr) => AddressValue {
            value: addr_val.value,
            original: ValueSubTree::Str(pretty_addr.clone()),
        },
        None => addr_val,
    }
}
