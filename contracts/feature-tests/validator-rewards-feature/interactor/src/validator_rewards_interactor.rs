mod validator_rewards_interactor_cli;

use clap::Parser;
use multiversx_sc_snippets::imports::*;
use serde::{Deserialize, Serialize};
use validator_rewards_feature::validator_rewards_feature_proxy;

/// Validator Rewards Interact configuration
#[derive(Debug, Deserialize)]
pub struct Config {
    pub connection: ConnectionConfig,
    pub owner: WalletConfig,
}

impl InteractorConfig for Config {
    fn connection(&self) -> &ConnectionConfig {
        &self.connection
    }

    fn register_wallets(&self) -> Vec<Wallet> {
        vec![self.owner.wallet().clone()]
    }
}

/// Validator Rewards Interact state
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct State {
    pub contract_address: Option<Bech32Address>,
}

impl State {
    /// Returns the validator rewards contract
    pub fn current_contract_address(&self) -> &Bech32Address {
        self.contract_address
            .as_ref()
            .expect("no known validator rewards contract, deploy first")
    }
}

pub const CODE_PATH: MxscPath = MxscPath::new("../output/validator-rewards-feature.mxsc.json");

pub async fn validator_rewards_cli() {
    env_logger::init();

    let mut interact = ValidatorRewardsInteract::new().await;

    let cli = validator_rewards_interactor_cli::InteractCli::parse();
    match &cli.command {
        Some(validator_rewards_interactor_cli::InteractCliCommand::Deploy) => {
            interact.deploy().await;
        }
        Some(validator_rewards_interactor_cli::InteractCliCommand::Rewards) => {
            let cumulated = interact.cumulated_rewards().await;
            let unclaimed = interact.unclaimed_rewards().await;
            println!("cumulated: {cumulated}, unclaimed: {unclaimed}");
        }
        Some(validator_rewards_interactor_cli::InteractCliCommand::Claim) => {
            let claimed = interact.claim_rewards().await;
            println!("claimed: {claimed}");
        }
        None => {}
    }
}

pub struct ValidatorRewardsInteract {
    pub interactor: Interactor,
    pub config: Config,
    pub state: AutoSave<State>,
}

impl ValidatorRewardsInteract {
    pub async fn new() -> Self {
        let mut interactor = Interactor::empty().with_current_dir(env!("CARGO_MANIFEST_DIR"));
        let config: Config = interactor.load_config_toml().await;
        let state = interactor.load_state::<State>();
        Self {
            interactor,
            config,
            state,
        }
    }

    pub fn contract_address(&self) -> Bech32Address {
        self.state.current_contract_address().clone()
    }

    pub async fn deploy(&mut self) {
        let new_address = self
            .interactor
            .tx()
            .from(self.config.owner.address())
            .gas(30_000_000)
            .typed(validator_rewards_feature_proxy::ValidatorRewardsFeatureProxy)
            .init()
            .code(CODE_PATH)
            .code_metadata(CodeMetadata::UPGRADEABLE)
            .returns(ReturnsNewBech32Address)
            .run()
            .await;

        println!("new address: {new_address}");
        self.state.contract_address = Some(new_address);
    }

    pub async fn store_note(&mut self, note: &[u8]) {
        self.interactor
            .tx()
            .from(self.config.owner.address())
            .to(self.state.current_contract_address())
            .gas(6_000_000)
            .typed(validator_rewards_feature_proxy::ValidatorRewardsFeatureProxy)
            .store_note(note)
            .run()
            .await;
    }

    pub async fn note(&mut self) -> Vec<u8> {
        self.interactor
            .query()
            .to(self.state.current_contract_address())
            .typed(validator_rewards_feature_proxy::ValidatorRewardsFeatureProxy)
            .note()
            .returns(ReturnsResult)
            .run()
            .await
            .to_vec()
    }

    /// Reads the cumulated rewards through the contract's blockchain API call.
    pub async fn cumulated_rewards(&mut self) -> RustBigUint {
        self.interactor
            .query()
            .to(self.state.current_contract_address())
            .typed(validator_rewards_feature_proxy::ValidatorRewardsFeatureProxy)
            .get_cumulated_rewards()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await
    }

    pub async fn claimed_rewards(&mut self) -> RustBigUint {
        self.interactor
            .query()
            .to(self.state.current_contract_address())
            .typed(validator_rewards_feature_proxy::ValidatorRewardsFeatureProxy)
            .claimed_rewards()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await
    }

    pub async fn unclaimed_rewards(&mut self) -> RustBigUint {
        self.interactor
            .query()
            .to(self.state.current_contract_address())
            .typed(validator_rewards_feature_proxy::ValidatorRewardsFeatureProxy)
            .get_unclaimed_rewards()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await
    }

    /// Claims as the owner, returns the amount the contract reports as sent.
    pub async fn claim_rewards(&mut self) -> RustBigUint {
        self.interactor
            .tx()
            .from(self.config.owner.address())
            .to(self.state.current_contract_address())
            .gas(10_000_000)
            .typed(validator_rewards_feature_proxy::ValidatorRewardsFeatureProxy)
            .claim_rewards()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await
    }

    pub async fn code_metadata(&mut self) -> Vec<u8> {
        let contract = self.contract_address();
        self.interactor
            .query()
            .to(&contract)
            .typed(validator_rewards_feature_proxy::ValidatorRewardsFeatureProxy)
            .get_code_metadata(&contract)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await
            .to_vec()
    }
}
