#![allow(deprecated)] // TODO: migrate tests

use multiversx_sc::types::{Address, ManagedVec, MultiValueEncoded};
use multiversx_sc_modules::governance::{
    governance_configurable::GovernanceConfigurablePropertiesModule, governance_proposal::VoteType,
    GovernanceModule,
};
use multiversx_sc_scenario::{
    managed_address, managed_biguint, managed_buffer, managed_token_id, rust_biguint,
    testing_framework::{BlockchainStateWrapper, ContractObjWrapper, TxResult},
    DebugApi,
};

static GOV_TOKEN_ID: &[u8] = b"GOV-123456";
const QUORUM: u64 = 1_500;
const MIN_BALANCE_PROPOSAL: u64 = 500;
const VOTING_DELAY_BLOCKS: u64 = 10;
const VOTING_PERIOD_BLOCKS: u64 = 20;
const LOCKING_PERIOD_BLOCKS: u64 = 30;

const INITIAL_GOV_TOKEN_BALANCE: u64 = 1_000;
const GAS_LIMIT: u64 = 1_000_000;

pub struct Payment {
    pub token: Vec<u8>,
    pub nonce: u64,
    pub amount: u64,
}

pub struct GovSetup<GovBuilder>
where
    GovBuilder: 'static + Copy + Fn() -> use_module::ContractObj<DebugApi>,
{
    pub b_mock: BlockchainStateWrapper,
    pub owner: Address,
    pub first_user: Address,
    pub second_user: Address,
    pub third_user: Address,
    pub gov_wrapper: ContractObjWrapper<use_module::ContractObj<DebugApi>, GovBuilder>,
    pub current_block: u64,
}

