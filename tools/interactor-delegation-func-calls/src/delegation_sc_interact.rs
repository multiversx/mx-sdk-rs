mod delegation_sc_interact_cli;
mod delegation_sc_interact_config;
mod delegation_sc_interact_state;

use std::vec;

use clap::Parser;
pub use delegation_sc_interact_config::Config;
use delegation_sc_interact_state::State;

use multiversx_sc_snippets::{
    hex,
    imports::*,
    sdk::{gateway::SetStateAccount, utils::base64_decode},
};

pub async fn delegation_sc_interact_cli() {
    env_logger::init();

    let mut interact = DelegateCallsInteract::new(Config::load_config()).await;

    let cli = delegation_sc_interact_cli::InteractCli::parse();
    match &cli.command {
        Some(delegation_sc_interact_cli::InteractCliCommand::Create(args)) => {
            interact.set_state().await;
            interact
                .create_new_delegation_contract(args.total_delegation_cap, args.service_fee)
                .await;
        },
        Some(delegation_sc_interact_cli::InteractCliCommand::SetMetadata(args)) => {
            interact
                .set_metadata(&args.name, &args.website, &args.identifier)
                .await;
        },
        Some(delegation_sc_interact_cli::InteractCliCommand::ChangeServiceFee(args)) => {
            interact.change_service_fee(args.fee).await;
        },
        Some(delegation_sc_interact_cli::InteractCliCommand::SetAutomaticActivation(args)) => {
            interact
                .set_automatic_activation(args.automatic_activation)
                .await;
        },
        Some(delegation_sc_interact_cli::InteractCliCommand::ModifyTotalDelegationCap(args)) => {
            interact
                .modify_total_delegation_cap(args.total_delegation_cap)
                .await;
        },
        Some(delegation_sc_interact_cli::InteractCliCommand::AddNode(args)) => {
            let bls_keys =
                vec![hex::decode(args.public_key.clone())
                    .expect("Failed to decode public key from hex")];
            interact
                .add_nodes(bls_keys, vec![&args.verified_message])
                .await;
        },
        Some(delegation_sc_interact_cli::InteractCliCommand::StakeNode(args)) => {
            let bls_keys =
                vec![hex::decode(args.public_key.clone())
                    .expect("Failed to decode public key from hex")];
            interact.stake_nodes(bls_keys).await;
        },
        Some(delegation_sc_interact_cli::InteractCliCommand::Delegate) => {},
        None => {},
    }
}

pub struct DelegateCallsInteract {
    interactor: Interactor,
    wallet_address: Bech32Address,
    #[allow(unused)]
    state: State,
}

impl DelegateCallsInteract {
    pub async fn new(config: Config) -> Self {
        let mut interactor = Interactor::new(config.gateway_uri())
            .await
            .use_chain_simulator(config.is_chain_simulator());

        interactor.set_current_dir_from_workspace("tools/interactor-delegation-func-calls");
        let wallet_address = interactor.register_wallet(test_wallets::alice()).await;

        // generate blocks until ESDTSystemSCAddress is enabled
        interactor.generate_blocks_until_epoch(1).await.unwrap();

        Self {
            interactor,
            wallet_address: wallet_address.into(),
            state: State::load_state(),
        }
    }

    pub async fn set_state(&mut self) {
        let mut account = self
            .interactor
            .get_account(&self.wallet_address.to_address())
            .await;
        account.balance = "20000000000000000000000000".to_owned();
        let set_state_account = SetStateAccount::from(account);
        let vec_state = vec![set_state_account];

        let _set_state_response = self.interactor.set_state(vec_state).await;
    }

    pub async fn create_new_delegation_contract(
        &mut self,
        total_delegation_cap: u128,
        service_fee: u64,
    ) {
        let logs = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(DelegationManagerSCAddress)
            .typed(DelegationManagerSCProxy)
            .create_new_delegation_contract(BigUint::from(total_delegation_cap), service_fee)
            .gas(60_000_000u64)
            .returns(ReturnsLogs)
            .run()
            .await;

        let deploy_log = logs
            .into_iter()
            .find(|log| log.endpoint == "SCDeploy")
            .expect("No SCDeploy log found");

        let decode_deploy_address = base64_decode(deploy_log.topics[0].clone());

        let deploy_address = Bech32Address::from(Address::from_slice(&decode_deploy_address));
        self.state.set_delegation_address(deploy_address);

        println!(
            "New delegation contract deployed at: {}",
            self.state.current_delegation_address().to_bech32_expr()
        );
    }

    pub async fn set_metadata(&mut self, name: &str, website: &str, identifier: &str) {
        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_delegation_address())
            .typed(DelegationSCProxy)
            .set_metadata(name, website, identifier)
            .gas(60_000_000u64)
            .run()
            .await;

        println!("Metadata set successfully");
    }

    pub async fn change_service_fee(&mut self, service_fee: u64) {
        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_delegation_address())
            .typed(DelegationSCProxy)
            .change_service_fee(service_fee)
            .gas(60_000_000u64)
            .run()
            .await;

        println!("Change service fee successfully");
    }

    pub async fn set_automatic_activation(&mut self, automatic_activation: bool) {
        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_delegation_address())
            .typed(DelegationSCProxy)
            .set_automatic_activation(automatic_activation)
            .gas(60_000_000u64)
            .run()
            .await;

        println!("Automatic activation set.");
    }

    pub async fn modify_total_delegation_cap(&mut self, total_delegation_cap: u128) {
        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_delegation_address())
            .typed(DelegationSCProxy)
            .modify_total_delegation_cap(BigUint::from(total_delegation_cap))
            .gas(60_000_000u64)
            .run()
            .await;

        println!("Automatic activation set.");
    }

    pub async fn add_nodes(&mut self, bls_keys: Vec<Vec<u8>>, verified_messages: Vec<&str>) {
        let managed_bls_keys: ManagedVec<StaticApi, ManagedBuffer<StaticApi>> = bls_keys
            .into_iter()
            .map(|key| ManagedBuffer::from(key))
            .collect();
        let managed_verified_messages: ManagedVec<StaticApi, ManagedBuffer<StaticApi>> =
            verified_messages
                .into_iter()
                .map(|verified_message| ManagedBuffer::from(verified_message))
                .collect();

        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_delegation_address())
            .typed(DelegationSCProxy)
            .add_nodes(&managed_bls_keys, &managed_verified_messages)
            .gas(60_000_000u64)
            .run()
            .await;

        println!("Nodes added successfully");
    }

    pub async fn stake_nodes(&mut self, bls_keys: Vec<Vec<u8>>) {
        let managed_bls_keys: ManagedVec<StaticApi, ManagedBuffer<StaticApi>> = bls_keys
            .into_iter()
            .map(|key| ManagedBuffer::from(key))
            .collect();

        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_delegation_address())
            .typed(DelegationSCProxy)
            .stake_nodes(&managed_bls_keys)
            .gas(1000000u64 + managed_bls_keys.len() as u64 * 6000000u64)
            .run()
            .await;

        println!("Nodes staked successfully");
    }

    pub async fn delegate(&mut self, egld_value: u128) {
        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_delegation_address())
            .typed(DelegationSCProxy)
            .delegate(BigUint::from(egld_value)) // Example delegation amount
            .gas(12000000)
            .run()
            .await;

        println!("Delegate successfully");
    }
}
