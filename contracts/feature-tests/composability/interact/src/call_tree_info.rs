use forwarder_queue::{CallInfo, forwarder_queue_proxy};
use multiversx_sc::codec::multi_types::MultiValueVec;
use multiversx_sc_snippets::imports::*;

use crate::{
    call_tree_config::{CALL_TREE_FILE, CallTreeConfig},
    comp_interact_controller::ComposabilityInteract,
};

impl ComposabilityInteract {
    /// Queries the `trace` view for every deployed contract in `call_tree.toml`
    /// and prints the results to console.
    pub async fn query_trace_info(&mut self) {
        let config = CallTreeConfig::load_from_file(CALL_TREE_FILE);

        let contracts_with_addresses: Vec<_> = config
            .contracts
            .iter()
            .filter_map(|c| {
                c.address.as_ref().map(|a| {
                    (
                        c.index,
                        c.name.clone(),
                        Bech32Address::from_bech32_string(a.clone()),
                    )
                })
            })
            .collect();

        if contracts_with_addresses.is_empty() {
            println!("No deployed contracts found in {CALL_TREE_FILE}. Run `setup` first.");
            return;
        }

        for (index, name, addr) in contracts_with_addresses {
            println!("\n=== Contract '{}' (index {}) @ {} ===", name, index, addr);

            let trace: MultiValueVec<Vec<CallInfo>> = self
                .interactor
                .query()
                .to(addr)
                .typed(forwarder_queue_proxy::ForwarderQueueProxy)
                .trace()
                .returns(ReturnsResultUnmanaged)
                .run()
                .await;

            if trace.0.is_empty() {
                println!("  trace: (empty)");
            } else {
                for (i, entry) in trace.0.iter().enumerate() {
                    print!("  trace[{i}]: [");
                    for (j, call_info) in entry.iter().enumerate() {
                        if j > 0 {
                            print!(", ");
                        }
                        print!(
                            "(caller_id={}, call_index={})",
                            call_info.caller_id, call_info.call_index
                        );
                    }
                    println!("]");
                }
            }
        }
    }
}
