mod system_sc_interact_cli;
mod system_sc_interact_config;
mod system_sc_interact_state;

use clap::Parser;
pub use system_sc_interact_cli::NftDummyAttributes;
pub use system_sc_interact_config::Config;
use system_sc_interact_state::State;

use multiversx_sc_snippets::imports::*;

pub async fn system_sc_interact_cli() {
    env_logger::init();

    let mut basic_interact = SysFuncCallsInteract::init(Config::load_config()).await;

    let cli = system_sc_interact_cli::InteractCli::parse();
    match &cli.command {
        Some(system_sc_interact_cli::InteractCliCommand::IssueToken(args)) => {
            basic_interact
                .issue_token_all_roles(
                    args.cost.clone(),
                    args.display_name.as_bytes(),
                    args.ticker.as_bytes(),
                    args.num_decimals,
                    args.token_type.into(),
                )
                .await;
        },
        Some(system_sc_interact_cli::InteractCliCommand::Mint(args)) => {
            basic_interact
                .mint_token(
                    args.token_id.clone().as_bytes(),
                    args.nonce,
                    args.amount.clone(),
                )
                .await;
        },
        Some(system_sc_interact_cli::InteractCliCommand::SetRoles(args)) => {
            basic_interact
                .set_roles(
                    args.token_id.as_bytes(),
                    args.roles
                        .clone()
                        .into_iter()
                        .map(EsdtLocalRole::from)
                        .collect(),
                )
                .await;
        },
        Some(system_sc_interact_cli::InteractCliCommand::Burn(args)) => {
            basic_interact
                .burn_token(args.token_id.as_bytes(), args.nonce, args.amount.clone())
                .await;
        },
        Some(system_sc_interact_cli::InteractCliCommand::PauseToken(args)) => {
            basic_interact.pause_token(args.token_id.as_bytes()).await;
        },
        Some(system_sc_interact_cli::InteractCliCommand::UnpauseToken(args)) => {
            basic_interact.unpause_token(args.token_id.as_bytes()).await;
        },
        Some(system_sc_interact_cli::InteractCliCommand::FreezeToken(args)) => {
            basic_interact
                .freeze_token(
                    args.token_id.as_bytes(),
                    &Bech32Address::from_bech32_string(args.address.clone()),
                )
                .await;
        },
        Some(system_sc_interact_cli::InteractCliCommand::UnfreezeToken(args)) => {
            basic_interact
                .unfreeze_token(
                    args.token_id.as_bytes(),
                    &Bech32Address::from_bech32_string(args.address.clone()),
                )
                .await;
        },
        Some(system_sc_interact_cli::InteractCliCommand::FreezeNFT(args)) => {
            basic_interact
                .freeze_nft(
                    args.token_id.as_bytes(),
                    args.nft_nonce,
                    &Bech32Address::from_bech32_string(args.address.clone()),
                )
                .await;
        },
        Some(system_sc_interact_cli::InteractCliCommand::UnfreezeNFT(args)) => {
            basic_interact
                .unfreeze_nft(
                    args.token_id.as_bytes(),
                    args.nft_nonce,
                    &Bech32Address::from_bech32_string(args.address.clone()),
                )
                .await;
        },
        Some(system_sc_interact_cli::InteractCliCommand::WipeToken(args)) => {
            basic_interact
                .wipe_token(
                    args.token_id.as_bytes(),
                    &Bech32Address::from_bech32_string(args.address.clone()),
                )
                .await;
        },
        Some(system_sc_interact_cli::InteractCliCommand::WipeNFT(args)) => {
            basic_interact
                .wipe_nft(
                    args.token_id.as_bytes(),
                    args.nft_nonce,
                    &Bech32Address::from_bech32_string(args.address.clone()),
                )
                .await;
        },
        Some(system_sc_interact_cli::InteractCliCommand::IssueNFTCollection(args)) => {
            basic_interact
                .issue_non_fungible_collection(
                    args.cost.clone(),
                    args.display_name.as_bytes(),
                    args.ticker.as_bytes(),
                )
                .await;
        },
        Some(system_sc_interact_cli::InteractCliCommand::CreateNFT(args)) => {
            basic_interact
                .mint_nft(
                    args.token_id.as_bytes(),
                    args.amount.clone(),
                    args.name.as_bytes(),
                    args.royalties,
                    args.hash.as_bytes(),
                    &NftDummyAttributes {
                        creation_epoch: 2u64,
                        cool_factor: 3u8,
                    },
                    Vec::new(),
                )
                .await;
        },
        Some(system_sc_interact_cli::InteractCliCommand::IssueFungible(args)) => {
            basic_interact
                .issue_fungible_token(
                    args.cost.clone(),
                    args.display_name.as_bytes(),
                    args.ticker.as_bytes(),
                    args.supply.clone(),
                    args.num_decimals,
                )
                .await;
        },
        Some(system_sc_interact_cli::InteractCliCommand::IssueSftCollection(args)) => {
            basic_interact
                .issue_semi_fungible_collection(
                    args.cost.clone(),
                    args.display_name.as_bytes(),
                    args.ticker.as_bytes(),
                )
                .await;
        },
        Some(system_sc_interact_cli::InteractCliCommand::MintSft(args)) => {
            basic_interact
                .mint_sft(
                    args.token_id.as_bytes(),
                    args.amount.clone(),
                    args.name.as_bytes(),
                    args.royalties,
                    args.hash.as_bytes(),
                )
                .await;
        },
        Some(system_sc_interact_cli::InteractCliCommand::RegisterMetaEsdt(args)) => {
            basic_interact
                .register_meta_esdt(
                    args.cost.clone(),
                    args.display_name.as_bytes(),
                    args.ticker.as_bytes(),
                    args.num_decimals,
                )
                .await;
        },
        Some(system_sc_interact_cli::InteractCliCommand::ChangeSftMetaEsdt(args)) => {
            basic_interact
                .change_sft_meta_esdt(args.token_id.as_bytes(), args.num_decimals)
                .await;
        },
        Some(system_sc_interact_cli::InteractCliCommand::UnsetRoles(args)) => {
            basic_interact
                .unset_roles(
                    &Bech32Address::from_bech32_string(args.address.clone()),
                    args.token_id.as_bytes(),
                    args.roles
                        .clone()
                        .into_iter()
                        .map(EsdtLocalRole::from)
                        .collect(),
                )
                .await;
        },
        Some(system_sc_interact_cli::InteractCliCommand::TransferOwnership(args)) => {
            basic_interact
                .transfer_ownership(
                    args.token_id.as_bytes(),
                    &Bech32Address::from_bech32_string(args.new_owner.clone()),
                )
                .await;
        },
        Some(system_sc_interact_cli::InteractCliCommand::TransferNftCreateRole(args)) => {
            basic_interact
                .transfer_nft_create_role(
                    args.token_id.as_bytes(),
                    &Bech32Address::from_bech32_string(args.old_owner.clone()),
                    &Bech32Address::from_bech32_string(args.new_owner.clone()),
                )
                .await;
        },
        Some(system_sc_interact_cli::InteractCliCommand::ControlChanges(args)) => {
            basic_interact
                .control_changes(args.token_id.as_bytes())
                .await;
        },

        None => {},
    }
}

