mod payable_interactor_cli;

use clap::Parser;
use multiversx_sc_snippets::imports::*;
use payable_features::payable_features_proxy;
use serde::{Deserialize, Serialize};

/// Payable Features Interact configuration
#[derive(Debug, Deserialize)]
pub struct Config {
    pub connection: ConnectionConfig,
    pub owner: WalletConfig,
    pub wallet: WalletConfig,
}

impl InteractorConfig for Config {
    fn connection(&self) -> &ConnectionConfig {
        &self.connection
    }

    fn register_wallets(&self) -> Vec<Wallet> {
        vec![self.owner.wallet().clone(), self.wallet.wallet().clone()]
    }
}

/// Payable Features Interact state
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct State {
    pub payable_features_address: Option<Bech32Address>,
}

impl State {
    /// Returns the payable features contract
    pub fn current_payable_features_address(&self) -> &Bech32Address {
        self.payable_features_address
            .as_ref()
            .expect("no known payable features contract, deploy first")
    }
}

const CODE_PATH: MxscPath = MxscPath::new("../output/payable-features.mxsc.json");

pub async fn payable_features_cli() {
    env_logger::init();

    let mut payable_interact = PayableInteract::new().await;

    let cli = payable_interactor_cli::InteractCli::parse();
    match &cli.command {
        Some(payable_interactor_cli::InteractCliCommand::Deploy) => {
            payable_interact.deploy().await;
        }
        Some(payable_interactor_cli::InteractCliCommand::AllTransfers) => {
            payable_interact.check_all_transfers().await;
        }
        Some(payable_interactor_cli::InteractCliCommand::MultiTransferWithOneEGLD) => {
            payable_interact
                .check_multi_transfer_only_egld_transfer()
                .await;
        }
        None => {}
    }
}

pub struct PayableInteract {
    pub interactor: Interactor,
    pub config: Config,
    pub state: AutoSave<State>,
}

impl PayableInteract {
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

    pub async fn deploy(&mut self) {
        let owner_address = self.config.owner.address();
        let new_address = self
            .interactor
            .tx()
            .from(&owner_address.clone())
            .gas(30_000_000)
            .typed(payable_features_proxy::PayableFeaturesProxy)
            .init()
            .code(CODE_PATH)
            .code_metadata(CodeMetadata::UPGRADEABLE)
            .returns(ReturnsNewBech32Address)
            .run()
            .await;

        println!("new address: {new_address}");
        self.state.payable_features_address = Some(new_address);
    }

    pub async fn check_multi_transfer_only_egld_transfer(&mut self) {
        let mut payment = MultiEgldOrEsdtPayment::new();
        payment.push(EgldOrEsdtTokenPayment::egld_payment(1_0000u64.into()));

        let result = self
            .interactor
            .tx()
            .from(self.config.wallet.address())
            .to(self.state.current_payable_features_address())
            .gas(6_000_000u64)
            .typed(payable_features_proxy::PayableFeaturesProxy)
            .payable_all_transfers()
            .payment(payment)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result:?}");
    }

    pub async fn check_all_transfers(&mut self) {
        let mut payment = MultiEgldOrEsdtPayment::new();
        payment.push(EgldOrEsdtTokenPayment::egld_payment(1_0000u64.into()));
        payment.push(EgldOrEsdtTokenPayment::egld_payment(2_0000u64.into()));

        let result = self
            .interactor
            .tx()
            .from(self.config.wallet.address())
            .to(self.state.current_payable_features_address())
            .gas(6_000_000u64)
            .typed(payable_features_proxy::PayableFeaturesProxy)
            .payable_all_transfers()
            .payment(payment)
            .returns(ReturnsResult)
            .run()
            .await;

        println!("Result: {result:?}");
    }
}
