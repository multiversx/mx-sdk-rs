use crate::{call_tree::CallState, comp_interact_config::Config, comp_interact_state::State};

use multiversx_sc_snippets::imports::*;

const INTERACTOR_SCENARIO_TRACE_PATH: &str = "comp_interact_trace.scen.json";

pub struct ComposabilityInteract {
    pub interactor: Interactor,
    pub wallet_address: Address,
    pub forw_queue_code: BytesValue,
    pub vault_code: BytesValue,
    #[allow(dead_code)]
    pub state: State,
}

impl ComposabilityInteract {
    pub async fn init() -> Self {
        let config = Config::load_config();
        let mut interactor = Interactor::new(config.gateway_uri(), config.use_chain_simulator())
            .await
            .with_tracer(INTERACTOR_SCENARIO_TRACE_PATH)
            .await;
        let wallet_address = interactor.register_wallet(test_wallets::judy()).await;
        let forw_queue_code = BytesValue::interpret_from(
            "mxsc:../forwarder-queue/output/forwarder-queue.mxsc.json",
            &InterpreterContext::default(),
        );
        let vault_code = BytesValue::interpret_from(
            "mxsc:../vault/output/vault.mxsc.json",
            &InterpreterContext::default(),
        );

        ComposabilityInteract {
            interactor,
            wallet_address,
            forw_queue_code,
            vault_code,
            state: State::load_state(),
        }
    }

    pub async fn full_scenario(
        &mut self,
        endpoint_name: &str,
        _endpoint_args: &Option<Vec<String>>,
    ) {
        let config = Config::load_config();
        let payment_token = config.token_id();
        let call_type = config.call_type();
        let payment_amount = config.token_amount();
        let payment_nonce = config.token_nonce();

        let call_state = CallState::simple_example_2();
        call_state.print();

        self.deploy_call_tree_contracts(&call_state).await;

        self.add_queued_calls_to_children(
            &call_state.forwarders,
            call_type,
            endpoint_name,
            payment_token,
            payment_nonce,
            payment_amount,
        )
        .await;

        self.call_root(&call_state).await;
    }
}
