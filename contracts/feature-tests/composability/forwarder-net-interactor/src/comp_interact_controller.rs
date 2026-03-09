use crate::{
    call_tree_config::{CALL_TREE_FILE, CallTreeConfig},
    comp_interact_state::State,
};

use multiversx_sc_snippets::imports::*;

const INTERACTOR_SCENARIO_TRACE_PATH: &str = "comp_interact_trace.scen.json";

pub struct ComposabilityInteract {
    pub interactor: Interactor,
    pub wallet_address: Address,
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
        let wallet_address = interactor.register_wallet(test_wallets::judy()).await;
        let forw_queue_code = BytesValue::interpret_from(
            "mxsc:../forwarder-net/output/forwarder-net.mxsc.json",
            &InterpreterContext::default(),
        );

        ComposabilityInteract {
            interactor,
            wallet_address,
            forw_queue_code,
            state: State::load_state(),
        }
    }
}
