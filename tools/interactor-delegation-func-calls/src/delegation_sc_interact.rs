mod delegation_sc_interact_cli;
mod delegation_sc_interact_config;
mod delegation_sc_interact_state;

use std::vec;

use clap::Parser;
pub use delegation_sc_interact_config::Config;
use delegation_sc_interact_state::State;

use multiversx_sc_snippets::{
    imports::*,
    sdk::{gateway::SetStateAccount, utils::base64_decode},
};

pub async fn delegation_sc_interact_cli() {
    env_logger::init();

    let mut interact = DelegateCallsInteract::new(Config::load_config()).await;

    let cli = delegation_sc_interact_cli::InteractCli::parse();
    match cli.command {
        Some(delegation_sc_interact_cli::InteractCliCommand::Create(args)) => {
            interact.set_state(&interact.owner.to_address()).await;
            interact
                .create_new_delegation_contract(
                    args.total_delegation_cap,
                    args.service_fee,
                    1250000000000000000000u128,
                )
                .await;
        },
        Some(delegation_sc_interact_cli::InteractCliCommand::GetAllContractAddresses) => {
            interact.get_all_contract_addresses().await;
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
            let bls_key = BLSKey::parse_hex(&args.public_key)
                .expect("Failed to decode public BLS key from hex");
            let bls_sig = BLSSignature::parse_hex(&args.verified_message)
                .expect("Failed to decode BLS signature from hex");
            interact.add_nodes(vec![(bls_key, bls_sig)]).await;
        },
        Some(delegation_sc_interact_cli::InteractCliCommand::GetAllNodeStates) => {
            interact.get_all_node_states().await;
        },
        Some(delegation_sc_interact_cli::InteractCliCommand::StakeNode(args)) => {
            let bls_key = BLSKey::parse_hex(&args.public_key)
                .expect("Failed to decode public BLS key from hex");
            interact.stake_nodes(vec![bls_key]).await;
        },
        Some(delegation_sc_interact_cli::InteractCliCommand::UnstakeNode(args)) => {
            let bls_key = BLSKey::parse_hex(&args.public_key)
                .expect("Failed to decode public BLS key from hex");
            interact.unstake_nodes(vec![bls_key]).await;
        },
        Some(delegation_sc_interact_cli::InteractCliCommand::RestakeNode(args)) => {
            let bls_key = BLSKey::parse_hex(&args.public_key)
                .expect("Failed to decode public BLS key from hex");
            interact.restake_unstaked_nodes(vec![bls_key]).await;
        },
        Some(delegation_sc_interact_cli::InteractCliCommand::UnbondNode(args)) => {
            let bls_key = BLSKey::parse_hex(&args.public_key)
                .expect("Failed to decode public BLS key from hex");
            interact.unbond_nodes(vec![bls_key]).await;
        },
        Some(delegation_sc_interact_cli::InteractCliCommand::RemoveNode(args)) => {
            let bls_key = BLSKey::parse_hex(&args.public_key)
                .expect("Failed to decode public BLS key from hex");
            interact.remove_nodes(vec![bls_key]).await;
        },
        Some(delegation_sc_interact_cli::InteractCliCommand::UnjailNode(args)) => {
            let bls_key = BLSKey::parse_hex(&args.public_key)
                .expect("Failed to decode public BLS key from hex");
            interact.unjail_nodes(vec![bls_key]).await;
        },
        Some(delegation_sc_interact_cli::InteractCliCommand::Delegate(args)) => {
            let sender = Bech32Address::from_bech32_string(args.from.clone());
            interact.delegate(&sender, args.egld).await;
        },
        Some(delegation_sc_interact_cli::InteractCliCommand::ClaimRewards(args)) => {
            let sender = Bech32Address::from_bech32_string(args.from.clone());
            interact.claim_rewards(&sender).await;
        },
        Some(delegation_sc_interact_cli::InteractCliCommand::RedelegateRewards(args)) => {
            let sender = Bech32Address::from_bech32_string(args.from.clone());
            interact.redelegate_rewards(&sender).await;
        },
        Some(delegation_sc_interact_cli::InteractCliCommand::UndelegateFunds(args)) => {
            let sender = Bech32Address::from_bech32_string(args.from.clone());
            interact.undelegate(&sender, args.egld).await;
        },
        Some(delegation_sc_interact_cli::InteractCliCommand::Withdraw(args)) => {
            let sender = Bech32Address::from_bech32_string(args.from.clone());
            interact.withdraw(&sender).await;
        },
        Some(delegation_sc_interact_cli::InteractCliCommand::SetCheckCapOnRedelegateRewards(
            args,
        )) => {
            interact
                .set_check_cap_on_redelegate_rewards(args.check_cap_redelegate_rewards)
                .await;
        },
        None => {},
    }
}

