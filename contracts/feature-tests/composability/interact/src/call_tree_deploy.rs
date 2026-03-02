use crate::{
    call_tree_config::{CALL_TREE_FILE, CallTreeConfig, ContractKind},
    comp_interact_controller::ComposabilityInteract,
    forwarder_queue_proxy, vault_proxy,
};

use multiversx_sc_snippets::imports::*;

impl ComposabilityInteract {
    /// Deploy all contracts described in `call_tree.toml`, then write the
    /// assigned addresses back into the same file.
    pub async fn deploy_call_tree(&mut self) {
        let mut config = CallTreeConfig::load_from_file(CALL_TREE_FILE);

        // Collect indices split by kind so we know which result maps to which entry.
        let forwarder_indices: Vec<usize> = config
            .contracts
            .iter()
            .filter(|c| c.kind == ContractKind::Forwarder)
            .map(|c| c.index)
            .collect();
        let vault_indices: Vec<usize> = config
            .contracts
            .iter()
            .filter(|c| c.kind == ContractKind::Vault)
            .map(|c| c.index)
            .collect();

        // Deploy all forwarder-queue contracts in one parallel batch.
        let forwarder_addresses = self.deploy_forwarders(forwarder_indices.len()).await;
        // Deploy all vault contracts in one parallel batch.
        let vault_addresses = self.deploy_vaults(vault_indices.len()).await;

        // Write addresses back into the config by matching on the contract index.
        let index_to_fwd_addr: std::collections::HashMap<usize, String> = forwarder_indices
            .into_iter()
            .zip(forwarder_addresses.iter().map(|a| a.to_string()))
            .collect();
        let index_to_vault_addr: std::collections::HashMap<usize, String> = vault_indices
            .into_iter()
            .zip(vault_addresses.iter().map(|a| a.to_string()))
            .collect();

        for contract in &mut config.contracts {
            let addr = match contract.kind {
                ContractKind::Forwarder => index_to_fwd_addr.get(&contract.index),
                ContractKind::Vault => index_to_vault_addr.get(&contract.index),
            };
            if let Some(a) = addr {
                println!("Deployed '{}' at {}", contract.name, a);
                contract.address = Some(a.clone());
            }
        }

        config.save_to_file(CALL_TREE_FILE);
        println!("Addresses saved to {CALL_TREE_FILE}");
    }

    async fn deploy_forwarders(&mut self, count: usize) -> Vec<Bech32Address> {
        let mut buffer = self.interactor.homogenous_call_buffer();
        for _ in 0..count {
            buffer.push_tx(|tx| {
                tx.from(&self.wallet_address)
                    .typed(forwarder_queue_proxy::ForwarderQueueProxy)
                    .init(IgnoreValue)
                    .code(&self.forw_queue_code)
                    .gas(NumExpr("70,000,000"))
                    .returns(ReturnsNewBech32Address)
            });
        }
        buffer.run().await
    }

    async fn deploy_vaults(&mut self, count: usize) -> Vec<Bech32Address> {
        let mut buffer = self.interactor.homogenous_call_buffer();
        for _ in 0..count {
            buffer.push_tx(|tx| {
                tx.from(&self.wallet_address)
                    .typed(vault_proxy::VaultProxy)
                    .init(IgnoreValue)
                    .code(&self.vault_code)
                    .gas(NumExpr("70,000,000"))
                    .returns(ReturnsNewBech32Address)
            });
        }
        buffer.run().await
    }
}
