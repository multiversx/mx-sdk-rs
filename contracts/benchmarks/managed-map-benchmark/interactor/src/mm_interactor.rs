#![allow(non_snake_case)]

pub mod config;
mod proxy;

use config::Config;
use multiversx_sc_snippets::imports::*;
use serde::{Deserialize, Serialize};
use std::{
    io::{Read, Write},
    path::Path,
};

const STATE_FILE: &str = "state.toml";

pub async fn managed_map_benchmark_cli() {
    env_logger::init();

    let mut args = std::env::args();
    let _ = args.next();
    let cmd = args.next().expect("at least one argument required");
    let config = Config::new();
    let mut interact = ContractInteract::new(config).await;
    match cmd.as_str() {
        "deploy" => interact.deploy().await,
        "mm_get" => interact.mm_get().await,
        "mm_contains" => interact.mm_contains().await,
        "mm_remove" => interact.mm_remove().await,
        _ => panic!("unknown command: {}", &cmd),
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct State {
    contract_address: Option<Bech32Address>,
    repeats: usize,
}

impl State {
    // Deserializes state from file
    pub fn load_state() -> Self {
        if Path::new(STATE_FILE).exists() {
            let mut file = std::fs::File::open(STATE_FILE).unwrap();
            let mut content = String::new();
            file.read_to_string(&mut content).unwrap();
            toml::from_str(&content).unwrap()
        } else {
            Self::default()
        }
    }

    /// Sets the contract address
    pub fn set_address(&mut self, address: Bech32Address) {
        self.contract_address = Some(address);
    }

    /// Returns the contract address
    pub fn current_address(&self) -> &Bech32Address {
        self.contract_address
            .as_ref()
            .expect("no known contract, deploy first")
    }
}

impl Drop for State {
    // Serializes state to file
    fn drop(&mut self) {
        let mut file = std::fs::File::create(STATE_FILE).unwrap();
        file.write_all(toml::to_string(self).unwrap().as_bytes())
            .unwrap();
    }
}

pub struct ContractInteract {
    interactor: Interactor,
    wallet_address: Address,
    contract_code: BytesValue,
    state: State,
}

impl ContractInteract {
    pub async fn new(config: Config) -> Self {
        let mut interactor = Interactor::new(config.gateway_uri())
            .await
            .use_chain_simulator(config.use_chain_simulator());

        interactor.set_current_dir_from_workspace("managed-map-benchmark");
        let wallet_address = interactor.register_wallet(test_wallets::alice()).await;

        // Useful in the chain simulator setting
        // generate blocks until ESDTSystemSCAddress is enabled
        interactor.generate_blocks_until_epoch(1).await.unwrap();

        let contract_code = BytesValue::interpret_from(
            "mxsc:../output/managed-map-benchmark.mxsc.json",
            &InterpreterContext::default(),
        );

        ContractInteract {
            interactor,
            wallet_address,
            contract_code,
            state: State::load_state(),
        }
    }

    pub async fn deploy(&mut self) {
        let new_address = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .gas(30_000_000u64)
            .typed(proxy::ManagedMapBenchmarkProxy)
            .init()
            .code(&self.contract_code)
            .returns(ReturnsNewBech32Address)
            .run()
            .await;
        self.state.set_address(new_address.clone());
        println!("new address: {new_address}");
    }

    pub async fn mm_get(&mut self) {
        let (result_value, gas_used) = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .typed(proxy::ManagedMapBenchmarkProxy)
            .mm_get("key0", self.state.repeats)
            .returns(ReturnsResultAs::<String>::new())
            .returns(ReturnsGasUsed)
            .run()
            .await;

        println!("Result: {result_value}");
        println!("Gas used: {gas_used}");
    }

    pub async fn mm_contains(&mut self) {
        let (result_value, gas_used) = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .typed(proxy::ManagedMapBenchmarkProxy)
            .mm_contains("key0", self.state.repeats)
            .returns(ReturnsResultUnmanaged)
            .returns(ReturnsGasUsed)
            .run()
            .await;

        println!("Result: {result_value}");
        println!("Gas used: {gas_used}");
    }

    pub async fn mm_remove(&mut self) {
        let gas_used = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(150_000_000u64)
            .typed(proxy::ManagedMapBenchmarkProxy)
            .mm_remove("key0", self.state.repeats)
            .returns(ReturnsGasUsed)
            .run()
            .await;

        println!("Gas used: {gas_used}");
    }
}