pub struct DelegateCallsInteract {
    pub interactor: Interactor,
    pub owner: Bech32Address,
    pub delegator1: Bech32Address,
    pub delegator2: Bech32Address,
    pub state: State,
}

impl DelegateCallsInteract {
    pub async fn new(config: Config) -> Self {
        let mut interactor = Interactor::new(config.gateway_uri())
            .await
            .use_chain_simulator(config.is_chain_simulator());

        interactor.set_current_dir_from_workspace("tools/interactor-delegation-func-calls");
        let wallet_address = interactor.register_wallet(test_wallets::alice()).await;
        let delegator1 = interactor.register_wallet(test_wallets::bob()).await;
        let delegator2 = interactor.register_wallet(test_wallets::dan()).await;

        // generate blocks until ESDTSystemSCAddress is enabled
        interactor.generate_blocks_until_epoch(1).await.unwrap();

        Self {
            interactor,
            owner: wallet_address.into(),
            delegator1: delegator1.into(),
            delegator2: delegator2.into(),
            state: State::load_state(),
        }
    }

    pub async fn set_state(&mut self, address: &Address) {
        let mut account = self.interactor.get_account(address).await;
        account.balance = "100000000000000000000000".to_owned();
        let set_state_account = SetStateAccount::from(account);
        let vec_state = vec![set_state_account];

        let _set_state_response = self.interactor.set_state(vec_state).await;
    }

    pub async fn get_balance(&mut self, address: &Address) -> u128 {
        let balance: u128 = self
            .interactor
            .get_account(address)
            .await
            .balance
            .parse()
            .unwrap();

        balance
    }

