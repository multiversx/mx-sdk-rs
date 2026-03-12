use crate::{
    call_tree_config::{CallTreeLayout, STATE_FILE},
    mesh_interact_controller::ComposabilityInteract,
};

use mesh_node::mesh_node_proxy;
use multiversx_sc_snippets::imports::*;

const DEPLOY_GAS_LIMIT: NumExpr = NumExpr("80,000,000");

impl ComposabilityInteract {
    /// Deploy all contracts described in the call tree layout, then write the
    /// assigned addresses into `state.toml`.
    pub async fn deploy_call_tree(&mut self, layout: &CallTreeLayout) {
        println!(
            "Deploying {} call tree contracts...",
            layout.contracts.len()
        );

        // Deploy all contracts in a single batch.
        let name_address_pairs = self.deploy_all(layout).await;

        let mut state = layout.clone();
        for (name, address) in name_address_pairs {
            println!("Deployed '{name}' at {address}");
            state.contracts.get_mut(&name).unwrap().address = Some(address.to_string());
        }

        // Fill in the `from` field for each start call with the wallet bech32 address.
        for start in &mut state.start {
            let wallet = self.wallets.wallet_for_shard(start.shard);
            start.wallet = Some(Bech32Address::from(wallet).to_bech32_string());
        }

        state.save_to_file(STATE_FILE);
        println!("Addresses saved to {STATE_FILE}");
    }

    async fn deploy_all(&mut self, layout: &CallTreeLayout) -> Vec<(String, Bech32Address)> {
        let wallets = self.wallets.clone();
        let mut buffer = self.interactor.homogenous_call_buffer();
        for (name, contract) in &layout.contracts {
            let wallet = wallets.wallet_for_shard(contract.shard);
            let code_metadata = if contract.payable.unwrap_or(false) {
                CodeMetadata::PAYABLE
            } else {
                CodeMetadata::DEFAULT
            };
            buffer.push_tx(|tx| {
                tx.from(wallet)
                    .typed(mesh_node_proxy::ForwarderQueueProxy)
                    .init(name)
                    .code(&self.forw_queue_code)
                    .code_metadata(code_metadata)
                    .gas(DEPLOY_GAS_LIMIT)
                    .returns(PassValue(name.clone()))
                    .returns(ReturnsNewBech32Address)
            });
        }
        buffer.run().await
    }
}
