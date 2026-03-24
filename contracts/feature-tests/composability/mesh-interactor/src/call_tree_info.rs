use mesh_node::{Trace, TraceName, mesh_node_proxy};
use multiversx_sc::codec::multi_types::MultiValueVec;
use multiversx_sc_snippets::imports::*;

use crate::{
    call_tree_config::{CallTreeLayout, STATE_FILE},
    mesh_interact_controller::ComposabilityInteract,
};

fn fmt_payments(payments: &multiversx_sc_snippets::imports::PaymentVec<StaticApi>) -> String {
    let mut out = String::from("[");
    for (i, p) in payments.iter().enumerate() {
        if i > 0 {
            out.push_str(", ");
        }
        out.push_str(&format!(
            "{}:{}:{}",
            p.token_identifier, p.token_nonce, p.amount
        ));
    }
    out.push(']');
    out
}

fn fmt_gas(v: u64) -> String {
    let s = v.to_string();
    let mut out = String::new();
    for (i, ch) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            out.push(' ');
        }
        out.push(ch);
    }
    let formatted: String = out.chars().rev().collect();
    // 9 digits with 2 group separators = 11 chars wide
    format!("{formatted:>11}")
}

impl ComposabilityInteract {
    /// Queries the `trace` view for every deployed contract in `state.toml`
    /// and prints the results to console.
    pub async fn query_trace_info(&mut self) {
        let state = CallTreeLayout::load_from_file(STATE_FILE);

        let contracts_with_addresses: Vec<_> = state
            .contracts
            .iter()
            .filter_map(|(name, c)| {
                c.address.as_ref().map(|addr| {
                    (
                        name.clone(),
                        Bech32Address::from_bech32_string(addr.clone()),
                    )
                })
            })
            .collect();

        if contracts_with_addresses.is_empty() {
            println!("No deployed contracts found in {STATE_FILE}. Run `setup` first.");
            return;
        }

        for (name, addr) in contracts_with_addresses {
            println!("\n=== Contract '{name}' @ {addr} ===");

            let account = self.interactor.get_account(&addr.to_address()).await;
            let esdts = self.interactor.get_account_esdt(&addr.to_address()).await;
            let mut balances: Vec<String> = Vec::new();
            if account.balance != "0" {
                balances.push(format!("EGLD={}", account.balance));
            }
            let mut esdt_list: Vec<_> = esdts.into_values().collect();
            esdt_list.sort_by(|a, b| a.token_identifier.cmp(&b.token_identifier));
            for esdt in &esdt_list {
                if esdt.balance != "0" {
                    balances.push(format!("{}={}", esdt.token_identifier, esdt.balance));
                }
            }
            if !balances.is_empty() {
                println!("  balance: {}", balances.join(" "));
            }

            let trace: MultiValueVec<Trace<StaticApi>> = self
                .interactor
                .query()
                .to(addr)
                .typed(mesh_node_proxy::MeshNodeProxy)
                .trace()
                .returns(ReturnsResultUnmanaged)
                .run()
                .await;

            if trace.0.is_empty() {
                println!("  trace: (empty)");
            } else {
                for (i, entry) in trace.0.iter().enumerate() {
                    let location = match entry.location {
                        TraceName::Bump => "Bump",
                        TraceName::AsyncV1CallbackOk => "Cb1✓",
                        TraceName::AsyncV1CallbackErr => "Cb1✗",
                    };
                    let gas_used = entry.initial_gas.saturating_sub(entry.final_gas);
                    print!(
                        "  trace[{i}] {location} (block_nonce:{}, gas:{} - {} = {}, items: [",
                        entry.block_nonce,
                        fmt_gas(entry.initial_gas),
                        fmt_gas(gas_used),
                        fmt_gas(entry.final_gas),
                    );
                    for (j, item) in entry.input.iter().enumerate() {
                        if j > 0 {
                            print!(", ");
                        }
                        print!("({} #{})", item.caller_id, item.call_index);
                    }
                    print!("])");
                    if !entry.results.is_empty() {
                        print!(" results: [");
                        for (j, buf) in entry.results.iter().enumerate() {
                            if j > 0 {
                                print!(", ");
                            }
                            let bytes = buf.to_boxed_bytes();
                            print!("0x");
                            for byte in bytes.as_slice() {
                                print!("{byte:02x}");
                            }
                        }
                        print!("]");
                    }
                    if !entry.call_value.is_empty() {
                        print!(" cv: {}", fmt_payments(&entry.call_value));
                    }
                    if !entry.back_transfers.is_empty() {
                        print!(" bt: {}", fmt_payments(&entry.back_transfers));
                    }
                    println!();
                }
            }
        }
    }
}
