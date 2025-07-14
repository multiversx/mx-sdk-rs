mod governance_sc_interact_cli;
mod governance_sc_interact_config;
mod governance_sc_interact_state;

use clap::Parser;
pub use governance_sc_interact_config::Config;
use governance_sc_interact_state::State;

use multiversx_sc_snippets::{imports::*, sdk::gateway::SetStateAccount};

pub async fn governance_sc_interact_cli() {
    env_logger::init();

    let mut interactor = GovernanceCallsInteract::new(Config::load_config()).await;

    let cli = governance_sc_interact_cli::InteractCli::parse();
    match cli.command {
        Some(governance_sc_interact_cli::InteractCliCommand::Propose) => {
            interactor
                .proposal("6db132d759482f9f3515fe3ca8f72a8d6dc61244", 981, 983)
                .await;
        },
        Some(governance_sc_interact_cli::InteractCliCommand::View) => {
            interactor.view_config().await;
        },
        None => {},
    }
}

pub struct GovernanceCallsInteract {
    pub interactor: Interactor,
    pub owner: Bech32Address,
    pub voter: Bech32Address,
    pub delegator2: Bech32Address,
    pub state: State,
}

impl GovernanceCallsInteract {
    pub async fn new(config: Config) -> Self {
        let mut interactor = Interactor::new(config.gateway_uri())
            .await
            .use_chain_simulator(config.is_chain_simulator());

        interactor.set_current_dir_from_workspace("tools/interactor-governance-func-calls");
        let owner = interactor.register_wallet(test_wallets::eve()).await;
        let voter = interactor.register_wallet(test_wallets::bob()).await;
        let delegator2 = interactor.register_wallet(test_wallets::dan()).await;

        // generate blocks until ESDTSystemSCAddress is enabled
        interactor.generate_blocks_until_epoch(1).await.unwrap();

        Self {
            interactor,
            owner: owner.into(),
            voter: voter.into(),
            delegator2: delegator2.into(),
            state: State::load_state(),
        }
    }

    pub async fn set_state(&mut self, address: &Address) {
        let mut account = self.interactor.get_account(address).await;
        account.balance = "10000000000000000000000".to_owned();
        let set_state_account = SetStateAccount::from(account);
        let vec_state = vec![set_state_account];

        let _set_state_response = self.interactor.set_state(vec_state).await;
    }

    pub async fn view_config(&mut self) {
        let raw = self
            .interactor
            .query()
            .to(GovernanceSystemSCAddress)
            .typed(GovernanceSCProxy)
            .view_config()
            .returns(ReturnsRawResult)
            // .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("config raw: {:?}", raw);
    }

    pub async fn proposal(
        &mut self,
        commit_hash: &str,
        start_vote_epoch: usize,
        end_vote_epoch: usize,
    ) {
        let user_address = self
            .interactor
            .register_wallet(Wallet::from_pem_file("delegator2.pem").unwrap())
            .await;

        self.interactor
            .tx()
            .from(user_address)
            .to(GovernanceSystemSCAddress)
            .typed(GovernanceSCProxy)
            .proposal(commit_hash, start_vote_epoch, end_vote_epoch)
            .gas(60_000_000u64)
            .run()
            .await;
    }

    pub async fn vote(&mut self, sender: &Bech32Address, nonce: usize, vote_type: &str) {
        self.interactor
            .tx()
            .from(sender)
            .to(GovernanceSystemSCAddress)
            .typed(GovernanceSCProxy)
            .vote(nonce, vote_type)
            .gas(60_000_000u64)
            .run()
            .await;
    }
}
