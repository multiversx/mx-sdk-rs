mod governance_sc_interact_cli;
mod governance_sc_interact_config;
mod governance_sc_interact_state;

use clap::Parser;
pub use governance_sc_interact_config::Config;
use governance_sc_interact_state::State;

use multiversx_sc_snippets::{
    imports::*,
    sdk::{gateway::SetStateAccount, utils::base64_decode},
};

pub async fn governance_sc_interact_cli() {
    env_logger::init();

    let mut interactor = GovernanceCallsInteract::new(Config::load_config()).await;

    let cli = governance_sc_interact_cli::InteractCli::parse();
    match cli.command {
        Some(governance_sc_interact_cli::InteractCliCommand::Propose(_args)) => {
            interactor.proposal_hardcoded().await;
            // interactor
            //     .proposal(
            //         &Bech32Address::from_bech32_string(args.from).to_address(),
            //         &args.commit_hash,
            //         args.start_vote_epoch,
            //         args.end_vote_epoch,
            //     )
            //     .await;
        },
        Some(governance_sc_interact_cli::InteractCliCommand::ViewConfig) => {
            interactor.view_config().await;
        },
        Some(governance_sc_interact_cli::InteractCliCommand::ViewProposal(_args)) => {
            // interactor.view_proposal(args.nonce).await;
            interactor.view_proposal(4).await;
        },
        Some(governance_sc_interact_cli::InteractCliCommand::Vote(_args)) => {
            // interactor
            //     .vote(
            //         &Bech32Address::from_bech32_string(args.from),
            //         args.nonce,
            //         &args.vote,
            //     )
            //     .await;
            interactor.vote_hardcoded().await;
        },
        Some(governance_sc_interact_cli::InteractCliCommand::DelegateVote(args)) => {
            interactor
                .delegate_vote(
                    &Bech32Address::from_bech32_string(args.from),
                    args.nonce,
                    &args.vote,
                    &Bech32Address::from_bech32_string(args.voter),
                    args.stake,
                    args.error.as_deref(),
                )
                .await;
        },
        Some(governance_sc_interact_cli::InteractCliCommand::Stake(args)) => {
            let bls_key = BLSKey::from_vec(args.bls_key.into_bytes());
            let bls_signature = BLSSignature::from_vec(args.bls_signature.into_bytes());
            let bls_keys_signatures = vec![(
                bls_key.expect("not BLSKey format"),
                bls_signature.expect("not BLSSignature format"),
            )];

            interactor
                .stake(
                    Bech32Address::from_bech32_string(args.from),
                    args.maximum_staked_nodes,
                    bls_keys_signatures,
                    args.amount,
                )
                .await;
        },
        None => {},
    }
}

pub struct GovernanceCallsInteract {
    pub interactor: Interactor,
    pub owner: Bech32Address,
    pub user1: Bech32Address,
    pub user2: Bech32Address,
    pub delegator: Bech32Address,
    pub state: State,
}