impl<GovBuilder> GovSetup<GovBuilder>
where
    GovBuilder: 'static + Copy + Fn() -> use_module::ContractObj<DebugApi>,
{
    pub fn new(gov_builder: GovBuilder) -> Self {
        let rust_zero = rust_biguint!(0);
        let initial_gov = rust_biguint!(INITIAL_GOV_TOKEN_BALANCE);

        let mut b_mock = BlockchainStateWrapper::new();

        let owner = b_mock.create_user_account(&rust_zero);
        b_mock.set_esdt_balance(&owner, GOV_TOKEN_ID, &initial_gov);

        let first_user = b_mock.create_user_account(&rust_zero);
        b_mock.set_esdt_balance(&first_user, GOV_TOKEN_ID, &initial_gov);

        let second_user = b_mock.create_user_account(&rust_zero);
        b_mock.set_esdt_balance(&second_user, GOV_TOKEN_ID, &initial_gov);

        let third_user = b_mock.create_user_account(&rust_zero);
        b_mock.set_esdt_balance(&third_user, GOV_TOKEN_ID, &initial_gov);

        let gov_wrapper =
            b_mock.create_sc_account(&rust_zero, Some(&owner), gov_builder, "gov path");

        b_mock
            .execute_tx(&owner, &gov_wrapper, &rust_zero, |sc| {
                sc.init_governance_module(
                    managed_token_id!(GOV_TOKEN_ID),
                    managed_biguint!(QUORUM),
                    managed_biguint!(MIN_BALANCE_PROPOSAL),
                    VOTING_DELAY_BLOCKS,
                    VOTING_PERIOD_BLOCKS,
                    LOCKING_PERIOD_BLOCKS,
                );
            })
            .assert_ok();

        b_mock.set_block_nonce(10);

        Self {
            b_mock,
            owner,
            first_user,
            second_user,
            third_user,
            gov_wrapper,
            current_block: 10,
        }
    }

    pub fn propose(
        &mut self,
        proposer: &Address,
        gov_token_amount: u64,
        dest_address: &Address,
        endpoint_name: &[u8],
        args: Vec<Vec<u8>>,
    ) -> (TxResult, usize) {
        let mut proposal_id = 0;
        let result = self.b_mock.execute_esdt_transfer(
            proposer,
            &self.gov_wrapper,
            GOV_TOKEN_ID,
            0,
            &rust_biguint!(gov_token_amount),
            |sc| {
                let mut args_managed = ManagedVec::new();
                for arg in args {
                    args_managed.push(managed_buffer!(&arg));
                }

                let mut actions = MultiValueEncoded::new();
                actions.push(
                    (
                        GAS_LIMIT,
                        managed_address!(dest_address),
                        managed_buffer!(endpoint_name),
                        args_managed,
                    )
                        .into(),
                );

                proposal_id = sc.propose(managed_buffer!(b"change quorum"), actions);
            },
        );

        (result, proposal_id)
    }

    pub fn vote(&mut self, voter: &Address, proposal_id: usize, gov_token_amount: u64) -> TxResult {
        self.b_mock.execute_esdt_transfer(
            voter,
            &self.gov_wrapper,
            GOV_TOKEN_ID,
            0,
            &rust_biguint!(gov_token_amount),
            |sc| {
                sc.vote(proposal_id, VoteType::UpVote);
            },
        )
    }

    pub fn down_vote(
        &mut self,
        voter: &Address,
        proposal_id: usize,
        gov_token_amount: u64,
    ) -> TxResult {
        self.b_mock.execute_esdt_transfer(
            voter,
            &self.gov_wrapper,
            GOV_TOKEN_ID,
            0,
            &rust_biguint!(gov_token_amount),
            |sc| {
                sc.vote(proposal_id, VoteType::DownVote);
            },
        )
    }

    pub fn down_veto_vote(
        &mut self,
        voter: &Address,
        proposal_id: usize,
        gov_token_amount: u64,
    ) -> TxResult {
        self.b_mock.execute_esdt_transfer(
            voter,
            &self.gov_wrapper,
            GOV_TOKEN_ID,
            0,
            &rust_biguint!(gov_token_amount),
            |sc| {
                sc.vote(proposal_id, VoteType::DownVetoVote);
            },
        )
    }

    pub fn abstain_vote(
        &mut self,
        voter: &Address,
        proposal_id: usize,
        gov_token_amount: u64,
    ) -> TxResult {
        self.b_mock.execute_esdt_transfer(
            voter,
            &self.gov_wrapper,
            GOV_TOKEN_ID,
            0,
            &rust_biguint!(gov_token_amount),
            |sc| {
                sc.vote(proposal_id, VoteType::AbstainVote);
            },
        )
    }

    pub fn queue(&mut self, proposal_id: usize) -> TxResult {
        self.b_mock.execute_tx(
            &self.first_user,
            &self.gov_wrapper,
            &rust_biguint!(0),
            |sc| {
                sc.queue(proposal_id);
            },
        )
    }

    pub fn execute(&mut self, proposal_id: usize) -> TxResult {
        self.b_mock.execute_tx(
            &self.first_user,
            &self.gov_wrapper,
            &rust_biguint!(0),
            |sc| {
                sc.execute(proposal_id);
            },
        )
    }

    pub fn cancel(&mut self, caller: &Address, proposal_id: usize) -> TxResult {
        self.b_mock
            .execute_tx(caller, &self.gov_wrapper, &rust_biguint!(0), |sc| {
                sc.cancel(proposal_id);
            })
    }

    pub fn increment_block_nonce(&mut self, inc_amount: u64) {
        self.current_block += inc_amount;
        self.b_mock.set_block_nonce(self.current_block);
    }

    pub fn set_block_nonce(&mut self, block_nonce: u64) {
        self.current_block = block_nonce;
        self.b_mock.set_block_nonce(self.current_block);
    }
}

#[test]
fn init_gov_test() {
    let _ = GovSetup::new(use_module::contract_obj);
}

