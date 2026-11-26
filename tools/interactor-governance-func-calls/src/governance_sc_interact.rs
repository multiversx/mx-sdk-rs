mod governance_sc_interact_cli;
mod governance_sc_interact_config;
mod governance_sc_interact_state;

use clap::Parser;
pub use governance_sc_interact_config::Config;
use governance_sc_interact_state::State;

use multiversx_sc_snippets::{
    imports::*,
    sdk::{chain_core::std::base64_decode, gateway::SetStateAccount},
};

pub async fn governance_sc_interact_cli() {
    env_logger::init();

    let mut interactor = GovernanceCallsInteract::new(Config::load_config()).await;

    let cli = governance_sc_interact_cli::InteractCli::parse();
    match cli.command {
        Some(governance_sc_interact_cli::InteractCliCommand::Propose(args)) => {
            interactor
                .proposal(
                    &Bech32Address::from_bech32_string(args.from).to_address(),
                    &args.commit_hash,
                    args.start_vote_epoch,
                    args.end_vote_epoch,
                )
                .await;
        }
        Some(governance_sc_interact_cli::InteractCliCommand::ViewConfig) => {
            interactor.view_config().await;
        }
        Some(governance_sc_interact_cli::InteractCliCommand::ViewProposal(args)) => {
            interactor.view_proposal(args.nonce).await;
        }
        Some(governance_sc_interact_cli::InteractCliCommand::Vote(args)) => {
            interactor
                .vote(
                    &Bech32Address::from_bech32_string(args.from),
                    args.nonce,
                    &args.vote,
                )
                .await;
        }
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
        }
        Some(governance_sc_interact_cli::InteractCliCommand::Stake(args)) => {
            let bls_key = BLSKey::parse_hex(&args.bls_key).expect("invalid BLS key hex");
            let bls_signature =
                BLSSignature::parse_hex(&args.bls_signature).expect("invalid BLS signature hex");
            let bls_keys_signatures = vec![(bls_key, bls_signature)];

            interactor
                .stake(
                    Bech32Address::from_bech32_string(args.from),
                    bls_keys_signatures,
                    args.amount,
                )
                .await;
        }
        None => {}
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
            .use_chain_simulator(config.is_chain_simulator())
            .with_current_dir(env!("CARGO_MANIFEST_DIR"));
        let owner = interactor.register_wallet(test_wallets::eve()).await;
        let user1 = interactor.register_wallet(test_wallets::mike()).await;
        let user2 = interactor.register_wallet(test_wallets::judy()).await;
        let delegator = interactor.register_wallet(test_wallets::heidi()).await;

        // generate blocks until ESDTSystemSCAddress is enabled
        interactor.generate_blocks_until_all_activations().await;

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

    pub async fn view_config(&mut self) -> GovernanceConfigResult {
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
        result
    }

    pub async fn proposal(
        &mut self,
        sender: &Address,
        commit_hash: &str,
        start_vote_epoch: u64,
        end_vote_epoch: u64,
    ) {
        println!(
            "proposing hash: {commit_hash}, start epoch: {start_vote_epoch}, end epoch: {end_vote_epoch}"
        );

        let proposal_fee: u128 = self
            .view_config()
            .await
            .proposal_fee
            .parse()
            .expect("invalid proposal fee");

        let logs = self
            .interactor
            .tx()
            .from(sender)
            .to(GovernanceSystemSCAddress)
            .typed(GovernanceSCProxy)
            .proposal(commit_hash, start_vote_epoch, end_vote_epoch, proposal_fee)
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

        println!(
            r#"view proposal with nonce {nonce}:
    proposal_cost: {},
    commit_hash: {},
    proposal_nonce: {},
    issuer_address: {},
    start_vote_epoch: {},
    end_vote_epoch: {},
    quorum_stake: {},
    yes: {},
    no: {},
    veto: {},
    abstain: {},
    closed: {},
    passed: {},"#,
            result.proposal_cost.to_display(),
            result.commit_hash,
            result.proposal_nonce,
            Bech32Address::from(result.issuer_address).to_bech32_expr(),
            result.start_vote_epoch,
            result.end_vote_epoch,
            result.quorum_stake.to_display(),
            result.yes.to_display(),
            result.no.to_display(),
            result.veto.to_display(),
            result.abstain.to_display(),
            result.closed,
            result.passed
        );
    }

    pub async fn vote(&mut self, sender: &Bech32Address, nonce: u64, vote_type: &str) {
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
                if let Some(expected_msg) = err_message {
                    assert_eq!(expected_msg, err.message.to_string());
                } else {
                    panic!("Unexpected error: {}", err);
                }
            }
        };
    }

    pub async fn stake(
        &mut self,
        sender: Bech32Address,
        bls_keys_signatures: Vec<(BLSKey, BLSSignature)>,
        amount: u128,
    ) {
        let managed_bls_keys_signatures = MultiValueVec::from(
            bls_keys_signatures
                .into_iter()
                .map(MultiValue2::from)
                .collect::<Vec<_>>(),
        );

        let total_amount = amount * managed_bls_keys_signatures.len() as u128;

        self.interactor
            .tx()
            .from(sender)
            .to(ValidatorSystemSCAddress)
            .typed(ValidatorSCProxy)
            .stake(managed_bls_keys_signatures, total_amount)
            .gas(60_000_000u64)
            .run()
            .await;

        println!("Stake successfully done!");
    }
}
