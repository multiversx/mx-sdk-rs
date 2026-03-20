mod interact_cli;
mod config;
mod state;
pub mod wegld_proxy;

use clap::Parser;
use forwarder_blind::forwarder_blind_proxy;
use multiversx_sc_snippets::imports::*;
pub use config::Config;
use state::State;

const FORWARDER_BLIND_CODE_PATH: MxscPath =
    MxscPath::new("../output/forwarder-blind.mxsc.json");

pub async fn forwarder_blind_cli() {
    env_logger::init();

    let config = Config::load_config();
    let mut interact = ContractInteract::new(config).await;

    let cli = interact_cli::InteractCli::parse();
    match &cli.command {
        Some(interact_cli::InteractCliCommand::Deploy) => {
            interact.deploy().await;
        }
        Some(interact_cli::InteractCliCommand::WrapEgld(args)) => {
            interact.wrap_egld(args.amount).await;
        }
        None => {}
    }
}

pub struct ContractInteract {
    pub interactor: Interactor,
    pub wallet_address: Bech32Address,
    pub wegld_address: Bech32Address,
    pub state: State,
}

impl ContractInteract {
    pub async fn new(config: Config) -> Self {
        let mut interactor = Interactor::new(config.gateway_uri())
            .await
            .use_chain_simulator(config.use_chain_simulator());
        interactor.set_current_dir_from_workspace(
            "contracts/feature-tests/composability/forwarder-blind/interactor",
        );

        let wallet_address = interactor.register_wallet(test_wallets::judy()).await;

        interactor.generate_blocks_until_all_activations().await;

        ContractInteract {
            interactor,
            wallet_address: wallet_address.into(),
            wegld_address: config.wegld_address,
            state: State::load_state(),
        }
    }

    pub async fn deploy(&mut self) {
        let new_address = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .gas(30_000_000u64)
            .typed(forwarder_blind_proxy::ForwarderBlindProxy)
            .init()
            .code(FORWARDER_BLIND_CODE_PATH)
            .returns(ReturnsNewBech32Address)
            .run()
            .await;

        println!("new address: {new_address}");
        self.state.set_contract_address(new_address);
    }

    pub async fn wrap_egld(&mut self, amount: u64) {
        let (status, gas_used) = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(&self.wegld_address)
            .gas(5_000_000)
            .typed(wegld_proxy::EgldEsdtSwapProxy)
            .wrap_egld()
            .egld(amount)
            .returns(ReturnsStatus)
            .returns(ReturnsGasUsed)
            .run()
            .await;

        println!("WEGLD received: status={status:?}, gas_used={gas_used:?}");
    }
}
