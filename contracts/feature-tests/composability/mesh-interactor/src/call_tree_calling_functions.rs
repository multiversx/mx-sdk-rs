use std::collections::HashMap;

use mesh_node::{ProgrammedCall, ProgrammedCallType, mesh_node_proxy};
use multiversx_sc_snippets::imports::*;

use crate::{
    call_tree_config::{CallTreeLayout, PaymentConfig, ProgrammedCallTypeConfig, STATE_FILE},
    mesh_interact_controller::ComposabilityInteract,
};

impl ComposabilityInteract {
    /// For every forwarder in the layout that has children, build the
    /// corresponding `ProgrammedCall` list and send a `set_queued_calls` tx.
    /// All txs are batched in a single homogenous call buffer.
    pub async fn program_calls(&mut self, layout: &CallTreeLayout) {
        let state = CallTreeLayout::load_from_file(STATE_FILE);

        println!("Setting up programmed calls from config...");

        // Build name → bech32 address map from state.
        let addr_map: HashMap<String, Bech32Address> = state
            .contracts
            .iter()
            .map(|(name, c)| {
                let addr = c
                    .address
                    .clone()
                    .unwrap_or_else(|| panic!("no address in state for '{name}'"));
                (name.clone(), Bech32Address::from_bech32_string(addr))
            })
            .collect();

        // Pre-compute shard wallets before creating the buffer (wallet_for_shard
        // borrows &self, which conflicts with the mutable borrow held by the buffer).
        let wallet_map: HashMap<&String, Address> = layout
            .contracts
            .iter()
            .map(|(name, c)| (name, self.wallets.wallet_for_shard(c.shard)))
            .collect();

        let mut buffer = self.interactor.homogenous_call_buffer();

        for (name, contract) in &layout.contracts {
            if contract.calls.is_empty() {
                continue;
            }

            let fwd_addr = addr_map
                .get(name)
                .unwrap_or_else(|| panic!("no address for forwarder '{name}'"))
                .clone();

            // Convert each ProgrammedCallConfig into a managed ProgrammedCall,
            // using the bottom-up gas estimate for each target contract.
            let mut calls: MultiValueManagedVec<StaticApi, ProgrammedCall<StaticApi>> =
                MultiValueManagedVec::new();
            for child_call in &contract.calls {
                let child_addr = addr_map
                    .get(&child_call.to)
                    .unwrap_or_else(|| panic!("no address for contract '{}'", child_call.to))
                    .to_address();

                let gas_limit = child_call.gas_limit.unwrap_or_else(|| {
                    panic!("gas_limit not set for call to '{}'", child_call.to,)
                });

                let call_type = to_queued_call_type(&child_call.call_type);
                calls.push(ProgrammedCall {
                    call_type,
                    to: ManagedAddress::from(child_addr),
                    gas_limit,
                    payments: to_payment_vec(&child_call.payments),
                });
            }

            let wallet = &wallet_map[name];
            println!(
                "Setting {} programmed call(s) on forwarder '{name}'",
                calls.len(),
            );
            buffer.push_tx(|tx| {
                tx.from(wallet)
                    .to(fwd_addr)
                    .gas(NumExpr("70,000,000"))
                    .typed(mesh_node_proxy::MeshNodeProxy)
                    .program_calls(calls)
                    .returns(ReturnsStatus)
            });
        }

        let results = buffer.run().await;
        for (i, status) in results.iter().enumerate() {
            if *status == 0u64 {
                println!("program_calls #{i}: ok");
            } else {
                println!("program_calls #{i}: failed with status {status}");
            }
        }
    }

