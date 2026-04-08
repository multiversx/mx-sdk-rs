mod config;
mod interact_cli;
pub mod proxies;
mod state;

use clap::Parser;
pub use config::Config;
use multiversx_sc_snippets::imports::*;
use proxies::*;
use state::State;

const FORWARDER_BLIND_CODE_PATH: FilePath = FilePath("forwarder-blind-bon.wasm");

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
        Some(interact_cli::InteractCliCommand::Balances) => {
            interact.balances().await;
        }
        None => {}
    }
}

#[derive(Copy, Clone)]
enum ForwarderMethod {
    Sync,
    AsyncV1,
    AsyncV2,
    TransfExec,
}

pub struct ContractInteract {
    pub interactor: Interactor,
    pub wallet_addresses: Vec<Bech32Address>,
    pub config: Config,
    pub state: State,
}

impl ContractInteract {
    pub async fn new(config: Config) -> Self {
        let mut interactor = Interactor::new(config.gateway_uri())
            .await
            .use_chain_simulator(config.use_chain_simulator());
        interactor.set_current_dir_from_workspace(
            "contracts/feature-tests/composability/forwarder-blind/dex-interactor",
        );

        let wallet_addresses: Vec<Bech32Address> = if config.wallet_pem_paths.is_empty() {
            println!("WARNING: no wallet_pem_paths configured — all operations will be skipped.");
            Vec::new()
        } else {
            let mut addrs = Vec::new();
            for pem_path in &config.wallet_pem_paths {
                let wallet = Wallet::from_pem_file(pem_path)
                    .unwrap_or_else(|e| panic!("failed to load wallet from {pem_path}: {e}"));
                addrs.push(interactor.register_wallet(wallet).await.into());
            }
            addrs
        };

        interactor.generate_blocks_until_all_activations().await;

        ContractInteract {
            interactor,
            wallet_addresses,
            config,
            state: State::load_state(),
        }
    }

    pub async fn deploy(&mut self) {
        let wallet_addresses = self.wallet_addresses.clone();
        let mut buffer = self.interactor.homogenous_call_buffer();
        for wallet in &wallet_addresses {
            buffer.push_tx(|tx| {
                tx.from(wallet)
                    .gas(80_000_000u64)
                    .typed(forwarder_blind_proxy::ForwarderBlindProxy)
                    .init()
                    .code(FORWARDER_BLIND_CODE_PATH)
                    .code_metadata(CodeMetadata::PAYABLE)
                    .returns(ReturnsNewBech32Address)
            });
        }
        let new_addresses: Vec<Bech32Address> = buffer.run().await;
        for (i, addr) in new_addresses.iter().enumerate() {
            println!("new address (wallet {i}): {addr}");
        }
        self.state.set_contract_addresses(new_addresses);
    }

