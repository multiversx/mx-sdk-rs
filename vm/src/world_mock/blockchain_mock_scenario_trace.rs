use crate::{
    scenario::model::{AddressKey, AddressValue, Step},
    scenario_format::{interpret_trait::IntoRaw, serde_raw::ValueSubTree},
    BlockchainMock,
};
use multiversx_sc::types::Address;
use std::{collections::HashMap, path::Path};

const SC_ADDRESS_NUM_LEADING_ZEROS: u8 = 8;
const UNDERSCORE: u8 = b'_';
static ADDR_PREFIX: &str = "address:";
static SC_ADDR_PREFIX: &str = "sc:";
static HEX_PREFIX: &str = "0x";

impl BlockchainMock {
    pub fn write_scenario_trace<P: AsRef<Path>>(&mut self, file_path: P) {
        self.scenario_trace_prettify();

        let mandos_trace = core::mem::take(&mut self.scenario_trace);
        let mandos_trace_raw = mandos_trace.into_raw();
        mandos_trace_raw.save_to_file(file_path);
    }

    fn scenario_trace_prettify(&mut self) {
        for step in &mut self.scenario_trace.steps {
            match step {
                Step::ExternalSteps(_) => {},
                Step::SetState(set_state_step) => {
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
                            addr_key_to_pretty(&self.addr_to_pretty_string_map, addr_key);
                        set_state_step.accounts.insert(pretty_addr_key, acc);
                    }
                },
                Step::ScCall(sc_call_step) => {
                    sc_call_step.tx.from = addr_value_to_pretty(
                        &self.addr_to_pretty_string_map,
                        sc_call_step.tx.from.clone(),
                    );
                    sc_call_step.tx.to = addr_value_to_pretty(
                        &self.addr_to_pretty_string_map,
                        sc_call_step.tx.to.clone(),
                    );
                },
                Step::ScQuery(sc_query_step) => {
                    sc_query_step.tx.to = addr_value_to_pretty(
                        &self.addr_to_pretty_string_map,
                        sc_query_step.tx.to.clone(),
                    );
                },
                Step::ScDeploy(sc_deploy_step) => {
                    sc_deploy_step.tx.from = addr_value_to_pretty(
                        &self.addr_to_pretty_string_map,
                        sc_deploy_step.tx.from.clone(),
                    );
                },
                Step::Transfer(transfer_step) => {
                    transfer_step.tx.from = addr_value_to_pretty(
                        &self.addr_to_pretty_string_map,
                        transfer_step.tx.from.clone(),
                    );
                    transfer_step.tx.to = addr_value_to_pretty(
                        &self.addr_to_pretty_string_map,
                        transfer_step.tx.to.clone(),
                    );
                },
                Step::ValidatorReward(_) => todo!(),
                Step::CheckState(check_state_step) => {
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
                            addr_key_to_pretty(&self.addr_to_pretty_string_map, addr_key);
                        check_state_step
                            .accounts
                            .accounts
                            .insert(pretty_addr_key, acc);
                    }
                },
                Step::DumpState(_) => {},
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

pub fn address_as_scenario_string(address: &Address) -> String {
    let addr_bytes = address.as_array();
    let (string_start_index, prefix) = if super::is_smart_contract_address(address) {
        (SC_ADDRESS_NUM_LEADING_ZEROS as usize, SC_ADDR_PREFIX)
    } else {
        (0, ADDR_PREFIX)
    };

    let mut string_end_index = Address::len_bytes() - 1;
    while addr_bytes[string_end_index] == UNDERSCORE {
        string_end_index -= 1;
    }

    let addr_readable_part = &addr_bytes[string_start_index..=string_end_index];
    match String::from_utf8(addr_readable_part.to_vec()) {
        Ok(readable_string) => {
            let mut result = prefix.to_string();
            result.push_str(&readable_string);

            result
        },
        Err(_) => {
            let mut result = HEX_PREFIX.to_string();
            result.push_str(&hex::encode(&addr_bytes[..]));

            result
        },
    }
}