    /// For every contract in the layout that has programmed returns, call the
    /// `program_returns` endpoint on-chain to configure them.
    pub async fn program_returns(&mut self, layout: &CallTreeLayout) {
        let has_returns = layout.contracts.values().any(|c| !c.returns.is_empty());
        if !has_returns {
            return;
        }

        let state = CallTreeLayout::load_from_file(STATE_FILE);
        println!("Setting up programmed returns from config...");

        let addr_map: HashMap<String, Bech32Address> = state
            .contracts
            .iter()
            .map(|(name, c)| {
                let addr = c
                    .address
                    .clone()
                    .unwrap_or_else(|| panic!("no address in state for '{name}'"));
                (name.clone(), Bech32Address::from_bech32_string(addr))
            })
            .collect();

        let wallet_map: HashMap<&String, Address> = layout
            .contracts
            .iter()
            .map(|(name, c)| (name, self.wallets.wallet_for_shard(c.shard)))
            .collect();

        let mut buffer = self.interactor.homogenous_call_buffer();

        for (name, contract) in &layout.contracts {
            if contract.returns.is_empty() {
                continue;
            }
            let fwd_addr = addr_map
                .get(name)
                .unwrap_or_else(|| panic!("no address for contract '{name}'"))
                .clone();
            let returns: MultiValueManagedVec<StaticApi, Payment<StaticApi>> =
                to_payment_vec(&contract.returns).into();
            let wallet = &wallet_map[name];
            println!(
                "Setting {} programmed return(s) on contract '{name}'",
                contract.returns.len(),
            );
            buffer.push_tx(|tx| {
                tx.from(wallet)
                    .to(fwd_addr)
                    .gas(NumExpr("70,000,000"))
                    .typed(mesh_node_proxy::MeshNodeProxy)
                    .program_returns(returns)
                    .returns(ReturnsStatus)
            });
        }

        let results = buffer.run().await;
        for (i, status) in results.iter().enumerate() {
            if *status == 0u64 {
                println!("program_returns #{i}: ok");
            } else {
                println!("program_returns #{i}: failed with status {status}");
            }
        }
    }

    /// Send all `[[start]]` transactions from the layout in a single batch.
    pub async fn bump(&mut self, layout: &CallTreeLayout) {
        let state = CallTreeLayout::load_from_file(STATE_FILE);

        if layout.start.is_empty() {
            println!("No start calls defined in the call tree layout");
            return;
        }

        // Pre-compute shard wallets before creating the buffer.
        let start_wallets: Vec<Address> = layout
            .start
            .iter()
            .map(|s| {
                if let Some(from) = &s.wallet {
                    Bech32Address::from_bech32_string(from.clone()).to_address()
                } else {
                    self.wallets.wallet_for_shard(s.shard)
                }
            })
            .collect();

        // Build name → bech32 address map from state.
        let addr_map: HashMap<String, Bech32Address> = state
            .contracts
            .iter()
            .map(|(name, c)| {
                let addr = c
                    .address
                    .clone()
                    .unwrap_or_else(|| panic!("no address in state for '{name}'"));
                (name.clone(), Bech32Address::from_bech32_string(addr))
            })
            .collect();

        let mut buffer = self.interactor.homogenous_call_buffer();

        for (start_call, wallet) in layout.start.iter().zip(start_wallets.iter()) {
            let to_addr = addr_map
                .get(&start_call.to)
                .unwrap_or_else(|| panic!("no address for contract '{}'", start_call.to))
                .clone();

            assert!(
                start_call.gas_limit.is_some(),
                "gas_limit not set for start call to '{}'",
                start_call.to,
            );
            let gas_limit = start_call.gas_limit.unwrap();
            println!(
                "Calling bump on contract '{}' ({}) with gas_limit = {gas_limit}",
                start_call.to, to_addr,
            );

            let start_payments = to_payment_vec(&start_call.payments);
            buffer.push_tx(|tx| {
                tx.from(wallet)
                    .to(to_addr)
                    .gas(gas_limit)
                    .typed(mesh_node_proxy::MeshNodeProxy)
                    .bump(IgnoreValue)
                    .payment(start_payments)
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

fn to_payment_vec(configs: &[PaymentConfig]) -> PaymentVec<StaticApi> {
    use multiversx_sc::codec::num_bigint::BigUint as RustBigUint;
    let mut vec = ManagedVec::new();
    for p in configs {
        let rust_amount: RustBigUint = p.amount.parse().expect("invalid payment amount");
        let amount = BigUint::<StaticApi>::from(rust_amount);
        let payment =
            Payment::try_new(p.token_id.as_str(), p.nonce, amount).expect("payment amount is zero");
        vec.push(payment);
    }
    vec
}

fn to_queued_call_type(call_type: &ProgrammedCallTypeConfig) -> ProgrammedCallType {
    match call_type {
        ProgrammedCallTypeConfig::Sync => ProgrammedCallType::Sync,
        ProgrammedCallTypeConfig::AsyncV1 => ProgrammedCallType::AsyncV1,
        ProgrammedCallTypeConfig::AsyncV2 => ProgrammedCallType::AsyncV2,
        ProgrammedCallTypeConfig::TransfExec => ProgrammedCallType::TransferExecute,
    }
}