    pub async fn wrap_egld(&mut self, amount: u64) {
        let wallet_addresses = self.wallet_addresses.clone();
        let wegld_address = self.config.wegld_address.clone();
        let mut buffer = self.interactor.homogenous_call_buffer();
        for wallet in &wallet_addresses {
            buffer.push_tx(|tx| {
                tx.from(wallet)
                    .to(&wegld_address)
                    .gas(5_000_000u64)
                    .typed(wegld_proxy::EgldEsdtSwapProxy)
                    .wrap_egld()
                    .egld(amount)
                    .returns(ReturnsStatus)
                    .returns(ReturnsGasUsed)
            });
        }
        let results: Vec<(u64, u64)> = buffer.run().await;
        for (wallet, (status, gas_used)) in wallet_addresses.iter().zip(results.iter()) {
            println!(
                "Wrapping complete (wallet: {wallet}): status={status:?}, gas_used={gas_used:?}"
            );
        }
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

    async fn swap_direct_impl(
        &mut self,
        send_token_id: String,
        send_amount: u64,
        want_token_id: String,
        want_amount_min: u64,
    ) {
        for wallet in self.wallet_addresses.clone() {
            let fc = self.build_swap_function_call(&want_token_id, want_amount_min);
            let response = self
                .interactor
                .tx()
                .from(&wallet)
                .to(&self.config.pair_address)
                .gas(50_000_000u64)
                .raw_data(fc)
                .payment(
                    Payment::try_new(&send_token_id, 0, send_amount).expect("Amount must be > 0"),
                )
                .original_result::<EsdtTokenPayment<StaticApi>>()
                .returns(ReturnsResult)
                .run()
                .await;

            println!("{want_token_id} received (wallet: {wallet}): {response}");
        }
    }

    async fn swap_via_forwarder_impl(
        &mut self,
        send_token_id: String,
        send_amount: u64,
        want_token_id: String,
        want_amount_min: u64,
        method: ForwarderMethod,
    ) {
        let wallet_addresses = self.wallet_addresses.clone();
        let contract_addresses = self.config.contract_addresses.clone();
        let pair_address = self.config.pair_address.clone();
        let fc = self.build_swap_function_call(&want_token_id, want_amount_min);

        let mut buffer = self.interactor.homogenous_call_buffer();
        for wallet in &wallet_addresses {
            for contract in &contract_addresses {
                if matches!(method, ForwarderMethod::Sync) {
                    let contract_shard = contract.as_address().shard_of_3();
                    let pair_shard = pair_address.as_address().shard_of_3();
                    if contract_shard != pair_shard {
                        println!(
                            "WARNING: skipping swap from {wallet} to {contract} due to incompatible shard \
                            (contract shard: {contract_shard}, pair shard: {pair_shard})"
                        );
                        continue;
                    }
                }
                buffer.push_tx(|tx| {
                    let typed = tx
                        .from(wallet)
                        .to(contract)
                        .gas(70_000_000u64)
                        .typed(forwarder_blind_proxy::ForwarderBlindProxy);
                    match method {
                        ForwarderMethod::Sync => typed.blind_sync(&pair_address, fc.clone()),
                        ForwarderMethod::AsyncV1 => typed.blind_async_v1(&pair_address, fc.clone()),
                        ForwarderMethod::AsyncV2 => typed.blind_async_v2(&pair_address, fc.clone()),
                        ForwarderMethod::TransfExec => {
                            typed.blind_transf_exec(&pair_address, fc.clone())
                        }
                    }
                    .payment(
                        Payment::try_new(&send_token_id, 0, send_amount)
                            .expect("Amount must be > 0"),
                    )
                    .returns(PassValue(wallet.clone()))
                    .returns(PassValue(contract.clone()))
                    .returns(ReturnsStatus)
                    .returns(ReturnsGasUsed)
                });
            }
        }
        for (wallet, contract, status, gas_used) in buffer.run().await {
            println!(
                "swap via forwarder (wallet: {wallet}, contract: {contract}): status={status:?}, gas_used={gas_used:?}"
            );
        }
    }

    pub async fn swap1_direct(&mut self, wegld_amount: u64, usdc_amount_min: u64) {
        let send_token = self.config.wegld_token_id.clone();
        let want_token = self.config.usdc_token_id.clone();
        self.swap_direct_impl(send_token, wegld_amount, want_token, usdc_amount_min)
            .await;
    }

    pub async fn swap2_direct(&mut self, usdc_amount: u64, wegld_amount_min: u64) {
        let send_token = self.config.usdc_token_id.clone();
        let want_token = self.config.wegld_token_id.clone();
        self.swap_direct_impl(send_token, usdc_amount, want_token, wegld_amount_min)
            .await;
    }

    pub async fn swap1_sync(&mut self, wegld_amount: u64, usdc_amount_min: u64) {
        let send_token = self.config.wegld_token_id.clone();
        let want_token = self.config.usdc_token_id.clone();
        self.swap_via_forwarder_impl(
            send_token,
            wegld_amount,
            want_token,
            usdc_amount_min,
            ForwarderMethod::Sync,
        )
        .await;
    }

    pub async fn swap2_sync(&mut self, usdc_amount: u64, wegld_amount_min: u64) {
        let send_token = self.config.usdc_token_id.clone();
        let want_token = self.config.wegld_token_id.clone();
        self.swap_via_forwarder_impl(
            send_token,
            usdc_amount,
            want_token,
            wegld_amount_min,
            ForwarderMethod::Sync,
        )
        .await;
    }

    pub async fn swap1_async1(&mut self, wegld_amount: u64, usdc_amount_min: u64) {
        let send_token = self.config.wegld_token_id.clone();
        let want_token = self.config.usdc_token_id.clone();
        self.swap_via_forwarder_impl(
            send_token,
            wegld_amount,
            want_token,
            usdc_amount_min,
            ForwarderMethod::AsyncV1,
        )
        .await;
    }

    pub async fn swap2_async1(&mut self, usdc_amount: u64, wegld_amount_min: u64) {
        let send_token = self.config.usdc_token_id.clone();
        let want_token = self.config.wegld_token_id.clone();
        self.swap_via_forwarder_impl(
            send_token,
            usdc_amount,
            want_token,
            wegld_amount_min,
            ForwarderMethod::AsyncV1,
        )
        .await;
    }

    pub async fn swap1_async2(&mut self, wegld_amount: u64, usdc_amount_min: u64) {
        let send_token = self.config.wegld_token_id.clone();
        let want_token = self.config.usdc_token_id.clone();
        self.swap_via_forwarder_impl(
            send_token,
            wegld_amount,
            want_token,
            usdc_amount_min,
            ForwarderMethod::AsyncV2,
        )
        .await;
    }

    pub async fn swap2_async2(&mut self, usdc_amount: u64, wegld_amount_min: u64) {
        let send_token = self.config.usdc_token_id.clone();
        let want_token = self.config.wegld_token_id.clone();
        self.swap_via_forwarder_impl(
            send_token,
            usdc_amount,
            want_token,
            wegld_amount_min,
            ForwarderMethod::AsyncV2,
        )
        .await;
    }

    pub async fn swap1_te(&mut self, wegld_amount: u64, usdc_amount_min: u64) {
        let send_token = self.config.wegld_token_id.clone();
        let want_token = self.config.usdc_token_id.clone();
        self.swap_via_forwarder_impl(
            send_token,
            wegld_amount,
            want_token,
            usdc_amount_min,
            ForwarderMethod::TransfExec,
        )
        .await;
    }

    pub async fn swap2_te(&mut self, usdc_amount: u64, wegld_amount_min: u64) {
        let send_token = self.config.usdc_token_id.clone();
        let want_token = self.config.wegld_token_id.clone();
        self.swap_via_forwarder_impl(
            send_token,
            usdc_amount,
            want_token,
            wegld_amount_min,
            ForwarderMethod::TransfExec,
        )
        .await;
    }

    pub async fn drain(&mut self) {
        let contract_addresses = self.config.contract_addresses.clone();
        let wegld_token_id = self.config.wegld_token_id.clone();
        let usdc_token_id = self.config.usdc_token_id.clone();

        // For each contract, fetch its on-chain owner and check if we have it registered.
        let mut owner_per_contract: Vec<(Bech32Address, Bech32Address)> = Vec::new();
        for contract in &contract_addresses {
            match self
                .interactor
                .get_registered_owner(contract.as_address())
                .await
            {
                Some(owner) => owner_per_contract.push((contract.clone(), owner)),
                None => {
                    println!("Drain: no registered owner found for contract {contract}, skipping")
                }
            }
        }

        // Fetch ESDT balances for each contract before opening the call buffer.
        let mut esdt_per_contract = Vec::new();
        for (contract, owner) in &owner_per_contract {
            let contract_esdt = self
                .interactor
                .get_account_esdt(contract.as_address())
                .await;
            esdt_per_contract.push((contract.clone(), owner.clone(), contract_esdt));
        }

        let mut buffer = self.interactor.homogenous_call_buffer();
        for (contract, owner, contract_esdt) in &esdt_per_contract {
            for token_id in [wegld_token_id.clone(), usdc_token_id.clone()] {
                let balance = contract_esdt
                    .get(&token_id)
                    .and_then(|b| b.balance.parse::<u128>().ok())
                    .unwrap_or(0);
                if balance == 0 {
                    println!("Drain {token_id} (contract: {contract}): no balance, skipping");
                    continue;
                }
                buffer.push_tx(|tx| {
                    tx.from(owner)
                        .to(contract)
                        .gas(10_000_000u64)
                        .typed(forwarder_blind_proxy::ForwarderBlindProxy)
                        .drain(EsdtTokenIdentifier::from(token_id.as_str()), 0u64)
                        .returns(PassValue(owner.clone()))
                        .returns(PassValue(contract.clone()))
                        .returns(PassValue(token_id.clone()))
                        .returns(ReturnsStatus)
                        .returns(ReturnsGasUsed)
                });
            }
        }

        for (owner, contract, token_id, status, gas_used) in buffer.run().await {
            println!(
                "Drain {token_id} (owner: {owner}, contract: {contract}): status={status:?}, gas_used={gas_used:?}"
            );
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

    pub async fn balances(&mut self) {
        let wegld_token_id = self.config.wegld_token_id.clone();
        let usdc_token_id = self.config.usdc_token_id.clone();

        println!("=== Wallet Balances ===");
        for wallet in &self.wallet_addresses.clone() {
            let account = self.interactor.get_account(wallet.as_address()).await;
            let esdt = self.interactor.get_account_esdt(wallet.as_address()).await;
            let wegld = esdt
                .get(&wegld_token_id)
                .map(|b| b.balance.as_str())
                .unwrap_or("0");
            let usdc = esdt
                .get(&usdc_token_id)
                .map(|b| b.balance.as_str())
                .unwrap_or("0");
            println!("  wallet: {wallet}");
            println!("    EGLD:          {}", account.balance);
            println!("    {wegld_token_id}: {wegld}");
            println!("    {usdc_token_id}:          {usdc}");
        }

        let contract_addresses = self.config.contract_addresses.clone();
        if !contract_addresses.is_empty() {
            println!("=== Contract Balances ===");
            for contract in &contract_addresses {
                let esdt = self
                    .interactor
                    .get_account_esdt(contract.as_address())
                    .await;
                let wegld = esdt
                    .get(&wegld_token_id)
                    .map(|b| b.balance.as_str())
                    .unwrap_or("0");
                let usdc = esdt
                    .get(&usdc_token_id)
                    .map(|b| b.balance.as_str())
                    .unwrap_or("0");
                println!("  contract: {contract}");
                println!("    {wegld_token_id}: {wegld}");
                println!("    {usdc_token_id}:          {usdc}");
            }
        }
    }
}