pub struct SysFuncCallsInteract {
    interactor: Interactor,
    wallet_address: Bech32Address,
    other_wallet_address: Bech32Address,
    #[allow(unused)]
    state: State,
}

impl SysFuncCallsInteract {
    pub async fn init(config: Config) -> Self {
        let mut interactor = Interactor::new(config.gateway_uri())
            .await
            .use_chain_simulator(config.is_chain_simulator());

        interactor.set_current_dir_from_workspace("tools/interactor-system-func-calls");
        let wallet_address = interactor.register_wallet(test_wallets::alice()).await;
        let other_wallet_address = interactor.register_wallet(test_wallets::mike()).await;

        // generate blocks until ESDTSystemSCAddress is enabled
        interactor.generate_blocks_until_epoch(1).await.unwrap();

        Self {
            interactor,
            wallet_address: wallet_address.into(),
            other_wallet_address: other_wallet_address.into(),
            state: State::load_state(),
        }
    }

    pub async fn issue_fungible_token(
        &mut self,
        issue_cost: RustBigUint,
        token_display_name: &[u8],
        token_ticker: &[u8],
        initial_supply: RustBigUint,
        num_decimals: usize,
    ) {
        println!("Issuing fungible token...");

        let res = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(ESDTSystemSCAddress)
            .gas(100_000_000u64)
            .typed(ESDTSystemSCProxy)
            .issue_fungible(
                issue_cost.into(),
                token_display_name,
                token_ticker,
                initial_supply,
                FungibleTokenProperties {
                    num_decimals,
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
            .returns(ReturnsNewTokenIdentifier)
            .run()
            .await;

        println!("TOKEN ID: {:?}", res);
    }

    pub async fn issue_non_fungible_collection(
        &mut self,
        issue_cost: RustBigUint,
        token_display_name: &[u8],
        token_ticker: &[u8],
    ) {
        println!("Issuing NFT Collection...");

        let nft_collection_id = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(ESDTSystemSCAddress)
            .gas(100_000_000u64)
            .typed(ESDTSystemSCProxy)
            .issue_non_fungible(
                issue_cost.into(),
                token_display_name,
                token_ticker,
                NonFungibleTokenProperties {
                    can_freeze: true,
                    can_wipe: true,
                    can_pause: true,
                    can_transfer_create_role: true,
                    can_change_owner: true,
                    can_upgrade: true,
                    can_add_special_roles: true,
                },
            )
            .returns(ReturnsNewTokenIdentifier)
            .run()
            .await;

        println!("NFT Collection ID: {:?}", nft_collection_id);
    }

    pub async fn issue_semi_fungible_collection(
        &mut self,
        issue_cost: RustBigUint,
        token_display_name: &[u8],
        token_ticker: &[u8],
    ) {
        println!("Issuing SFT Collection...");

        let sft_collection_id = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(ESDTSystemSCAddress)
            .gas(100_000_000u64)
            .typed(ESDTSystemSCProxy)
            .issue_semi_fungible(
                issue_cost.into(),
                token_display_name,
                token_ticker,
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
            .returns(ReturnsNewTokenIdentifier)
            .run()
            .await;

        println!("SFT Collection ID: {:?}", sft_collection_id);
    }

    pub async fn issue_dynamic_token(
        &mut self,
        issue_cost: RustBigUint,
        token_display_name: &[u8],
        token_ticker: &[u8],
        token_type: EsdtTokenType,
        num_decimals: usize,
    ) -> String {
        println!("Registering dynamic token {token_ticker:?} of type {token_type:?}...");

        let token_id = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(ESDTSystemSCAddress)
            .gas(100_000_000u64)
            .typed(ESDTSystemSCProxy)
            .issue_dynamic(
                issue_cost.into(),
                token_display_name,
                token_ticker,
                token_type,
                num_decimals,
            )
            .returns(ReturnsNewTokenIdentifier)
            .run()
            .await;

        println!("TOKEN ID: {:?}", token_id);

        token_id
    }

    pub async fn issue_token_all_roles(
        &mut self,
        issue_cost: RustBigUint,
        token_display_name: &[u8],
        token_ticker: &[u8],
        num_decimals: usize,
        token_type: EsdtTokenType,
    ) -> String {
        println!("Registering and setting all roles for token {token_ticker:?} of type {token_type:?}...");

        let token_id = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(ESDTSystemSCAddress)
            .gas(100_000_000u64)
            .typed(ESDTSystemSCProxy)
            .issue_and_set_all_roles(
                issue_cost.into(),
                token_display_name,
                token_ticker,
                token_type,
                num_decimals,
            )
            .returns(ReturnsNewTokenIdentifier)
            .run()
            .await;

        println!("TOKEN ID: {:?}", token_id);

        token_id
    }

    pub async fn set_roles_for_other(&mut self, token_id: &[u8], roles: Vec<EsdtLocalRole>) {
        let wallet_address = &self.other_wallet_address.clone().into_address();
        println!("Setting the following roles: {roles:?} for {token_id:?} for other_address");

        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(ESDTSystemSCAddress)
            .gas(100_000_000u64)
            .typed(ESDTSystemSCProxy)
            .set_special_roles(
                ManagedAddress::from_address(wallet_address),
                TokenIdentifier::from(token_id),
                roles.into_iter(),
            )
            .run()
            .await;
    }

    pub async fn set_roles(&mut self, token_id: &[u8], roles: Vec<EsdtLocalRole>) {
        let wallet_address = &self.wallet_address.clone().into_address();
        println!("Setting the following roles: {roles:?} for {token_id:?}");

        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(ESDTSystemSCAddress)
            .gas(100_000_000u64)
            .typed(ESDTSystemSCProxy)
            .set_special_roles(
                ManagedAddress::from_address(wallet_address),
                TokenIdentifier::from(token_id),
                roles.into_iter(),
            )
            .run()
            .await;
    }

    pub async fn get_roles(&mut self, token_id: &[u8]) {
        println!("Retrieving special roles for {token_id:?}");

        let result = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(ESDTSystemSCAddress)
            .gas(100_000_000u64)
            .typed(ESDTSystemSCProxy)
            .get_special_roles(TokenIdentifier::from(token_id))
            .returns(ReturnsRawResult)
            .run()
            .await;

        println!("raw result for roles {result:?}");
    }

    pub async fn change_to_dynamic(&mut self, token_id: &[u8]) {
        println!("Changing the following token {token_id:?} to dynamic...");

        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(ESDTSystemSCAddress)
            .gas(100_000_000u64)
            .typed(ESDTSystemSCProxy)
            .change_to_dynamic(TokenIdentifier::from(token_id))
            .run()
            .await;
    }

    pub async fn update_token(&mut self, token_id: &[u8]) {
        println!("Updating the following token {token_id:?} to the newest version...");

        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(ESDTSystemSCAddress)
            .gas(100_000_000u64)
            .typed(ESDTSystemSCProxy)
            .update_token(TokenIdentifier::from(token_id))
            .run()
            .await;
    }

    pub async fn mint_sft(
        &mut self,
        token_id: &[u8],
        amount: RustBigUint,
        name: &[u8],
        royalties: u64,
        hash: &[u8],
    ) {
        println!("Minting SFT...");

        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(&self.wallet_address)
            .gas(100_000_000u64)
            .typed(UserBuiltinProxy)
            .esdt_nft_create(
                token_id,
                amount,
                name,
                royalties,
                hash,
                &NftDummyAttributes {
                    creation_epoch: 2104,
                    cool_factor: 5,
                },
                &ManagedVec::new(),
            )
            .run()
            .await;
    }

    pub async fn register_meta_esdt(
        &mut self,
        issue_cost: RustBigUint,
        token_display_name: &[u8],
        token_ticker: &[u8],
        num_decimals: usize,
    ) {
        println!("Registering meta ESDT...");

        let meta_esdt = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(ESDTSystemSCAddress)
            .gas(100_000_000u64)
            .typed(ESDTSystemSCProxy)
            .register_meta_esdt(
                issue_cost.into(),
                token_display_name,
                token_ticker,
                MetaTokenProperties {
                    num_decimals,
                    can_freeze: true,
                    can_wipe: true,
                    can_transfer_create_role: true,
                    can_change_owner: true,
                    can_upgrade: true,
                    can_pause: true,
                    can_add_special_roles: true,
                },
            )
            .returns(ReturnsNewTokenIdentifier)
            .run()
            .await;

        println!("Meta-ESDT ID: {:?}", meta_esdt);
    }

    pub async fn change_sft_meta_esdt(&mut self, token_id: &[u8], num_decimals: usize) {
        println!("Changing SFT to Meta-ESDT...");

        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(ESDTSystemSCAddress)
            .gas(100_000_000u64)
            .typed(ESDTSystemSCProxy)
            .change_sft_to_meta_esdt(token_id, num_decimals)
            .run()
            .await;
    }

    pub async fn mint_token(&mut self, token_id: &[u8], nonce: u64, amount: RustBigUint) {
        println!("Minting tokens...");

        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(&self.wallet_address)
            .gas(100_000_000u64)
            .typed(UserBuiltinProxy)
            .esdt_local_mint(token_id, nonce, amount)
            .run()
            .await;
    }

    pub async fn burn_token(&mut self, token_id: &[u8], nonce: u64, amount: RustBigUint) {
        println!("Burning tokens...");

        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(&self.wallet_address)
            .gas(100_000_000u64)
            .typed(UserBuiltinProxy)
            .esdt_local_burn(token_id, nonce, amount)
            .run()
            .await;
    }

    pub async fn pause_token(&mut self, token_id: &[u8]) {
        println!("Pausing token...");

        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(ESDTSystemSCAddress)
            .gas(100_000_000u64)
            .typed(ESDTSystemSCProxy)
            .pause(token_id)
            .run()
            .await;
    }

    pub async fn unpause_token(&mut self, token_id: &[u8]) {
        println!("Unpausing token...");

        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(ESDTSystemSCAddress)
            .gas(100_000_000u64)
            .typed(ESDTSystemSCProxy)
            .unpause(token_id)
            .run()
            .await;
    }

    pub async fn freeze_token(&mut self, token_id: &[u8], address: &Bech32Address) {
        println!("Freezing token...");

        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(ESDTSystemSCAddress)
            .gas(100_000_000u64)
            .typed(ESDTSystemSCProxy)
            .freeze(token_id, address)
            .run()
            .await;
    }

    pub async fn unfreeze_token(&mut self, token_id: &[u8], address: &Bech32Address) {
        println!("Unfreezing token...");

        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(ESDTSystemSCAddress)
            .gas(100_000_000u64)
            .typed(ESDTSystemSCProxy)
            .unfreeze(token_id, address)
            .run()
            .await;
    }

    pub async fn freeze_nft(&mut self, token_id: &[u8], nonce: u64, address: &Bech32Address) {
        println!("Freezing NFT/SFT/Meta-ESDT...");

        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(ESDTSystemSCAddress)
            .gas(100_000_000u64)
            .typed(ESDTSystemSCProxy)
            .freeze_nft(token_id, nonce, address)
            .run()
            .await;
    }

    pub async fn unfreeze_nft(&mut self, token_id: &[u8], nonce: u64, address: &Bech32Address) {
        println!("Unfreezing NFT/SFT/Meta-ESDT...");

        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(ESDTSystemSCAddress)
            .gas(100_000_000u64)
            .typed(ESDTSystemSCProxy)
            .unfreeze_nft(token_id, nonce, address)
            .run()
            .await;
    }

    pub async fn wipe_token(&mut self, token_id: &[u8], address: &Bech32Address) {
        println!("Wiping token...");

        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(ESDTSystemSCAddress)
            .gas(100_000_000u64)
            .typed(ESDTSystemSCProxy)
            .wipe(token_id, address)
            .run()
            .await;
    }

    pub async fn wipe_nft(&mut self, token_id: &[u8], nonce: u64, address: &Bech32Address) {
        println!("Wiping NFT/SFT/Meta-ESDT...");

        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(ESDTSystemSCAddress)
            .gas(100_000_000u64)
            .typed(ESDTSystemSCProxy)
            .wipe_nft(token_id, nonce, address)
            .run()
            .await;
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn mint_nft<T: TopEncode>(
        &mut self,
        token_id: &[u8],
        amount: RustBigUint,
        name: &[u8],
        royalties: u64,
        hash: &[u8],
        attributes: &T,
        uris: Vec<String>,
    ) -> u64 {
        println!("Minting NFT...");

        let uris = uris
            .into_iter()
            .map(ManagedBuffer::from)
            .collect::<ManagedVec<StaticApi, ManagedBuffer<StaticApi>>>();

        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(&self.wallet_address)
            .gas(100_000_000u64)
            .typed(UserBuiltinProxy)
            .esdt_nft_create(token_id, amount, name, royalties, hash, attributes, &uris)
            .returns(ReturnsResult)
            .run()
            .await
    }

    pub async fn unset_roles(
        &mut self,
        address: &Bech32Address,
        token_id: &[u8],
        roles: Vec<EsdtLocalRole>,
    ) {
        println!("Unsetting the following roles: {:?}", roles);

        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(ESDTSystemSCAddress)
            .gas(100_000_000u64)
            .typed(ESDTSystemSCProxy)
            .unset_special_roles(address, token_id, roles.into_iter())
            .run()
            .await;
    }

    pub async fn transfer_ownership(&mut self, token_id: &[u8], new_owner: &Bech32Address) {
        println!("Transferring token ownership...");

        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(ESDTSystemSCAddress)
            .gas(100_000_000u64)
            .typed(ESDTSystemSCProxy)
            .transfer_ownership(token_id, new_owner)
            .run()
            .await;
    }

    pub async fn transfer_nft_create_role(
        &mut self,
        token_id: &[u8],
        old_owner: &Bech32Address,
        new_owner: &Bech32Address,
    ) {
        println!("Transferring NFT create role...");

        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(ESDTSystemSCAddress)
            .gas(100_000_000u64)
            .typed(ESDTSystemSCProxy)
            .transfer_nft_create_role(token_id, old_owner, new_owner)
            .run()
            .await;
    }

    pub async fn control_changes(&mut self, token_id: &[u8]) {
        println!("Control changes");

        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(ESDTSystemSCAddress)
            .gas(100_000_000u64)
            .typed(ESDTSystemSCProxy)
            .control_changes(
                token_id,
                &TokenPropertyArguments {
                    can_freeze: Some(true),
                    can_wipe: Some(true),
                    can_pause: Some(true),
                    can_transfer_create_role: Some(true),
                    can_mint: Some(true),
                    can_burn: Some(true),
                    can_change_owner: Some(true),
                    can_upgrade: Some(true),
                    can_add_special_roles: Some(true),
                },
            )
            .run()
            .await;
    }

    pub async fn modify_royalties(&mut self, token_id: &[u8], nonce: u64, new_royalty: u64) {
        println!("Modifying royalties for token {token_id:?} into {new_royalty:?}...");

        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(&self.wallet_address)
            .gas(100_000_000u64)
            .typed(UserBuiltinProxy)
            .esdt_modify_royalties(token_id, nonce, new_royalty)
            .run()
            .await;
    }

    pub async fn set_new_uris(&mut self, token_id: &[u8], nonce: u64, new_uris: Vec<String>) {
        println!("Setting new uris for token {token_id:?} with nonce {nonce:?}...");

        let uris = new_uris
            .into_iter()
            .map(ManagedBuffer::from)
            .collect::<ManagedVec<StaticApi, ManagedBuffer<StaticApi>>>();

        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(&self.wallet_address)
            .gas(100_000_000u64)
            .typed(UserBuiltinProxy)
            .esdt_nft_set_new_uris(token_id, nonce, &uris)
            .run()
            .await;
    }

    pub async fn send_esdt(&mut self, token_id: &[u8], nonce: u64, amount: RustBigUint) {
        println!("Sending token {token_id:?} with nonce {nonce:?} to other_wallet_address...");

        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(&self.other_wallet_address)
            .single_esdt(&token_id.into(), nonce, &amount.into()) // .transfer()
            .run()
            .await;
    }

    // changes creator into caller
    pub async fn modify_creator(&mut self, token_id: &[u8], nonce: u64) {
        println!(
            "Modifying the creator (into caller - other_wallet_address) for token {token_id:?} with nonce {nonce:?}..."
        );

        self.interactor
            .tx()
            .from(&self.other_wallet_address)
            .to(&self.other_wallet_address)
            .gas(100_000_000u64)
            .typed(UserBuiltinProxy)
            .esdt_nft_modify_creator(token_id, nonce)
            .run()
            .await;
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn metadata_recreate<T: TopEncode>(
        &mut self,
        token_id: &[u8],
        nonce: u64,
        name: &[u8],
        royalties: u64,
        hash: &[u8],
        new_attributes: &T,
        uris: Vec<String>,
    ) {
        println!("Recreating the token {token_id:?} with nonce {nonce:?} with new attributes...");

        let uris = uris
            .into_iter()
            .map(ManagedBuffer::from)
            .collect::<ManagedVec<StaticApi, ManagedBuffer<StaticApi>>>();

        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(&self.wallet_address)
            .gas(100_000_000u64)
            .typed(UserBuiltinProxy)
            .esdt_metadata_recreate(token_id, nonce, name, royalties, hash, new_attributes, uris)
            .run()
            .await;
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn metadata_update<T: TopEncode>(
        &mut self,
        token_id: &[u8],
        nonce: u64,
        name: &[u8],
        royalties: u64,
        hash: &[u8],
        new_attributes: &T,
        uris: Vec<String>,
    ) {
        println!("Updating the token {token_id:?} with nonce {nonce:?} with new attributes...");

        let uris = uris
            .into_iter()
            .map(ManagedBuffer::from)
            .collect::<ManagedVec<StaticApi, ManagedBuffer<StaticApi>>>();

        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(&self.wallet_address)
            .gas(100_000_000u64)
            .typed(UserBuiltinProxy)
            .esdt_metadata_update(token_id, nonce, name, royalties, hash, new_attributes, uris)
            .run()
            .await;
    }
}
