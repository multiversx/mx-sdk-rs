mod adder_interact_cli;

use clap::Parser;
use multiversx_sc_snippets::{
    env_logger,
    multiversx_sc::types::Address,
    multiversx_sc_scenario::{test_wallets, ContractInfo, DebugApi},
    tokio, Interactor,
};

#[allow(dead_code)]
/// Default adder address
const DEFAULT_ADDER_ADDRESS: &str =
    "0x0000000000000000000000000000000000000000000000000000000000000000";

// Gateway url
const GATEWAY_URL: &str = "https://testnet-gateway.multiversx.com";

pub type AdderContract = ContractInfo<adder::Proxy<DebugApi>>;

#[tokio::main]
async fn main() {
    DebugApi::dummy();
    env_logger::init();

    let mut adder_interact = AdderInteract::init().await;

    let cli = adder_interact_cli::InteractCli::parse();
    match &cli.command {
        Some(adder_interact_cli::InteractCliCommand::Add(args)) => {
            adder_interact.add(args.value).await;
        },
        Some(adder_interact_cli::InteractCliCommand::Deploy) => {
            adder_interact.deploy().await;
        },
        Some(adder_interact_cli::InteractCliCommand::Sum) => {
            adder_interact.sum().await;
        },
        None => {},
    }
}

#[allow(unused)]
struct AdderInteract {
    interactor: Interactor,
    wallet_address: Address,
}

impl AdderInteract {
    async fn init() -> Self {
        let mut interactor = Interactor::new(GATEWAY_URL).await;
        let wallet_address = interactor.register_wallet(test_wallets::mike());

        Self {
            interactor,
            wallet_address,
        }
    }

    async fn deploy(&mut self) {}

    async fn add(&mut self, _value: u8) {}

    async fn sum(&mut self) {}
}
