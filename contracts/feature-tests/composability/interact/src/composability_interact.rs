mod composability_interact_cli;
mod composability_interact_config;
mod composability_interact_state;

use clap::Parser;
use composability_interact_config::Config;
use composability_interact_state::State;
use forwarder_raw::ProxyTrait as ForwarderRawProxyTrait;
use multiversx_sc_snippets::{
    env_logger,
    erdrs::wallet::Wallet,
    multiversx_sc::{
        codec::multi_types::OptionalValue,
        types::{Address, BoxedBytes, CodeMetadata},
    },
    multiversx_sc_scenario::{
        bech32,
        scenario_format::interpret_trait::InterpreterContext,
        scenario_model::{IntoBlockchainCall, TxExpect},
        ContractInfo, DebugApi,
    },
    tokio, Interactor,
};
use promises_features::ProxyTrait as PromisesProxyTrait;
use vault::ProxyTrait as VaultProxyTrait;

#[tokio::main]
async fn main() {
    DebugApi::dummy();
    env_logger::init();

    let mut composability_interact = ComposabilityInteract::init().await;

    let cli = composability_interact_cli::InteractCli::parse();
    match &cli.command {
        Some(composability_interact_cli::InteractCliCommand::DeployVault) => {
            composability_interact.deploy_vault().await;
        },
        Some(composability_interact_cli::InteractCliCommand::DeployForwarderRaw) => {
            composability_interact.deploy_forwarder_raw().await;
        },
        Some(composability_interact_cli::InteractCliCommand::DeployPromises) => {
            composability_interact.deploy_promises().await;
        },
        None => {},
    }
}

struct ComposabilityInteract {
    interactor: Interactor,
    wallet_address: Address,
    state: State,
}

impl ComposabilityInteract {
    async fn init() -> Self {
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

    async fn deploy_vault(&mut self) {
        let deploy_result: multiversx_sc_snippets::InteractorResult<OptionalValue<BoxedBytes>> =
            self.interactor
                .sc_deploy(
                    self.state
                        .default_vault_address()
                        .init(OptionalValue::<BoxedBytes>::None)
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
        self.state.set_vault_address(&new_address_expr);
    }

    async fn deploy_forwarder_raw(&mut self) {
        let deploy_result: multiversx_sc_snippets::InteractorResult<()> = self
            .interactor
            .sc_deploy(
                self.state
                    .default_forwarder_raw_address()
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
        self.state.set_forwarder_raw_address(&new_address_expr);
    }

    async fn deploy_promises(&mut self) {
        let deploy_result: multiversx_sc_snippets::InteractorResult<()> = self
            .interactor
            .sc_deploy(
                self.state
                    .default_promises_address()
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
        self.state.set_promises_address(&new_address_expr);
    }
}
