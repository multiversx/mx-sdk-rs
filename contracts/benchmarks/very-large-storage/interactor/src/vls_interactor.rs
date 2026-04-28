mod vls_interactor_cli;
mod vls_interactor_config;
mod vls_interactor_state;

use clap::Parser;
use very_large_storage::very_large_storage_proxy;
pub use vls_interactor_config::Config;
use vls_interactor_state::State;

use multiversx_sc_snippets::imports::*;

const CODE_PATH: MxscPath = MxscPath::new("../output/very-large-storage.mxsc.json");

pub async fn very_large_storage_cli() {
    env_logger::init();

    let config = Config::load_config();

    let mut basic_interact = BasicInteract::new(config).await;

    let cli = vls_interactor_cli::InteractCli::parse();
    match &cli.command {
        Some(vls_interactor_cli::InteractCliCommand::Deploy) => {
            basic_interact.deploy().await;
        }
        Some(vls_interactor_cli::InteractCliCommand::Append(args)) => {
            basic_interact.append(args.num_bytes).await;
        }
        None => {}
    }
}

pub struct BasicInteract {
    pub interactor: Interactor,
    pub owner_address: Bech32Address,
    pub state: State,
}

impl BasicInteract {
    pub async fn new(config: Config) -> Self {
        let mut interactor = Interactor::new(config.gateway_uri())
            .await
            .use_chain_simulator(config.use_chain_simulator());
        interactor
            .set_current_dir_from_workspace("contracts/benchmarks/very-large-storage/interactor");

        let owner_address = interactor.register_wallet(test_wallets::mike()).await;

        interactor.generate_blocks(30u64).await.unwrap();

        BasicInteract {
            interactor,
            owner_address: owner_address.into(),
            state: State::load_state(),
        }
    }

    pub async fn deploy(&mut self) {
        let new_address = self
            .interactor
            .tx()
            .from(&self.owner_address)
            .gas(10_000_000u64)
            .typed(very_large_storage_proxy::VeryLargeStorageProxy)
            .init()
            .code(CODE_PATH)
            .returns(ReturnsNewBech32Address)
            .run()
            .await;

        println!("new address: {new_address}");
        self.state.set_contract_address(new_address);
    }

    pub async fn append(&mut self, num_bytes: u64) {
        let gas_used = self
            .interactor
            .tx()
            .from(&self.owner_address)
            .to(self.state.current_contract_address())
            .gas(SimulateGas)
            .typed(very_large_storage_proxy::VeryLargeStorageProxy)
            .append(num_bytes)
            .returns(ReturnsGasUsed)
            .run()
            .await;

        let x_len = self.get_x_len().await;
        println!(
            "Successfully appended {num_bytes} bytes (gas used: {gas_used}, total storage size: {x_len} bytes)"
        );
    }

    pub async fn get_x_len(&mut self) -> usize {
        self.interactor
            .query()
            .to(self.state.current_contract_address())
            .typed(very_large_storage_proxy::VeryLargeStorageProxy)
            .x_len()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await
    }
}
