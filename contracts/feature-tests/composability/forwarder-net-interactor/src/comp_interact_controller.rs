use crate::{
    call_tree_config::{CALL_TREE_FILE, CallTreeConfig},
    comp_interact_state::State,
};

use multiversx_sc_snippets::imports::*;

const INTERACTOR_SCENARIO_TRACE_PATH: &str = "comp_interact_trace.scen.json";

pub struct ComposabilityInteract {
    pub interactor: Interactor,
    pub wallets: ComposabilityInteractWallets,
    pub forw_queue_code: BytesValue,
    #[allow(dead_code)]
    pub state: State,
}

impl ComposabilityInteract {
    pub async fn init() -> Self {
        let tree_config = CallTreeConfig::load_from_file(CALL_TREE_FILE);
        let gateway_config = &tree_config.gateway;
        let mut interactor = Interactor::new(&gateway_config.uri)
            .await
            .use_chain_simulator(tree_config.gateway.use_chain_simulator())
            .with_tracer(INTERACTOR_SCENARIO_TRACE_PATH)
            .await;
        interactor.set_current_dir_from_workspace("contracts/feature-tests/composability/interact");
        let shard_wallet_addresses = [
            interactor.register_wallet(test_wallets::for_shard(0)).await,
            interactor.register_wallet(test_wallets::for_shard(1)).await,
            interactor.register_wallet(test_wallets::for_shard(2)).await,
        ];
        let forw_queue_code = BytesValue::interpret_from(
            "mxsc:../forwarder-net/output/forwarder-net.mxsc.json",
            &InterpreterContext::default(),
        );

        ComposabilityInteract {
            interactor,
            wallets: ComposabilityInteractWallets {
                shard_wallet_addresses,
            },
            forw_queue_code,
            state: State::load_state(),
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
