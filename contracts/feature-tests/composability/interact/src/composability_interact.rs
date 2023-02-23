mod composability_interact_cli;

use clap::Parser;
use multiversx_sc_snippets::{
    env_logger,
    erdrs::wallet::Wallet,
    multiversx_sc::{codec::multi_types::{OptionalValue}, types::{Address, CodeMetadata}},
    multiversx_sc_scenario::{bech32, ContractInfo, DebugApi, scenario_model::{TxExpect, IntoBlockchainCall}, scenario_format::interpret_trait::InterpreterContext},
    tokio, Interactor,
};
use forwarder_raw::ProxyTrait as ForwarderRawProxyTrait;
use promises_features::ProxyTrait as PromisesProxyTrait;
use vault::ProxyTrait as VaultProxyTrait;


use std::io::{Read, Write};

const GATEWAY: &str = multiversx_sc_snippets::erdrs::blockchain::TESTNET_GATEWAY;
const PEM: &str = "alice.pem";
const DEFAULT_COMPOSABILITY_ADDRESS_EXPR: &str =
    "0x0000000000000000000000000000000000000000000000000000000000000000";
const SYSTEM_SC_BECH32: &str = "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u";
const SAVED_ADDRESS_FILE_NAME: &str = "composability_address.txt";

type VaultContract = ContractInfo<vault::Proxy<DebugApi>>;
type ForwarderRawContract = ContractInfo<forwarder_raw::Proxy<DebugApi>>;
type PromisesContract = ContractInfo<promises_features::Proxy<DebugApi>>;

#[tokio::main]
async fn main() {
    DebugApi::dummy();
    env_logger::init();

    let mut state = State::init().await;

    let cli = composability_interact_cli::InteractCli::parse();
    match &cli.command {
        Some(composability_interact_cli::InteractCliCommand::DeployVault) => {
            state.deploy_vault().await;
        },
        Some(composability_interact_cli::InteractCliCommand::DeployForwarderRaw) => {
            state.deploy_forwarder_raw().await;
        },
        Some(composability_interact_cli::InteractCliCommand::DeployPromises) => {
            state.deploy_promises().await;
        },
        None => {},
    }
}

struct State {
    interactor: Interactor,
    wallet_address: Address,
    vault: VaultContract,
    forwarder_raw: ForwarderRawContract,
    promises: PromisesContract,
    system_sc_address: Address,
    collection_token_identifier: String,
}

impl State {
    async fn init() -> Self {
        let mut interactor = Interactor::new(GATEWAY).await;
        let wallet_address = interactor.register_wallet(Wallet::from_pem_file(PEM).unwrap());
        let vault = VaultContract::new(load_address_expr());
        let forwarder_raw = ForwarderRawContract::new(load_address_expr());
        let promises = PromisesContract::new(load_address_expr());

        State {
            interactor,
            wallet_address,
            vault,
            forwarder_raw,
            promises,
            system_sc_address: bech32::decode(SYSTEM_SC_BECH32),
            collection_token_identifier: "".to_owned(),
        }
    }

    async fn deploy_vault(&mut self) {
        let deploy_result: multiversx_sc_snippets::InteractorResult<()> = self
            .interactor
            .sc_deploy(
                self.vault
                    .init(OptionalValue::None)
                    .into_blockchain_call()
                    .from(&self.wallet_address)
                    .code_metadata(CodeMetadata::all())
                    .contract_code(
                        "file:../../vault/output/vault.wasm",
                        &InterpreterContext::default(),
                    )
                    .gas_limit("70,000,000")
                    .expect(TxExpect::ok()),
            )
            .await;
        let new_address = deploy_result.new_deployed_address();
        let new_address_bech32 = bech32::encode(&new_address);
        println!("Vault address: {new_address_bech32}");
        let new_address_expr = format!("bech32:{new_address_bech32}");
        save_address_expr(new_address_expr.as_str());
        self.vault = VaultContract::new(new_address_expr);
    }

    async fn deploy_forwarder_raw(&mut self) {
        let deploy_result: multiversx_sc_snippets::InteractorResult<()> = self
            .interactor
            .sc_deploy(
                self.forwarder_raw
                    .init()
                    .into_blockchain_call()
                    .from(&self.wallet_address)
                    .code_metadata(CodeMetadata::all())
                    .contract_code(
                        "file:../../forwarder-raw/output/forwarder-raw.wasm",
                        &InterpreterContext::default(),
                    )
                    .gas_limit("70,000,000")
                    .expect(TxExpect::ok()),
            )
            .await;
        let new_address = deploy_result.new_deployed_address();
        let new_address_bech32 = bech32::encode(&new_address);
        println!("Forwarder Raw address: {new_address_bech32}");
        let new_address_expr = format!("bech32:{new_address_bech32}");
        save_address_expr(new_address_expr.as_str());
        self.vault = VaultContract::new(new_address_expr);
    }

    async fn deploy_promises(&mut self) {
        let deploy_result: multiversx_sc_snippets::InteractorResult<()> = self
            .interactor
            .sc_deploy(
                self.promises
                    .init()
                    .into_blockchain_call()
                    .from(&self.wallet_address)
                    .code_metadata(CodeMetadata::all())
                    .contract_code(
                        "file:../../promises/output/promises.wasm",
                        &InterpreterContext::default(),
                    )
                    .gas_limit("70,000,000")
                    .expect(TxExpect::ok()),
            )
            .await;
        let new_address = deploy_result.new_deployed_address();
        let new_address_bech32 = bech32::encode(&new_address);
        println!("Promises address: {new_address_bech32}");
        let new_address_expr = format!("bech32:{new_address_bech32}");
        save_address_expr(new_address_expr.as_str());
        self.vault = VaultContract::new(new_address_expr);
    }

}

fn load_address_expr() -> String {
    match std::fs::File::open(SAVED_ADDRESS_FILE_NAME) {
        Ok(mut file) => {
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            contents
        },
        Err(_) => DEFAULT_COMPOSABILITY_ADDRESS_EXPR.to_string(),
    }
}

fn save_address_expr(address_expr: &str) {
    let mut file = std::fs::File::create(SAVED_ADDRESS_FILE_NAME).unwrap();
    file.write_all(address_expr.as_bytes()).unwrap();
}
