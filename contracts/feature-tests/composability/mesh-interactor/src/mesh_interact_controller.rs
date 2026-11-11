use crate::call_tree_config::{CONFIG_FILE, InteractConfig};

use multiversx_sc_snippets::imports::*;

pub struct ComposabilityInteract {
    pub interactor: Interactor,
    pub wallets: ComposabilityInteractWallets,
    pub forw_queue_code: BytesValue,
    pub config: InteractConfig,
}

impl ComposabilityInteract {
    pub async fn init() -> Self {
        let config = InteractConfig::load_from_file(CONFIG_FILE);
        let gateway_config = &config.gateway;
        let mut interactor = Interactor::new(&gateway_config.uri)
            .await
            .use_chain_simulator(gateway_config.use_chain_simulator());
        interactor.set_current_dir_from_workspace(
            "contracts/feature-tests/composability/mesh-interactor",
        );
        let shard_wallet_addresses = [
            interactor
                .register_wallet(test_wallets::for_shard(0u32.into()))
                .await,
            interactor
                .register_wallet(test_wallets::for_shard(1u32.into()))
                .await,
            interactor
                .register_wallet(test_wallets::for_shard(2u32.into()))
                .await,
        ];
        let forw_queue_code = BytesValue::interpret_from(
            "mxsc:../mesh-node/output/mesh-node.mxsc.json",
            &InterpreterContext::default(),
        );

        interactor.generate_blocks(30u64).await.unwrap();

        ComposabilityInteract {
            interactor,
            wallets: ComposabilityInteractWallets {
                shard_wallet_addresses,
            },
            forw_queue_code,
            config,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ComposabilityInteractWallets {
    shard_wallet_addresses: [Address; 3],
}

impl ComposabilityInteractWallets {
    pub fn wallet_for_shard(&self, shard: Option<ShardId>) -> Address {
        let index = shard.map(|s| s.as_u32() as usize).unwrap_or(0);
        self.shard_wallet_addresses[index].clone()
    }
}
