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
        Some(basic_interact_cli::InteractCliCommand::IssueToken(args)) => {
            basic_interact
                .issue_token(
                    args.cost.clone(),
                    &args.display_name,
                    &args.ticker,
                    args.num_decimals,
                    args.token_type.into(),
                )
                .await;
        },
        Some(basic_interact_cli::InteractCliCommand::Mint(args)) => {
            basic_interact.mint_token(args.amount.clone()).await;
        },
        Some(basic_interact_cli::InteractCliCommand::SetRoles(args)) => {
            // let parsed_args = SetRolesArgs::parse();
            basic_interact.set_role(args.roles.clone()).await;
        },
        Some(basic_interact_cli::InteractCliCommand::Burn(args)) => {
            basic_interact.burn_token(args.amount.clone()).await;
        },

        None => {},
    }
}

#[allow(unused)]
struct SysFuncCallsInteract {
    interactor: Interactor,
    wallet_address: Bech32Address,
    state: State,
    token_id: String,
}

impl SysFuncCallsInteract {
    async fn init() -> Self {
        let config = Config::load_config();
        let mut interactor = Interactor::new(config.gateway()).await;

        let wallet_address =
            interactor.register_wallet(Wallet::from_pem_file("wallet.pem").unwrap());

        Self {
            interactor,
            wallet_address: wallet_address.into(),
            state: State::load_state(),
            token_id: String::new(),
        }
    }

    async fn _issue_fungible_token(
        &mut self,
        issue_cost: RustBigUint,
        token_display_name: &str,
        token_ticker: &str,
        num_decimals: usize,
    ) {
        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(ESDTSystemSCAddress.to_managed_address())
            .gas(100_000_000u64)
            .typed(ESDTSystemSCProxy)
            .issue_fungible(
                issue_cost.into(),
                &token_display_name.into(),
                &token_ticker.into(),
                &BigUint::from(100000000000000000000u128),
                FungibleTokenProperties {
                    num_decimals: num_decimals,
                    can_freeze: true,
                    can_wipe: true,
                    can_pause: true,
                    can_mint: true,
                    can_burn: true,
                    can_change_owner: true,
                    can_upgrade: true,
                    can_add_special_roles: true,
                },
            )
            .prepare_async()
            .run()
            .await;
    }

    async fn issue_token(
        &mut self,
        issue_cost: RustBigUint,
        token_display_name: &str,
        token_ticker: &str,
        num_decimals: usize,
        token_type: EsdtTokenType,
    ) {
        let token_id = self
            .interactor
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
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("TOKEN ID: {:?}", token_id.to_string());
        self.token_id = token_id.to_string();
    }

    async fn set_role(&mut self, roles: Vec<u16>) {
        let wallet_address = &self.wallet_address.clone().into_address();
        let converted_roles: Vec<EsdtLocalRole> =
            roles.into_iter().map(|r| EsdtLocalRole::from(r)).collect();

        println!("ROLES: {:?}", converted_roles);

        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(ESDTSystemSCAddress.to_managed_address())
            .gas(100_000_000u64)
            .typed(ESDTSystemSCProxy)
            .set_special_roles::<std::vec::IntoIter<EsdtLocalRole>>(
                &ManagedAddress::from_address(wallet_address),
                &TokenIdentifier::from(&self.token_id),
                converted_roles.into_iter(),
            )
            .prepare_async()
            .run()
            .await;
    }

    async fn mint_token(&mut self, amount: RustBigUint) {
        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(&self.wallet_address)
            .gas(100_000_000u64)
            .typed(UserBuiltinProxy)
            .esdt_local_mint(
                &TokenIdentifier::from(&self.token_id),
                0,
                &BigUint::from(amount),
            )
            .prepare_async()
            .run()
            .await;
    }

    async fn burn_token(&mut self, amount: RustBigUint) {
        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(&self.wallet_address)
            .gas(100_000_000u64)
            .typed(UserBuiltinProxy)
            .esdt_local_burn(
                &TokenIdentifier::from(&self.token_id),
                0,
                &BigUint::from(amount),
            )
            .prepare_async()
            .run()
            .await;
    }
}
