mod basic_interact_cli;
mod basic_interact_config;
mod basic_interact_state;

use basic_interact_config::Config;
use basic_interact_state::State;
use clap::Parser;

use multiversx_sc_snippets::imports::*;

#[tokio::main]
async fn main() {
    env_logger::init();

    let mut basic_interact = SysFuncCallsInteract::init().await;

    let cli = basic_interact_cli::InteractCli::parse();
    match &cli.command {
        Some(basic_interact_cli::InteractCliCommand::Add(args)) => {},
        Some(basic_interact_cli::InteractCliCommand::IssueToken(args)) => {
            basic_interact
                .issue_token(
                    args.cost.clone(),
                    &args.display_name,
                    &args.ticker,
                    EsdtTokenType::from(args.token_type),
                    args.num_decimals,
                )
                .await;
        },

        None => {},
    }
}

#[allow(unused)]
struct SysFuncCallsInteract {
    interactor: Interactor,
    wallet_address: Bech32Address,
    state: State,
}

impl SysFuncCallsInteract {
    async fn init() -> Self {
        let config = Config::load_config();
        let mut interactor = Interactor::new(config.gateway()).await;

        let wallet_address = interactor.register_wallet(test_wallets::alice());

        Self {
            interactor,
            wallet_address: wallet_address.into(),
            state: State::load_state(),
        }
    }

    async fn set_state(&mut self) {
        println!("wallet address: {}", self.wallet_address);
        self.interactor.retrieve_account(&self.wallet_address).await;
    }

    async fn issue_token(
        &mut self,
        issue_cost: RustBigUint,
        token_display_name: &str,
        token_ticker: &str,
        token_type: EsdtTokenType,
        num_decimals: usize,
    ) {
        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(ESDTSystemSCAddress.to_managed_address())
            .gas(100_000_000u64)
            .typed(ESDTSystemSCProxy)
            .issue_and_set_all_roles(
                issue_cost.into(),
                token_display_name.into(),
                token_ticker.into(),
                token_type,
                num_decimals,
            )
            .prepare_async()
            .run()
            .await;
    }
}
