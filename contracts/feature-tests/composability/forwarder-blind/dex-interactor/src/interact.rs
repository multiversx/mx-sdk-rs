mod config;
mod interact_cli;
pub mod pair_proxy;
mod state;
pub mod wegld_proxy;

use clap::Parser;
pub use config::Config;
use forwarder_blind::forwarder_blind_proxy;
use multiversx_sc_snippets::imports::*;
use state::State;

const FORWARDER_BLIND_CODE_PATH: MxscPath = MxscPath::new("../output/forwarder-blind.mxsc.json");

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
        Some(interact_cli::InteractCliCommand::SwapWegldForUsdc(args)) => {
            interact
                .swap_wegld_for_usdc(args.wegld_amount, args.usdc_amount_min)
                .await;
        }
        Some(interact_cli::InteractCliCommand::SwapUsdcForWegld(args)) => {
            interact
                .swap_usdc_for_wegld(args.usdc_amount, args.wegld_amount_min)
                .await;
        }
        Some(interact_cli::InteractCliCommand::GetRate(args)) => {
            interact.get_rate(args.wegld_amount).await;
        }
        Some(interact_cli::InteractCliCommand::GetLiquidity) => {
            interact.get_liquidity().await;
        }
        None => {}
    }
}

pub struct ContractInteract {
    pub interactor: Interactor,
    pub wallet_address: Bech32Address,
    pub config: Config,
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
            config,
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
            .to(&self.config.wegld_address)
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

    pub async fn swap_wegld_for_usdc(&mut self, wegld_amount: u64, usdc_amount_min: u64) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(&self.config.pair_address)
            .gas(50_000_000u64)
            .typed(pair_proxy::PairProxy)
            .swap_tokens_fixed_input(
                TokenIdentifier::from(self.config.usdc_token_id.as_str()),
                BigUint::from(usdc_amount_min),
            )
            .payment(Payment::new(
                TokenId::from(self.config.wegld_token_id.as_str()),
                0,
                NonZeroBigUint::new(BigUint::from(wegld_amount)).expect("Amount must be > 0"),
            ))
            .returns(ReturnsResult)
            .run()
            .await;

        println!("USDC received: {response}");
    }

    pub async fn swap_usdc_for_wegld(&mut self, usdc_amount: u64, wegld_amount_min: u64) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(&self.config.pair_address)
            .gas(50_000_000u64)
            .typed(pair_proxy::PairProxy)
            .swap_tokens_fixed_input(
                TokenIdentifier::from(self.config.wegld_token_id.as_str()),
                BigUint::from(wegld_amount_min),
            )
            .payment(Payment::new(
                TokenId::from(self.config.usdc_token_id.as_str()),
                0,
                NonZeroBigUint::new(BigUint::from(usdc_amount)).expect("Amount must be > 0"),
            ))
            .returns(ReturnsResult)
            .run()
            .await;

        println!("WEGLD received: {response}");
    }

    pub async fn get_rate(&mut self, wegld_amount: u64) {
        let amount_out = self
            .interactor
            .query()
            .to(&self.config.pair_address)
            .typed(pair_proxy::PairProxy)
            .get_amount_out_view(
                TokenIdentifier::from(self.config.wegld_token_id.as_str()),
                BigUint::from(wegld_amount),
            )
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!(
            "{wegld_amount} {} -> {} {}",
            self.config.wegld_token_id, amount_out, self.config.usdc_token_id
        );
    }

    pub async fn get_liquidity(&mut self) {
        let (wegld_reserve, usdc_reserve, lp_supply) = self
            .interactor
            .query()
            .to(&self.config.pair_address)
            .typed(pair_proxy::PairProxy)
            .get_reserves_and_total_supply()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await
            .into_tuple();

        println!("{} reserve: {wegld_reserve}", self.config.wegld_token_id);
        println!("{} reserve: {usdc_reserve}", self.config.usdc_token_id);
        println!("LP token supply: {lp_supply}");
    }
}
