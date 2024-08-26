mod basic_interact_cli;
mod basic_interact_config;
mod basic_interact_state;

use basic_interact_cli::NftDummyAttributes;
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
            basic_interact
                .mint_token(&args.token_id, args.amount.clone())
                .await;
        },
        Some(basic_interact_cli::InteractCliCommand::SetRoles(args)) => {
            // let parsed_args = SetRolesArgs::parse();
            basic_interact
                .set_role(&args.token_id, args.roles.clone())
                .await;
        },
        Some(basic_interact_cli::InteractCliCommand::Burn(args)) => {
            basic_interact
                .burn_token(&args.token_id, args.amount.clone())
                .await;
        },
        Some(basic_interact_cli::InteractCliCommand::IssueFungible(args)) => {
            basic_interact
                ._issue_fungible_token(
                    args.cost.clone(),
                    &args.display_name,
                    &args.ticker,
                    args.supply.clone(),
                    args.num_decimals,
                )
                .await;
        },
        Some(basic_interact_cli::InteractCliCommand::IssueSft(args)) => {
            basic_interact
                .issue_semi_fungible_token(args.cost.clone(), &args.display_name, &args.ticker)
                .await;
        },
        Some(basic_interact_cli::InteractCliCommand::MintSft(args)) => {
            basic_interact
                .mint_sft(
                    &args.token_id,
                    args.amount.clone(),
                    &args.name,
                    args.royalties.clone(),
                    &args.hash,
                )
                .await;
        },
        Some(basic_interact_cli::InteractCliCommand::RegisterMetaEsdt(args)) => {
            basic_interact
                .register_meta_esdt(
                    args.cost.clone(),
                    &args.display_name,
                    &args.ticker,
                    args.num_decimals,
                )
                .await;
        },
        Some(basic_interact_cli::InteractCliCommand::ChangeSftMetaEsdt(args)) => {
            basic_interact
                .change_sft_meta_esdt(&args.token_id, args.num_decimals)
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

        let wallet_address =
            interactor.register_wallet(Wallet::from_pem_file("wallet.pem").unwrap());

        Self {
            interactor,
            wallet_address: wallet_address.into(),
            state: State::load_state(),
        }
    }

    async fn _issue_fungible_token(
        &mut self,
        issue_cost: RustBigUint,
        token_display_name: &str,
        token_ticker: &str,
        initial_supply: RustBigUint,
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
                &initial_supply.into(),
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

    async fn issue_semi_fungible_token(
        &mut self,
        issue_cost: RustBigUint,
        token_display_name: &str,
        token_ticker: &str,
    ) {
        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(ESDTSystemSCAddress.to_managed_address())
            .gas(100_000_000u64)
            .typed(ESDTSystemSCProxy)
            .issue_semi_fungible(
                issue_cost.into(),
                &token_display_name.into(),
                &token_ticker.into(),
                SemiFungibleTokenProperties {
                    can_freeze: true,
                    can_wipe: true,
                    can_pause: true,
                    can_transfer_create_role: true,
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
    }

    async fn set_role(&mut self, token_id: &str, roles: Vec<u16>) {
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
                &TokenIdentifier::from(token_id),
                converted_roles.into_iter(),
            )
            .prepare_async()
            .run()
            .await;
    }

    async fn mint_sft(
        &mut self,
        token_id: &str,
        amount: RustBigUint,
        name: &str,
        royalties: RustBigUint,
        hash: &str,
    ) {
        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(&self.wallet_address)
            .gas(100_000_000u64)
            .typed(UserBuiltinProxy)
            .esdt_nft_create(
                &TokenIdentifier::from(token_id),
                &BigUint::from(amount),
                &ManagedBuffer::from(name),
                &royalties.into(),
                &ManagedBuffer::from(hash),
                &NftDummyAttributes {
                    creation_epoch: 2104,
                    cool_factor: 5,
                },
                &ManagedVec::new(),
            )
            .prepare_async()
            .run()
            .await;
    }

    async fn register_meta_esdt(
        &mut self,
        issue_cost: RustBigUint,
        token_display_name: &str,
        token_ticker: &str,
        num_decimals: usize,
    ) {
        let meta_esdt = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(ESDTSystemSCAddress.to_managed_address())
            .gas(100_000_000u64)
            .typed(ESDTSystemSCProxy)
            .register_meta_esdt(
                issue_cost.into(),
                &token_display_name.into(),
                &token_ticker.into(),
                MetaTokenProperties {
                    num_decimals: num_decimals,
                    can_freeze: true,
                    can_wipe: true,
                    can_transfer_create_role: true,
                    can_change_owner: true,
                    can_upgrade: true,
                    can_pause: true,
                    can_add_special_roles: true,
                },
            )
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("META ESDT: {:?}", meta_esdt.to_string());
    }

    async fn change_sft_meta_esdt(&mut self, token_id: &str, num_decimals: usize) {
        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(ESDTSystemSCAddress.to_managed_address())
            .gas(100_000_000u64)
            .typed(ESDTSystemSCProxy)
            .change_sft_to_meta_esdt(&TokenIdentifier::from(token_id), num_decimals)
            .prepare_async()
            .run()
            .await;
    }

    async fn mint_token(&mut self, token_id: &str, amount: RustBigUint) {
        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(&self.wallet_address)
            .gas(100_000_000u64)
            .typed(UserBuiltinProxy)
            .esdt_local_mint(&TokenIdentifier::from(token_id), 0, &BigUint::from(amount))
            .prepare_async()
            .run()
            .await;
    }

    async fn burn_token(&mut self, token_id: &str, amount: RustBigUint) {
        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(&self.wallet_address)
            .gas(100_000_000u64)
            .typed(UserBuiltinProxy)
            .esdt_local_burn(&TokenIdentifier::from(token_id), 0, &BigUint::from(amount))
            .prepare_async()
            .run()
            .await;
    }
}
