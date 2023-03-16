use crate::{call_tree::CallState, comp_interact_config::Config, comp_interact_state::State};

use forwarder_queue::QueuedCallType;
use multiversx_sc_snippets::{
    multiversx_sc::types::{Address, EgldOrEsdtTokenIdentifier},
    multiversx_sc_scenario::{test_wallets::judy, DebugApi},
    Interactor,
};

const INTERACTOR_SCENARIO_TRACE_PATH: &str = "comp_interact_trace.scen.json";

pub struct ComposabilityInteract {
    pub interactor: Interactor,
    pub wallet_address: Address,
    pub state: State,
}

impl ComposabilityInteract {
    pub async fn init() -> Self {
        let config = Config::load_config();
        let mut interactor = Interactor::new(config.gateway())
            .await
            .with_tracer(INTERACTOR_SCENARIO_TRACE_PATH)
            .await;
        let wallet_address = interactor.register_wallet(judy());

        ComposabilityInteract {
            interactor,
            wallet_address,
            state: State::load_state(),
        }
    }

    pub async fn full_scenario(
        &mut self,
        call_type: QueuedCallType,
        endpoint_name: &str,
        payment_token: EgldOrEsdtTokenIdentifier<DebugApi>,
        payment_nonce: u64,
        payment_amount: u64,
    ) {
        let call_state = CallState::simple_example_2();
        call_state.print();

        self.deploy_call_tree_contracts(&call_state).await;

        self.add_calls_to_all_fwds(&call_state, call_type, endpoint_name)
            .await;

        self.call_root(&call_state, payment_token, payment_nonce, payment_amount)
            .await;
    }
}
