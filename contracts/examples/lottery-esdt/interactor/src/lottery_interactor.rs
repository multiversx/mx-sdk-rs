mod lottery_interactor_cli;
mod lottery_interactor_config;
mod lottery_interactor_state;

use clap::Parser;
use lottery_esdt::lottery_proxy;
pub use lottery_interactor_config::Config;
use lottery_interactor_state::State;
use num_bigint::BigUint;

use multiversx_sc_snippets::imports::*;

const LOTTERY_CODE_PATH: MxscPath = MxscPath::new("../output/lottery-esdt.mxsc.json");

pub async fn lottery_cli() {
    env_logger::init();

    let config = Config::load_config();

    let mut lottery_interact = LotteryInteract::new(config).await;

    let cli = lottery_interactor_cli::InteractCli::parse();
    match &cli.command {
        Some(lottery_interactor_cli::InteractCliCommand::Deploy) => {
            lottery_interact.deploy().await;
        },
        Some(lottery_interactor_cli::InteractCliCommand::Upgrade) => {
            let owner_address = lottery_interact.lottery_owner_address.clone();
            lottery_interact.upgrade(&owner_address, None).await
        },
        Some(lottery_interactor_cli::InteractCliCommand::CreateLotteryPool(args)) => {
            lottery_interact
                .create_lottery_pool(
                    &args.lottery_name,
                    TokenIdentifier::from(&args.token_identifier),
                    args.ticket_price.clone(),
                    args.opt_total_tickets,
                    args.opt_deadline,
                    args.opt_max_entries_per_user,
                    args.opt_prize_distribution.clone(),
                    args.get_opt_whitelist_arg(),
                    OptionalValue::from(args.opt_burn_percentage.clone()),
                )
                .await;
        },
        Some(lottery_interactor_cli::InteractCliCommand::BuyTicket(args)) => {
            lottery_interact.buy_ticket(&args.name).await;
        },
        Some(lottery_interactor_cli::InteractCliCommand::DetermineWinner(args)) => {
            lottery_interact.determine_winner(&args.name).await;
        },
        Some(lottery_interactor_cli::InteractCliCommand::ClaimRewards(args)) => {
            lottery_interact
                .claim_rewards(
                    args.tokens
                        .iter()
                        .map(|token| TokenIdentifier::from(token))
                        .collect(),
                )
                .await;
        },
        None => {},
    }
}

pub struct LotteryInteract {
    pub interactor: Interactor,
    pub lottery_owner_address: Bech32Address,
    pub wallet_address: Bech32Address,
    pub state: State,
}

impl LotteryInteract {
    pub async fn new(config: Config) -> Self {
        let mut interactor = Interactor::new(config.gateway_uri())
            .await
            .use_chain_simulator(config.use_chain_simulator());
        interactor.set_current_dir_from_workspace("contracts/examples/lottery-esdt/interactor");

        let lottery_owner_address = interactor.register_wallet(test_wallets::heidi()).await;
        let wallet_address = interactor.register_wallet(test_wallets::ivan()).await;

        interactor.generate_blocks(30u64).await.unwrap();

        LotteryInteract {
            interactor,
            lottery_owner_address: lottery_owner_address.into(),
            wallet_address: wallet_address.into(),
            state: State::load_state(),
        }
    }

    pub async fn deploy(&mut self) {
        let new_address = self
            .interactor
            .tx()
            .from(&self.lottery_owner_address.clone())
            .gas(50_000_000)
            .typed(lottery_proxy::LotteryProxy)
            .init()
            .code(LOTTERY_CODE_PATH)
            .returns(ReturnsNewBech32Address)
            .run()
            .await;

        println!("new address: {new_address}");
        self.state.set_lottery_address(new_address);
    }

    pub async fn upgrade(&mut self, sender: &Bech32Address, err: Option<&str>) {
        let response = self
            .interactor
            .tx()
            .from(sender)
            .to(self.state.current_lottery_address())
            .gas(6_000_000)
            .typed(lottery_proxy::LotteryProxy)
            .upgrade()
            .code(LOTTERY_CODE_PATH)
            .code_metadata(CodeMetadata::UPGRADEABLE)
            .returns(ReturnsHandledOrError::new())
            .run()
            .await;

        match response {
            Ok(_) => {
                println!("Contract successfully upgraded.");
            },
            Err(tx_err) => {
                println!("Contract failed upgrade with error: {}", tx_err.message);
                assert_eq!(tx_err.message, err.unwrap_or_default());
            },
        }
    }

    pub async fn create_lottery_pool(
        &mut self,
        lottery_name: &String,
        token_identifier: TokenIdentifier<StaticApi>,
        ticket_price: BigUint,
        opt_total_tickets: Option<usize>,
        opt_deadline: Option<u64>,
        opt_max_entries_per_user: Option<usize>,
        opt_prize_distribution: Option<Vec<u8>>,
        opt_whitelist: Option<Vec<Address>>,
        opt_burn_percentage: OptionalValue<BigUint>,
    ) {
        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_lottery_address())
            .gas(6_000_000u64)
            .typed(lottery_proxy::LotteryProxy)
            .create_lottery_pool(
                lottery_name,
                token_identifier,
                ticket_price,
                opt_total_tickets,
                opt_deadline,
                opt_max_entries_per_user,
                opt_prize_distribution,
                opt_whitelist,
                opt_burn_percentage,
            )
            .run()
            .await;

        println!("Successfully performed create_lottery_poll");
    }

    pub async fn buy_ticket(&mut self, lottery_name: &String) {
        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_lottery_address())
            .gas(6_000_000u64)
            .typed(lottery_proxy::LotteryProxy)
            .buy_ticket(lottery_name)
            .run()
            .await;

        println!("Successfully performed buy_ticket");
    }

    pub async fn determine_winner(&mut self, lottery_name: &String) {
        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_lottery_address())
            .gas(6_000_000u64)
            .typed(lottery_proxy::LotteryProxy)
            .determine_winner(lottery_name)
            .run()
            .await;
        println!("Successfully performed determine_winner");
    }

    pub async fn claim_rewards(
        &mut self,
        tokens: MultiValueEncoded<StaticApi, TokenIdentifier<StaticApi>>,
    ) {
        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_lottery_address())
            .gas(6_000_000u64)
            .typed(lottery_proxy::LotteryProxy)
            .claim_rewards(tokens)
            .run()
            .await;
        println!("Successfully performed claim_rewards");
    }
}
