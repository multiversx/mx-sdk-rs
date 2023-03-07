use crate::{call_tree::CallState, comp_interact_config::Config, comp_interact_state::State};

use multiversx_sc_snippets::{erdrs::wallet::Wallet, multiversx_sc::types::Address, Interactor};

pub struct ComposabilityInteract {
    pub interactor: Interactor,
    pub wallet_address: Address,
    pub state: State,
}

impl ComposabilityInteract {
    pub async fn init() -> Self {
        let config = Config::load_config();
        let mut interactor = Interactor::new(config.gateway()).await;
        let wallet_address =
            interactor.register_wallet(Wallet::from_pem_file(config.pem()).unwrap());

        ComposabilityInteract {
            interactor,
            wallet_address,
            state: State::load_state(),
        }
    }

    pub async fn full_scenario(&mut self) {
        let call_state = CallState::simple_example_2();
        call_state.print();

        self.deploy_call_tree_contracts(&call_state).await;
    }
}
