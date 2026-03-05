use crate::{
    call_tree_config::{CALL_TREE_FILE, CallTreeConfig},
    comp_interact_controller::ComposabilityInteract,
};

use forwarder_queue::forwarder_queue_proxy;
use multiversx_sc_snippets::imports::*;

impl ComposabilityInteract {
    /// Deploy all contracts described in `call_tree.toml`, then write the
    /// assigned addresses back into the same file.
    pub async fn deploy_call_tree(&mut self) {
        let mut config = CallTreeConfig::load_from_file(CALL_TREE_FILE);

        // Deploy all contracts in declaration order in a single batch.
        let addresses = self.deploy_all(&config).await;

        for (contract, address) in config.contracts.iter_mut().zip(addresses.iter()) {
            println!("Deployed '{}' at {}", contract.name, address);
            contract.address = Some(address.to_string());
        }

        config.save_to_file(CALL_TREE_FILE);
        println!("Addresses saved to {CALL_TREE_FILE}");
    }

    async fn deploy_all(&mut self, config: &CallTreeConfig) -> Vec<Bech32Address> {
        let mut buffer = self.interactor.homogenous_call_buffer();
        for contract in &config.contracts {
            buffer.push_tx(|tx| {
                tx.from(&self.wallet_address)
                    .typed(forwarder_queue_proxy::ForwarderQueueProxy)
                    .init(contract.index)
                    .code(&self.forw_queue_code)
                    .gas(NumExpr("70,000,000"))
                    .returns(ReturnsNewBech32Address)
            });
        }
        buffer.run().await
    }
}