#[test]
fn change_gov_config_test() {
    let mut gov_setup = GovSetup::new(use_module::contract_obj);

    let owner_addr = gov_setup.owner.clone();
    let first_user_addr = gov_setup.first_user.clone();
    let second_user_addr = gov_setup.second_user.clone();
    let third_user_addr = gov_setup.third_user.clone();
    let sc_addr = gov_setup.gov_wrapper.address_ref().clone();

    let (result, proposal_id) = gov_setup.propose(
        &first_user_addr,
        500,
        &sc_addr,
        b"changeQuorum",
        vec![1_000u64.to_be_bytes().to_vec()],
    );
    result.assert_ok();
    assert_eq!(proposal_id, 1);

    // vote too early
    gov_setup
        .vote(&second_user_addr, proposal_id, 999)
        .assert_user_error("Proposal is not active");

    gov_setup.increment_block_nonce(VOTING_DELAY_BLOCKS);

    gov_setup
        .vote(&second_user_addr, proposal_id, 999)
        .assert_ok();

    // try execute before queue
    gov_setup
        .execute(proposal_id)
        .assert_user_error("Can only execute queued proposals");

    // try queue before voting ends
    gov_setup
        .queue(proposal_id)
        .assert_user_error("Can only queue succeeded proposals");

    gov_setup.increment_block_nonce(VOTING_PERIOD_BLOCKS);

    // try queue not enough votes
    gov_setup
        .queue(proposal_id)
        .assert_user_error("Can only queue succeeded proposals");

    // user 1 vote again
    gov_setup.set_block_nonce(20);
    gov_setup
        .vote(&first_user_addr, proposal_id, 200)
        .assert_ok();

    // owner downvote
    gov_setup
        .down_vote(&owner_addr, proposal_id, 200)
        .assert_ok();

    // try queue too many downvotes
    gov_setup.set_block_nonce(45);
    gov_setup
        .queue(proposal_id)
        .assert_user_error("Can only queue succeeded proposals");

    // user 1 vote again
    gov_setup.set_block_nonce(20);
    gov_setup
        .vote(&first_user_addr, proposal_id, 200)
        .assert_user_error("Already voted for this proposal");

    // user 3 vote again
    gov_setup
        .vote(&third_user_addr, proposal_id, 200)
        .assert_ok();

    // queue ok
    gov_setup.set_block_nonce(45);
    gov_setup.queue(proposal_id).assert_ok();

    // try execute too early
    gov_setup
        .execute(proposal_id)
        .assert_user_error("Proposal is in timelock status. Try again later");

    // execute ok
    gov_setup.increment_block_nonce(LOCKING_PERIOD_BLOCKS);
    gov_setup.execute(proposal_id).assert_ok();

    // after execution, quorum changed from 1_500 to the proposed 1_000
    gov_setup
        .b_mock
        .execute_query(&gov_setup.gov_wrapper, |sc| {
            assert_eq!(sc.quorum().get(), managed_biguint!(1_000));
            assert!(sc.proposals().item_is_empty(1));
        })
        .assert_ok();

    gov_setup
        .b_mock
        .check_esdt_balance(&first_user_addr, GOV_TOKEN_ID, &rust_biguint!(300));
    gov_setup
        .b_mock
        .check_esdt_balance(&second_user_addr, GOV_TOKEN_ID, &rust_biguint!(1));
    gov_setup
        .b_mock
        .check_esdt_balance(&third_user_addr, GOV_TOKEN_ID, &rust_biguint!(800));
    gov_setup
        .b_mock
        .check_esdt_balance(&owner_addr, GOV_TOKEN_ID, &rust_biguint!(800));
}

#[test]
fn down_veto_gov_config_test() {
    let mut gov_setup = GovSetup::new(use_module::contract_obj);

    let first_user_addr = gov_setup.first_user.clone();
    let second_user_addr = gov_setup.second_user.clone();
    let third_user_addr = gov_setup.third_user.clone();
    let sc_addr = gov_setup.gov_wrapper.address_ref().clone();

    let (result, proposal_id) = gov_setup.propose(
        &first_user_addr,
        500,
        &sc_addr,
        b"changeQuorum",
        vec![1_000u64.to_be_bytes().to_vec()],
    );
    result.assert_ok();
    assert_eq!(proposal_id, 1);

    gov_setup.increment_block_nonce(VOTING_DELAY_BLOCKS);

    gov_setup
        .vote(&first_user_addr, proposal_id, 300)
        .assert_ok();

    gov_setup.increment_block_nonce(VOTING_PERIOD_BLOCKS);

    // user 1 vote again
    gov_setup.set_block_nonce(20);
    gov_setup
        .vote(&second_user_addr, proposal_id, 200)
        .assert_ok();

    // user 3 vote again
    gov_setup
        .down_veto_vote(&third_user_addr, proposal_id, 200)
        .assert_ok();

    // Vote didn't succeed;
    gov_setup.set_block_nonce(45);
    gov_setup
        .queue(proposal_id)
        .assert_user_error("Can only queue succeeded proposals");

    gov_setup
        .b_mock
        .check_esdt_balance(&first_user_addr, GOV_TOKEN_ID, &rust_biguint!(200));
    gov_setup
        .b_mock
        .check_esdt_balance(&second_user_addr, GOV_TOKEN_ID, &rust_biguint!(800));
    gov_setup
        .b_mock
        .check_esdt_balance(&third_user_addr, GOV_TOKEN_ID, &rust_biguint!(800));
}

