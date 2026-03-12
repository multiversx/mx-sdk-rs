use crate::{
    call_tree_config::{CALL_TREE_FILE, CallTreeConfig},
    mesh_interact_controller::ComposabilityInteract,
};

use mesh_node::mesh_node_proxy;
use multiversx_sc_snippets::imports::*;

const DEPLOY_GAS_LIMIT: NumExpr = NumExpr("80,000,000");

impl ComposabilityInteract {
    /// Deploy all contracts described in `call_tree.toml`, then write the
    /// assigned addresses back into the same file.
    pub async fn deploy_call_tree(&mut self) {
        let mut config = CallTreeConfig::load_from_file(CALL_TREE_FILE);

        println!(
            "Deploying {} call tree contracts...",
            config.contracts.len()
        );

        // Deploy all contracts in a single batch.
        let name_address_pairs = self.deploy_all(&config).await;

        for (name, address) in name_address_pairs {
            println!("Deployed '{name}' at {address}");
            config.contracts.get_mut(&name).unwrap().address = Some(address.to_string());
        }

        config.save_to_file(CALL_TREE_FILE);
        println!("Addresses saved to {CALL_TREE_FILE}");
    }

    async fn deploy_all(&mut self, config: &CallTreeConfig) -> Vec<(String, Bech32Address)> {
        let wallets = self.wallets.clone();
        let mut buffer = self.interactor.homogenous_call_buffer();
        for (name, contract) in &config.contracts {
            let wallet = wallets.wallet_for_shard(contract.shard);
            buffer.push_tx(|tx: Tx<ScenarioTxEnvData, (), (), (), (), (), ()>| {
                tx.from(wallet)
                    .typed(mesh_node_proxy::ForwarderQueueProxy)
                    .init(name)
                    .code(&self.forw_queue_code)
                    .code_metadata(CodeMetadata::PAYABLE)
                    .gas(DEPLOY_GAS_LIMIT)
                    .returns(PassValue(name.clone()))
                    .returns(ReturnsNewBech32Address)
            });
        }
        buffer.run().await
    }
}
