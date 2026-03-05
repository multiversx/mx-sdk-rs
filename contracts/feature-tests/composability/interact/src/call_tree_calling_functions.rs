use std::collections::HashMap;

use forwarder_queue::{QueuedCall, QueuedCallType, forwarder_queue_proxy};
use multiversx_sc_snippets::imports::*;

use crate::{
    call_tree_config::{CALL_TREE_FILE, CallTreeConfig, CallType},
    comp_interact_controller::ComposabilityInteract,
};

impl ComposabilityInteract {
    /// For every forwarder in `call_tree.toml` that has children, build the
    /// corresponding `QueuedCall` list and send a `set_queued_calls` tx.
    /// All txs are batched in a single homogenous call buffer.
    pub async fn set_queued_calls_from_config(&mut self) {
        let config = CallTreeConfig::load_from_file(CALL_TREE_FILE);

        // Build name → bech32 address map.
        let addr_map: HashMap<String, Bech32Address> = config
            .contracts
            .iter()
            .filter_map(|(name, c)| {
                c.address
                    .as_ref()
                    .map(|a| (name.clone(), Bech32Address::from_bech32_string(a.clone())))
            })
            .collect();

        let mut buffer = self.interactor.homogenous_call_buffer();

        for (name, contract) in &config.contracts {
            if contract.children.is_empty() {
                continue;
            }

            let fwd_addr = addr_map
                .get(name)
                .unwrap_or_else(|| panic!("no address for forwarder '{name}'"))
                .clone();

            // Convert each ChildCall config into a managed QueuedCall.
            let mut calls: MultiValueManagedVec<StaticApi, QueuedCall<StaticApi>> =
                MultiValueManagedVec::new();
            for child_call in &contract.children {
                let child_addr = addr_map
                    .get(&child_call.to)
                    .unwrap_or_else(|| panic!("no address for contract '{}'", child_call.to))
                    .to_address();

                let call_type = to_queued_call_type(&child_call.call_type);
                calls.push(QueuedCall {
                    call_type,
                    to: ManagedAddress::from(child_addr),
                    gas_limit: child_call.gas_limit,
                    endpoint_name: ManagedBuffer::from(b"bump"),
                    args: ManagedArgBuffer::new(),
                    payments: ManagedVec::new(),
                });
            }

            println!(
                "Setting {} queued call(s) on forwarder '{name}'",
                calls.len(),
            );
            buffer.push_tx(|tx| {
                tx.from(&self.wallet_address)
                    .to(fwd_addr)
                    .gas(NumExpr("70,000,000"))
                    .typed(forwarder_queue_proxy::ForwarderQueueProxy)
                    .set_queued_calls(calls)
                    .returns(ReturnsStatus)
            });
        }

        let results = buffer.run().await;
        for (i, status) in results.iter().enumerate() {
            if *status == 0u64 {
                println!("set_queued_calls #{i}: ok");
            } else {
                println!("set_queued_calls #{i}: failed with status {status}");
            }
        }
    }

    /// Send all `[[start]]` transactions from `call_tree.toml` in a single batch.
    pub async fn bump(&mut self) {
        let config = CallTreeConfig::load_from_file(CALL_TREE_FILE);

        if config.start.is_empty() {
            println!("No start calls defined in {CALL_TREE_FILE}");
            return;
        }

        // Build name → bech32 address map.
        let addr_map: HashMap<String, Bech32Address> = config
            .contracts
            .iter()
            .filter_map(|(name, c)| {
                c.address
                    .as_ref()
                    .map(|a| (name.clone(), Bech32Address::from_bech32_string(a.clone())))
            })
            .collect();

        let mut buffer = self.interactor.homogenous_call_buffer();

        for start_call in &config.start {
            let to_addr = addr_map
                .get(&start_call.to)
                .unwrap_or_else(|| panic!("no address for contract '{}'", start_call.to))
                .clone();

            println!("Calling bump on contract '{}' ({})", start_call.to, to_addr);

            buffer.push_tx(|tx| {
                tx.from(&self.wallet_address)
                    .to(to_addr)
                    .gas(start_call.gas_limit)
                    .typed(forwarder_queue_proxy::ForwarderQueueProxy)
                    .bump(IgnoreValue)
                    .returns(ReturnsStatus)
            });
        }

        let results = buffer.run().await;
        for (i, status) in results.iter().enumerate() {
            if *status == 0u64 {
                println!("start call #{i}: ok");
            } else {
                println!("start call #{i}: failed with status {status}");
            }
        }
    }
}

fn to_queued_call_type(call_type: &CallType) -> QueuedCallType {
    match call_type {
        CallType::Sync => QueuedCallType::Sync,
        CallType::LegacyAsync => QueuedCallType::LegacyAsync,
        CallType::TransferExecute => QueuedCallType::TransferExecute,
        CallType::Promise => QueuedCallType::Promise,
    }
}