#[test]
fn abstain_vote_gov_config_test() {
    let mut gov_setup = GovSetup::new(use_module::contract_obj);

    let first_user_addr = gov_setup.first_user.clone();
    let second_user_addr = gov_setup.second_user.clone();
    let third_user_addr = gov_setup.third_user.clone();
    let sc_addr = gov_setup.gov_wrapper.address_ref().clone();

    let (result, proposal_id) = gov_setup.propose(
        &first_user_addr,
        500,
        &sc_addr,
        b"changeQuorum",
        vec![1_000u64.to_be_bytes().to_vec()],
    );
    result.assert_ok();
    assert_eq!(proposal_id, 1);

    gov_setup.increment_block_nonce(VOTING_DELAY_BLOCKS);

    gov_setup
        .vote(&first_user_addr, proposal_id, 500)
        .assert_ok();

    gov_setup.increment_block_nonce(VOTING_PERIOD_BLOCKS);

    // user 1 vote again
    gov_setup.set_block_nonce(20);
    gov_setup
        .down_vote(&second_user_addr, proposal_id, 400)
        .assert_ok();

    // user 3 vote again
    gov_setup
        .abstain_vote(&third_user_addr, proposal_id, 600)
        .assert_ok();

    // Vote didn't succeed;
    gov_setup.set_block_nonce(45);
    gov_setup.queue(proposal_id).assert_ok();

    // execute ok
    gov_setup.increment_block_nonce(LOCKING_PERIOD_BLOCKS);
    gov_setup.execute(proposal_id).assert_ok();

    // after execution, quorum changed from 1_500 to the proposed 1_000
    gov_setup
        .b_mock
        .execute_query(&gov_setup.gov_wrapper, |sc| {
            assert_eq!(sc.quorum().get(), managed_biguint!(1_000));
            assert!(sc.proposals().item_is_empty(1));
        })
        .assert_ok();

    gov_setup
        .b_mock
        .check_esdt_balance(&first_user_addr, GOV_TOKEN_ID, &rust_biguint!(0));
    gov_setup
        .b_mock
        .check_esdt_balance(&second_user_addr, GOV_TOKEN_ID, &rust_biguint!(600));
    gov_setup
        .b_mock
        .check_esdt_balance(&third_user_addr, GOV_TOKEN_ID, &rust_biguint!(400));
}

#[test]
fn gov_cancel_defeated_proposal_test() {
    let mut gov_setup = GovSetup::new(use_module::contract_obj);

    let first_user_addr = gov_setup.first_user.clone();
    let second_user_addr = gov_setup.second_user.clone();
    let sc_addr = gov_setup.gov_wrapper.address_ref().clone();
    let (result, proposal_id) = gov_setup.propose(
        &first_user_addr,
        500,
        &sc_addr,
        b"changeQuorum",
        vec![1_000u64.to_be_bytes().to_vec()],
    );
    result.assert_ok();
    assert_eq!(proposal_id, 1);

    gov_setup.increment_block_nonce(VOTING_DELAY_BLOCKS);
    gov_setup
        .down_vote(&second_user_addr, proposal_id, 999)
        .assert_ok();

    // try cancel too early
    gov_setup
        .cancel(&second_user_addr, proposal_id)
        .assert_user_error("Action may not be cancelled");

    gov_setup.increment_block_nonce(VOTING_PERIOD_BLOCKS);
    gov_setup.cancel(&second_user_addr, proposal_id).assert_ok();
}