    pub async fn create_new_delegation_contract(
        &mut self,
        total_delegation_cap: u128,
        service_fee: u64,
        amount: u128,
    ) {
        let logs = self
            .interactor
            .tx()
            .from(&self.owner)
            .to(DelegationManagerSCAddress)
            .typed(DelegationManagerSCProxy)
            .create_new_delegation_contract(
                BigUint::from(total_delegation_cap),
                service_fee,
                BigUint::from(amount),
            )
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

    pub async fn get_all_contract_addresses(&mut self) -> Vec<Bech32Address> {
        let addresses = self
            .interactor
            .query()
            .to(DelegationManagerSCAddress)
            .typed(DelegationManagerSCProxy)
            .get_all_contract_addresses()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("All delegation contract addresses:");
        for address in addresses.iter() {
            println!("{}", Bech32Address::from(address).to_bech32_expr());
        }

        addresses.iter().map(Bech32Address::from).collect()
    }

    pub async fn set_metadata(&mut self, name: &str, website: &str, identifier: &str) {
        self.interactor
            .tx()
            .from(&self.owner)
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
            .from(&self.owner)
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
            .from(&self.owner)
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
            .from(&self.owner)
            .to(self.state.current_delegation_address())
            .typed(DelegationSCProxy)
            .modify_total_delegation_cap(BigUint::from(total_delegation_cap))
            .gas(60_000_000u64)
            .run()
            .await;

        println!("Automatic activation set.");
    }

    pub async fn add_nodes(&mut self, bls_keys_signatures: Vec<(BLSKey, BLSSignature)>) {
        let arg = MultiValueVec::from(
            bls_keys_signatures
                .into_iter()
                .map(MultiValue2::from)
                .collect::<Vec<_>>(),
        );

        self.interactor
            .tx()
            .from(&self.owner)
            .to(self.state.current_delegation_address())
            .typed(DelegationSCProxy)
            .add_nodes(arg)
            .gas(60_000_000u64)
            .run()
            .await;

        println!("Nodes added successfully");
    }

    pub async fn get_all_node_states(&mut self) -> String {
        let node_states = self
            .interactor
            .query()
            .to(self.state.current_delegation_address())
            .typed(DelegationSCProxy)
            .get_all_node_states()
            .returns(ReturnsResult)
            .run()
            .await;

        println!("Node states: {}", node_states);
        node_states.to_string()
    }

    pub async fn stake_nodes(&mut self, bls_keys: Vec<BLSKey>) {
        self.interactor
            .tx()
            .from(&self.owner)
            .to(self.state.current_delegation_address())
            .gas(1000000u64 + bls_keys.len() as u64 * 6000000u64)
            .typed(DelegationSCProxy)
            .stake_nodes(MultiValueVec::from(bls_keys))
            .run()
            .await;

        println!("Nodes staked successfully");
    }

    pub async fn delegate(&mut self, sender: &Bech32Address, egld_value: u128) {
        self.interactor
            .tx()
            .from(sender)
            .to(self.state.current_delegation_address())
            .typed(DelegationSCProxy)
            .delegate(BigUint::from(egld_value)) // Example delegation amount
            .gas(12000000)
            .run()
            .await;

        println!("Delegate successfully");
    }

    pub async fn get_total_active_stake(&mut self) -> RustBigUint {
        let total_stake = self
            .interactor
            .query()
            .to(self.state.current_delegation_address())
            .typed(DelegationSCProxy)
            .get_total_active_stake()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Total active stake: {}", total_stake);
        total_stake
    }

    pub async fn get_user_active_stake(&mut self) -> RustBigUint {
        let active_stake = self
            .interactor
            .query()
            .to(self.state.current_delegation_address())
            .typed(DelegationSCProxy)
            .get_user_active_stake(&ManagedAddress::from_address(&self.owner.to_address()))
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("User active stake: {}", active_stake);
        active_stake
    }

    pub async fn unstake_nodes(&mut self, bls_keys: Vec<BLSKey>) {
        self.interactor
            .tx()
            .from(&self.owner)
            .to(self.state.current_delegation_address())
            .gas(1000000u64 + bls_keys.len() as u64 * 6000000u64)
            .typed(DelegationSCProxy)
            .unstake_nodes(MultiValueVec::from(bls_keys))
            .run()
            .await;

        println!("Nodes unstaked successfully");
    }

    pub async fn restake_unstaked_nodes(&mut self, bls_keys: Vec<BLSKey>) {
        self.interactor
            .tx()
            .from(&self.owner)
            .to(self.state.current_delegation_address())
            .typed(DelegationSCProxy)
            .restake_unstaked_nodes(MultiValueVec::from(bls_keys))
            .gas(30_000_000u64)
            .run()
            .await;

        println!("Nodes restaked successfully");
    }

    pub async fn unbond_nodes(&mut self, bls_keys: Vec<BLSKey>) {
        self.interactor
            .tx()
            .from(&self.owner)
            .to(self.state.current_delegation_address())
            .gas(1000000u64 + bls_keys.len() as u64 * 6000000u64)
            .typed(DelegationSCProxy)
            .unbond_nodes(MultiValueVec::from(bls_keys))
            .run()
            .await;

        println!("Nodes unbond successfully");
    }

    pub async fn remove_nodes(&mut self, bls_keys: Vec<BLSKey>) {
        self.interactor
            .tx()
            .from(&self.owner)
            .to(self.state.current_delegation_address())
            .gas(1000000u64 + bls_keys.len() as u64 * 6000000u64)
            .typed(DelegationSCProxy)
            .remove_nodes(MultiValueVec::from(bls_keys))
            .run()
            .await;

        println!("Nodes removed successfully");
    }

    pub async fn unjail_nodes(&mut self, bls_keys: Vec<BLSKey>) {
        self.interactor
            .tx()
            .from(&self.owner)
            .to(self.state.current_delegation_address())
            .gas(1000000u64 + bls_keys.len() as u64 * 6000000u64)
            .typed(DelegationSCProxy)
            .unjail_nodes(MultiValueVec::from(bls_keys))
            .run()
            .await;

        println!("Nodes unjailed successfully");
    }

    pub async fn claim_rewards(&mut self, sender: &Bech32Address) {
        self.interactor
            .tx()
            .from(sender)
            .to(self.state.current_delegation_address())
            .typed(DelegationSCProxy)
            .claim_rewards() // Example delegation amount
            .gas(12000000)
            .run()
            .await;

        println!("Claim rewards successfully");
    }

    pub async fn redelegate_rewards(&mut self, sender: &Bech32Address) {
        self.interactor
            .tx()
            .from(sender)
            .to(self.state.current_delegation_address())
            .typed(DelegationSCProxy)
            .redelegate_rewards() // Example delegation amount
            .gas(12000000)
            .run()
            .await;

        println!("Redelegate rewards successfully");
    }

    pub async fn undelegate(&mut self, sender: &Bech32Address, egld_value: u128) {
        self.interactor
            .tx()
            .from(sender)
            .to(self.state.current_delegation_address())
            .typed(DelegationSCProxy)
            .undelegate(BigUint::from(egld_value)) // Example delegation amount
            .gas(12000000)
            .run()
            .await;

        println!("Undelegate successfully");
    }

    pub async fn withdraw(&mut self, sender: &Bech32Address) {
        self.interactor
            .tx()
            .from(sender)
            .to(self.state.current_delegation_address())
            .typed(DelegationSCProxy)
            .withdraw() // Example delegation amount
            .gas(12000000)
            .run()
            .await;
        println!("Withdraw successfully");
    }

    pub async fn set_check_cap_on_redelegate_rewards(&mut self, state: bool) {
        self.interactor
            .tx()
            .from(self.owner.clone())
            .to(self.state.current_delegation_address())
            .typed(DelegationSCProxy)
            .set_check_cap_on_redelegate_rewards(state) // Example delegation amount
            .gas(12000000)
            .run()
            .await;

        println!("Set check cap on redelegate rewards to: {}", state);
    }
}