impl GovernanceCallsInteract {
    pub async fn new(config: Config) -> Self {
        let mut interactor = Interactor::new(config.gateway_uri())
            .await
            .use_chain_simulator(config.is_chain_simulator());

        interactor.set_current_dir_from_workspace("tools/interactor-governance-func-calls");
        let owner = interactor.register_wallet(test_wallets::eve()).await;
        let user1 = interactor.register_wallet(test_wallets::mike()).await;
        let user2 = interactor.register_wallet(test_wallets::judy()).await;
        let delegator = interactor.register_wallet(test_wallets::heidi()).await;

        // generate blocks until ESDTSystemSCAddress is enabled
        interactor.generate_blocks_until_epoch(1).await.unwrap();

        Self {
            interactor,
            owner: owner.into(),
            user1: user1.into(),
            user2: user2.into(),
            delegator: delegator.into(),
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

    pub async fn view_config(&mut self) {
        let result = self
            .interactor
            .query()
            .to(GovernanceSystemSCAddress)
            .typed(GovernanceSCProxy)
            .view_config()
            .returns(ReturnsResult)
            .run()
            .await;

        println!("view config: {:#?}", result);
    }

    pub async fn proposal_hardcoded(&mut self) {
        let user_address = self
            .interactor
            .register_wallet(Wallet::from_pem_file("testnet-delegator1.pem").unwrap())
            .await;

        let epoch = 1004;
        self.proposal(
            &user_address,
            &format!("aaaaaaaaaaaaaaaaaaaa0000000000000000{epoch}"),
            epoch,
            epoch,
        )
        .await;
    }

    pub async fn proposal(
        &mut self,
        sender: &Address,
        commit_hash: &str,
        start_vote_epoch: usize,
        end_vote_epoch: usize,
    ) {
        println!("proposing hash: {commit_hash}, start epoch: {start_vote_epoch}, end epoch: {end_vote_epoch}");

        let logs = self
            .interactor
            .tx()
            .from(sender)
            .to(GovernanceSystemSCAddress)
            .typed(GovernanceSCProxy)
            .proposal(commit_hash, start_vote_epoch, end_vote_epoch)
            .gas(60_000_000u64)
            .returns(ReturnsLogs)
            .run()
            .await;

        for log in logs {
            if log.endpoint == "proposal" && log.topics.len() >= 4 {
                let nonce = base64_decode(&log.topics[0]);
                println!("proposal nonce: {:?}", nonce);
            }
        }
    }

    pub async fn view_proposal(&mut self, nonce: u64) {
        let result = self
            .interactor
            .query()
            .to(GovernanceSystemSCAddress)
            .typed(GovernanceSCProxy)
            .view_proposal(nonce)
            .returns(ReturnsResult)
            .run()
            .await;

        println!("view proposal with nonce {nonce}: {:#?}", result);
    }

    pub async fn vote(&mut self, sender: &Bech32Address, nonce: usize, vote_type: &str) {
        self.interactor
            .tx()
            .from(sender)
            .to(GovernanceSystemSCAddress)
            .typed(GovernanceSCProxy)
            .vote(nonce, vote_type)
            .gas(60_000_000u64)
            .run()
            .await;
    }

    /// Temporary, some hardcoded values for quicker testing.
    pub async fn vote_hardcoded(&mut self) {
        let user_address = self
            .interactor
            .register_wallet(Wallet::from_pem_file("delegator2.pem").unwrap())
            .await;
        self.vote(
            &Bech32Address::encode_address_default_hrp(user_address),
            4,
            "yes",
        )
        .await;
    }

    pub async fn delegate_vote(
        &mut self,
        sender: &Bech32Address,
        nonce: u64,
        vote: &str,
        voter: &Bech32Address,
        stake: u64,
        err_message: Option<&str>,
    ) {
        let response = self
            .interactor
            .tx()
            .from(sender)
            .to(GovernanceSystemSCAddress)
            .typed(GovernanceSCProxy)
            .delegate_vote(nonce, vote, voter, stake)
            .gas(60_000_000u64)
            .returns(ReturnsHandledOrError::new())
            .run()
            .await;

        match response {
            Ok(_) => println!("Delegate vote successfully done!"),
            Err(err) => {
                if err_message.is_some() {
                    assert_eq!(err_message.unwrap(), err.message.to_string());
                } else {
                    panic!("Unexpected error: {}", err);
                }
            },
        };
    }

    pub async fn stake(
        &mut self,
        sender: Bech32Address,
        maximum_staked_nodes: usize,
        bls_keys_signatures: Vec<(BLSKey, BLSSignature)>,
        amount: u128,
    ) {
        let managed_bls_keys_signatures = MultiValueVec::from(
            bls_keys_signatures
                .into_iter()
                .map(MultiValue2::from)
                .collect::<Vec<_>>(),
        );

        let total_amount: BigUint<StaticApi> =
            BigUint::from(amount) * BigUint::from(managed_bls_keys_signatures.len());

        self.interactor
            .tx()
            .from(sender)
            .to(ValidatorSystemSCAddress)
            .typed(ValidatorSCProxy)
            .stake(
                maximum_staked_nodes,
                managed_bls_keys_signatures,
                total_amount,
            )
            .gas(60_000_000u64)
            .run()
            .await;

        println!("Stake successfully done!");
    }
}
