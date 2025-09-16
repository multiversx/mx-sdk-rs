mod payable_interactor_cli;
mod payable_interactor_config;
mod payable_interactor_state;

use clap::Parser;
use payable_features::payable_features_proxy;
pub use payable_interactor_config::Config;
use payable_interactor_state::State;

use multiversx_sc_snippets::imports::*;

const CODE_PATH: MxscPath = MxscPath::new("../output/payable-features.mxsc.json");

pub async fn payable_features_cli() {
    env_logger::init();

    let config = Config::load_config();

    let mut payable_interact = PayableInteract::new(config).await;

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
    pub sc_owner_address: Bech32Address,
    pub wallet_address: Bech32Address,
    pub state: State,
}

impl PayableInteract {
    pub async fn new(config: Config) -> Self {
        let mut interactor = Interactor::new(config.gateway_uri())
            .await
            .use_chain_simulator(config.use_chain_simulator());

        interactor
            .set_current_dir_from_workspace("contracts/feature-tests/payable-features/interactor");

        let sc_owner_address = interactor.register_wallet(test_wallets::heidi()).await;
        let wallet_address = interactor.register_wallet(test_wallets::ivan()).await;

        interactor.generate_blocks(30u64).await.unwrap();

        PayableInteract {
            interactor,
            sc_owner_address: sc_owner_address.into(),
            wallet_address: wallet_address.into(),
            state: State::load_state(),
        }
    }

    pub async fn deploy(&mut self) {
        let new_address = self
            .interactor
            .tx()
            .from(&self.sc_owner_address.clone())
            .gas(30_000_000)
            .typed(payable_features_proxy::PayableFeaturesProxy)
            .init()
            .code(CODE_PATH)
            .code_metadata(CodeMetadata::UPGRADEABLE)
            .returns(ReturnsNewBech32Address)
            .run()
            .await;

        println!("new address: {new_address}");
        self.state.set_payable_features_address(new_address);
    }

    pub async fn check_multi_transfer_only_egld_transfer(&mut self) {
        let mut payment = MultiEgldOrEsdtPayment::new();
        payment.push(EgldOrEsdtTokenPayment::egld_payment(1_0000u64.into()));

        let result = self
            .interactor
            .tx()
            .from(&self.wallet_address)
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
            .from(&self.wallet_address)
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
