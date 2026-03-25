mod config;
mod interact_cli;
pub mod proxies;
mod state;

use clap::Parser;
pub use config::Config;
use multiversx_sc_snippets::imports::*;
use proxies::*;
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
        Some(interact_cli::InteractCliCommand::Swap1(args)) => match &args.method {
            interact_cli::SwapWegldForUsdcMethod::Direct(args) => {
                interact
                    .swap1_direct(args.wegld_amount, args.usdc_amount_min)
                    .await;
            }
            interact_cli::SwapWegldForUsdcMethod::Sync(args) => {
                interact
                    .swap1_sync(args.wegld_amount, args.usdc_amount_min)
                    .await;
            }
            interact_cli::SwapWegldForUsdcMethod::Async1(args) => {
                interact
                    .swap1_async1(args.wegld_amount, args.usdc_amount_min)
                    .await;
            }
            interact_cli::SwapWegldForUsdcMethod::Async2(args) => {
                interact
                    .swap1_async2(args.wegld_amount, args.usdc_amount_min)
                    .await;
            }
            interact_cli::SwapWegldForUsdcMethod::Te(args) => {
                interact
                    .swap1_te(args.wegld_amount, args.usdc_amount_min)
                    .await;
            }
        },
        Some(interact_cli::InteractCliCommand::Swap2(args)) => match &args.method {
            interact_cli::SwapUsdcForWegldMethod::Direct(args) => {
                interact
                    .swap2_direct(args.usdc_amount, args.wegld_amount_min)
                    .await;
            }
            interact_cli::SwapUsdcForWegldMethod::Sync(args) => {
                interact
                    .swap2_sync(args.usdc_amount, args.wegld_amount_min)
                    .await;
            }
            interact_cli::SwapUsdcForWegldMethod::Async1(args) => {
                interact
                    .swap2_async1(args.usdc_amount, args.wegld_amount_min)
                    .await;
            }
            interact_cli::SwapUsdcForWegldMethod::Async2(args) => {
                interact
                    .swap2_async2(args.usdc_amount, args.wegld_amount_min)
                    .await;
            }
            interact_cli::SwapUsdcForWegldMethod::Te(args) => {
                interact
                    .swap2_te(args.usdc_amount, args.wegld_amount_min)
                    .await;
            }
        },
        Some(interact_cli::InteractCliCommand::GetRate(args)) => {
            interact.get_rate(args.wegld_amount).await;
        }
        Some(interact_cli::InteractCliCommand::GetLiquidity) => {
            interact.get_liquidity().await;
        }
        Some(interact_cli::InteractCliCommand::Drain) => {
            interact.drain().await;
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

        let wallet_address = interactor.register_wallet(test_wallets::simon()).await;

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
            .gas(SimulateGas)
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

        println!("Wrapping complete: status={status:?}, gas_used={gas_used:?}");
    }

    fn build_swap_function_call(
        &mut self,
        token_id: &str,
        amount_min: u64,
    ) -> FunctionCall<StaticApi> {
        self.interactor
            .tx()
            .typed(pair_proxy::PairProxy)
            .swap_tokens_fixed_input(EsdtTokenIdentifier::from(token_id), amount_min)
            .into_function_call()
    }

    pub async fn swap1_direct(&mut self, wegld_amount: u64, usdc_amount_min: u64) {
        let swap_function_call =
            self.build_swap_function_call(&self.config.usdc_token_id.clone(), usdc_amount_min);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(&self.config.pair_address)
            .gas(50_000_000u64)
            .raw_data(swap_function_call)
            .payment(
                Payment::try_new(&self.config.wegld_token_id, 0, wegld_amount)
                    .expect("Amount must be > 0"),
            )
            .original_result::<EsdtTokenPayment<StaticApi>>()
            .returns(ReturnsResult)
            .run()
            .await;

        println!("USDC received: {response}");
    }

    pub async fn swap2_direct(&mut self, usdc_amount: u64, wegld_amount_min: u64) {
        let swap_function_call =
            self.build_swap_function_call(&self.config.wegld_token_id.clone(), wegld_amount_min);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(&self.config.pair_address)
            .gas(50_000_000u64)
            .raw_data(swap_function_call)
            .payment(
                Payment::try_new(&self.config.usdc_token_id, 0, usdc_amount)
                    .expect("Amount must be > 0"),
            )
            .returns(ReturnsRawResult)
            .run()
            .await;

        let first = response.get(0).clone();
        let payment = Payment::<StaticApi>::top_decode(first).unwrap();
        println!("WEGLD received: {payment:?}");
    }

    pub async fn swap1_sync(&mut self, wegld_amount: u64, usdc_amount_min: u64) {
        let swap_function_call =
            self.build_swap_function_call(&self.config.usdc_token_id.clone(), usdc_amount_min);

        let (status, gas_used) = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(70_000_000u64)
            .typed(forwarder_blind_proxy::ForwarderBlindProxy)
            .blind_sync(&self.config.pair_address, swap_function_call)
            .payment(
                Payment::try_new(&self.config.wegld_token_id, 0, wegld_amount)
                    .expect("Amount must be > 0"),
            )
            .returns(ReturnsStatus)
            .returns(ReturnsGasUsed)
            .run()
            .await;

        println!("swap via forwarder: status={status:?}, gas_used={gas_used:?}");
    }

    pub async fn swap2_sync(&mut self, usdc_amount: u64, wegld_amount_min: u64) {
        let swap_function_call =
            self.build_swap_function_call(&self.config.wegld_token_id.clone(), wegld_amount_min);

        let (status, gas_used) = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(70_000_000u64)
            .typed(forwarder_blind_proxy::ForwarderBlindProxy)
            .blind_sync(&self.config.pair_address, swap_function_call)
            .payment(
                Payment::try_new(&self.config.usdc_token_id, 0, usdc_amount)
                    .expect("Amount must be > 0"),
            )
            .returns(ReturnsStatus)
            .returns(ReturnsGasUsed)
            .run()
            .await;

        println!("swap via forwarder: status={status:?}, gas_used={gas_used:?}");
    }

    pub async fn swap1_async1(&mut self, wegld_amount: u64, usdc_amount_min: u64) {
        let swap_function_call =
            self.build_swap_function_call(&self.config.usdc_token_id.clone(), usdc_amount_min);

        let (status, gas_used) = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(70_000_000u64)
            .typed(forwarder_blind_proxy::ForwarderBlindProxy)
            .blind_async_v1(&self.config.pair_address, swap_function_call)
            .payment(
                Payment::try_new(&self.config.wegld_token_id, 0, wegld_amount)
                    .expect("Amount must be > 0"),
            )
            .returns(ReturnsStatus)
            .returns(ReturnsGasUsed)
            .run()
            .await;

        println!("swap via forwarder: status={status:?}, gas_used={gas_used:?}");
    }

    pub async fn swap1_async2(&mut self, wegld_amount: u64, usdc_amount_min: u64) {
        let swap_function_call =
            self.build_swap_function_call(&self.config.usdc_token_id.clone(), usdc_amount_min);

        let (status, gas_used) = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(70_000_000u64)
            .typed(forwarder_blind_proxy::ForwarderBlindProxy)
            .blind_async_v2(&self.config.pair_address, swap_function_call)
            .payment(
                Payment::try_new(&self.config.wegld_token_id, 0, wegld_amount)
                    .expect("Amount must be > 0"),
            )
            .returns(ReturnsStatus)
            .returns(ReturnsGasUsed)
            .run()
            .await;

        println!("swap via forwarder: status={status:?}, gas_used={gas_used:?}");
    }

    pub async fn swap1_te(&mut self, wegld_amount: u64, usdc_amount_min: u64) {
        let swap_function_call =
            self.build_swap_function_call(&self.config.usdc_token_id.clone(), usdc_amount_min);

        let (status, gas_used) = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(70_000_000u64)
            .typed(forwarder_blind_proxy::ForwarderBlindProxy)
            .blind_transf_exec(&self.config.pair_address, swap_function_call)
            .payment(
                Payment::try_new(&self.config.wegld_token_id, 0, wegld_amount)
                    .expect("Amount must be > 0"),
            )
            .returns(ReturnsStatus)
            .returns(ReturnsGasUsed)
            .run()
            .await;

        println!("swap via forwarder: status={status:?}, gas_used={gas_used:?}");
    }

    pub async fn swap2_async1(&mut self, usdc_amount: u64, wegld_amount_min: u64) {
        let swap_function_call =
            self.build_swap_function_call(&self.config.wegld_token_id.clone(), wegld_amount_min);

        let (status, gas_used) = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(70_000_000u64)
            .typed(forwarder_blind_proxy::ForwarderBlindProxy)
            .blind_async_v1(&self.config.pair_address, swap_function_call)
            .payment(
                Payment::try_new(&self.config.usdc_token_id, 0, usdc_amount)
                    .expect("Amount must be > 0"),
            )
            .returns(ReturnsStatus)
            .returns(ReturnsGasUsed)
            .run()
            .await;

        println!("swap via forwarder: status={status:?}, gas_used={gas_used:?}");
    }

    pub async fn swap2_async2(&mut self, usdc_amount: u64, wegld_amount_min: u64) {
        let swap_function_call =
            self.build_swap_function_call(&self.config.wegld_token_id.clone(), wegld_amount_min);

        let (status, gas_used) = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(70_000_000u64)
            .typed(forwarder_blind_proxy::ForwarderBlindProxy)
            .blind_async_v2(&self.config.pair_address, swap_function_call)
            .payment(
                Payment::try_new(&self.config.usdc_token_id, 0, usdc_amount)
                    .expect("Amount must be > 0"),
            )
            .returns(ReturnsStatus)
            .returns(ReturnsGasUsed)
            .run()
            .await;

        println!("swap via forwarder: status={status:?}, gas_used={gas_used:?}");
    }

    pub async fn swap2_te(&mut self, usdc_amount: u64, wegld_amount_min: u64) {
        let swap_function_call =
            self.build_swap_function_call(&self.config.wegld_token_id.clone(), wegld_amount_min);

        let (status, gas_used) = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(70_000_000u64)
            .typed(forwarder_blind_proxy::ForwarderBlindProxy)
            .blind_transf_exec(&self.config.pair_address, swap_function_call)
            .payment(
                Payment::try_new(&self.config.usdc_token_id, 0, usdc_amount)
                    .expect("Amount must be > 0"),
            )
            .returns(ReturnsStatus)
            .returns(ReturnsGasUsed)
            .run()
            .await;

        println!("swap via forwarder: status={status:?}, gas_used={gas_used:?}");
    }

    pub async fn drain(&mut self) {
        let contract_esdt = self
            .interactor
            .get_account_esdt(&self.state.current_address().to_address())
            .await;

        for token_id in [
            self.config.wegld_token_id.clone(),
            self.config.usdc_token_id.clone(),
        ] {
            let balance = contract_esdt
                .get(&token_id)
                .map(|b| b.balance.parse::<u128>().unwrap_or(0))
                .unwrap_or(0);

            if balance == 0 {
                println!("Drain {token_id}: no balance, skipping");
                continue;
            }

            println!("Drain {token_id}: balance={balance}");

            let (status, gas_used) = self
                .interactor
                .tx()
                .from(&self.wallet_address)
                .to(self.state.current_address())
                .gas(10_000_000u64)
                .typed(forwarder_blind_proxy::ForwarderBlindProxy)
                .drain(token_id.as_str(), 0u64)
                .returns(ReturnsStatus)
                .returns(ReturnsGasUsed)
                .run()
                .await;

            println!("Drain {token_id}: status={status:?}, gas_used={gas_used:?}");
        }
    }

    pub async fn get_rate(&mut self, wegld_amount: u64) {
        let amount_out = self
            .interactor
            .query()
            .to(&self.config.pair_address)
            .typed(pair_proxy::PairProxy)
            .get_amount_out_view(
                EsdtTokenIdentifier::from(self.config.wegld_token_id.as_str()),
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
