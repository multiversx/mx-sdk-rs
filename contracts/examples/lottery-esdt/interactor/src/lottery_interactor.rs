mod lottery_interactor_cli;
mod lottery_interactor_config;
mod lottery_interactor_state;

use clap::Parser;
use lottery_esdt::lottery_proxy;
pub use lottery_interactor_config::Config;
use lottery_interactor_state::State;

use multiversx_sc_snippets::{hex, imports::*};

const LOTTERY_CODE_PATH: MxscPath = MxscPath::new("../output/lottery-esdt.mxsc.json");

pub async fn lottery_cli() {
    env_logger::init();

    let config = Config::load_config();

    let mut lottery_interact = LotteryInteract::new(config).await;

    let cli = lottery_interactor_cli::InteractCli::parse();
    match &cli.command {
        Some(lottery_interactor_cli::InteractCliCommand::Deploy) => {
            lottery_interact.deploy().await;
        }
        Some(lottery_interactor_cli::InteractCliCommand::CreateLotteryPool(args)) => {
            lottery_interact
                .create_lottery_pool(
                    &args.lottery_name,
                    &args.token_identifier,
                    args.ticket_price,
                    args.opt_total_tickets,
                    args.opt_deadline,
                    args.opt_max_entries_per_user,
                    args.opt_prize_distribution.clone(),
                    args.get_opt_whitelist_arg(),
                    OptionalValue::from(args.opt_burn_percentage.clone()),
                )
                .await;
        }
        Some(lottery_interactor_cli::InteractCliCommand::BuyTicket(args)) => {
            lottery_interact.buy_ticket(&args.name).await;
        }
        Some(lottery_interactor_cli::InteractCliCommand::DetermineWinner(args)) => {
            lottery_interact.determine_winner(&args.name).await;
        }
        Some(lottery_interactor_cli::InteractCliCommand::ClaimRewards(args)) => {
            lottery_interact
                .claim_rewards(
                    args.tokens
                        .iter()
                        .map(|token| TokenIdentifier::from(token))
                        .collect(),
                )
                .await;
        }
        None => {}
    }
}

#[derive(Clone)]
pub struct AddressWithShard {
    pub address: Bech32Address,
    pub shard: u8,
}

pub struct LotteryInteract {
    pub interactor: Interactor,
    pub lottery_owner: AddressWithShard,
    pub account_1: AddressWithShard,
    pub account_2: AddressWithShard,
    pub other_shard_account: AddressWithShard,
    pub state: State,
}

impl LotteryInteract {
    pub async fn new(config: Config) -> Self {
        let mut interactor = Interactor::new(config.gateway_uri())
            .await
            .use_chain_simulator(config.use_chain_simulator());
        interactor.set_current_dir_from_workspace("contracts/examples/lottery-esdt/interactor");

        let lottery_owner_wallet = test_wallets::heidi(); // shard 1
        let account_1_wallet = test_wallets::alice(); // shard 0
        let account_2_wallet = test_wallets::bob(); // shard 2
        let other_shard_wallet = test_wallets::carol(); // shard 0

        let lottery_owner_address = interactor.register_wallet(lottery_owner_wallet).await;
        let account_1_address = interactor.register_wallet(account_1_wallet).await;
        let account_2_address = interactor.register_wallet(account_2_wallet).await;
        let other_shard_address = interactor.register_wallet(other_shard_wallet).await;

        interactor.generate_blocks(30u64).await.unwrap();

        LotteryInteract {
            interactor,
            lottery_owner: AddressWithShard {
                address: lottery_owner_address.clone().into(),
                shard: lottery_owner_wallet.get_shard(),
            },
            account_1: AddressWithShard {
                address: account_1_address.into(),
                shard: account_1_wallet.get_shard(),
            },
            account_2: AddressWithShard {
                address: account_2_address.into(),
                shard: account_2_wallet.get_shard(),
            },
            other_shard_account: AddressWithShard {
                address: other_shard_address.into(),
                shard: other_shard_wallet.get_shard(),
            },
            state: State::load_state(),
        }
    }

    pub async fn deploy(&mut self) {
        let (new_address, shard) = self.handle_different_shard_address().await;
        println!("new address: {new_address} on shard {shard}");
        self.state.set_lottery_address(new_address);
    }

    async fn handle_different_shard_address(&mut self) -> (Bech32Address, u32) {
        let (new_address, tx_hash) = self
            .interactor
            .tx()
            .from(&self.lottery_owner.address)
            .gas(50_000_000)
            .typed(lottery_proxy::LotteryProxy)
            .init()
            .code(LOTTERY_CODE_PATH)
            .returns(ReturnsNewBech32Address)
            .returns(ReturnsTxHash)
            .run()
            .await;

        let tx_hash_string = hex::encode(tx_hash.to_vec());
        let tx_on_network = self
            .interactor
            .proxy
            .get_transaction_info_with_results(&tx_hash_string)
            .await
            .unwrap();
        let shard = tx_on_network.destination_shard;

        if self.other_shard_account.shard as u32 == shard {
            // we want to have other_shard_account on another shard than the SC
            std::mem::swap(&mut self.account_1, &mut self.other_shard_account);
        }

        return (new_address, 0);
    }

    pub async fn create_lottery_pool(
        &mut self,
        lottery_name: &String,
        token_identifier: &String,
        ticket_price: u128,
        opt_total_tickets: Option<usize>,
        opt_deadline: Option<u64>,
        opt_max_entries_per_user: Option<usize>,
        opt_prize_distribution: Option<Vec<u8>>,
        opt_whitelist: Option<Vec<Address>>,
        opt_burn_percentage: OptionalValue<u128>,
    ) {
        self.interactor
            .tx()
            .from(&self.account_1.address)
            .to(self.state.current_lottery_address())
            .gas(6_000_000u64)
            .typed(lottery_proxy::LotteryProxy)
            .create_lottery_pool(
                lottery_name,
                TokenIdentifier::from(token_identifier),
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
            .from(&self.account_1.address)
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
            .from(&self.account_1.address)
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
            .from(&self.account_1.address)
            .to(self.state.current_lottery_address())
            .gas(6_000_000u64)
            .typed(lottery_proxy::LotteryProxy)
            .claim_rewards(tokens)
            .run()
            .await;
        println!("Successfully performed claim_rewards");
    }
}
