use multiversx_sc_modules::governance::{
    GovernanceModule, governance_configurable::GovernanceConfigurablePropertiesModule,
    governance_proposal::VoteType,
};
use multiversx_sc_scenario::imports::*;

const GOV_TOKEN_ID: TestTokenIdentifier = TestTokenIdentifier::new("GOV-123456");
const QUORUM: u64 = 1_500;
const MIN_BALANCE_PROPOSAL: u64 = 500;
const VOTING_DELAY_BLOCKS: u64 = 10;
const VOTING_PERIOD_BLOCKS: u64 = 20;
const LOCKING_PERIOD_BLOCKS: u64 = 30;

const INITIAL_GOV_TOKEN_BALANCE: u64 = 1_000;
const GAS_LIMIT: u64 = 1_000_000;

const USE_MODULE_ADDRESS: TestSCAddress = TestSCAddress::new("use-module");
const USE_MODULE_PATH_EXPR: MxscPath = MxscPath::new("mxsc:output/use-module.mxsc.json");

const OWNER_ADDRESS: TestAddress = TestAddress::new("owner");
const FIRST_USER_ADDRESS: TestAddress = TestAddress::new("first-user");
const SECOND_USER_ADDRESS: TestAddress = TestAddress::new("second-user");
const THIRD_USER_ADDRESS: TestAddress = TestAddress::new("third-user");

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.set_current_dir_from_workspace("contracts/feature-tests/use-module");
    blockchain.register_contract(USE_MODULE_PATH_EXPR, use_module::ContractBuilder);
    blockchain
}

fn setup() -> ScenarioWorld {
    let mut world = world();

    world
        .account(OWNER_ADDRESS)
        .nonce(1)
        .esdt_balance(GOV_TOKEN_ID, INITIAL_GOV_TOKEN_BALANCE);
    world
        .account(FIRST_USER_ADDRESS)
        .nonce(1)
        .esdt_balance(GOV_TOKEN_ID, INITIAL_GOV_TOKEN_BALANCE);
    world
        .account(SECOND_USER_ADDRESS)
        .nonce(1)
        .esdt_balance(GOV_TOKEN_ID, INITIAL_GOV_TOKEN_BALANCE);
    world
        .account(THIRD_USER_ADDRESS)
        .nonce(1)
        .esdt_balance(GOV_TOKEN_ID, INITIAL_GOV_TOKEN_BALANCE);

    // init
    let new_address = world
        .tx()
        .from(OWNER_ADDRESS)
        .raw_deploy()
        .code(USE_MODULE_PATH_EXPR)
        .new_address(USE_MODULE_ADDRESS)
        .returns(ReturnsNewBech32Address)
        .whitebox(use_module::contract_obj, |sc| {
            sc.init_governance_module(
                EsdtTokenIdentifier::from(GOV_TOKEN_ID),
                BigUint::from(QUORUM),
                BigUint::from(MIN_BALANCE_PROPOSAL),
                VOTING_DELAY_BLOCKS,
                VOTING_PERIOD_BLOCKS,
                LOCKING_PERIOD_BLOCKS,
            );
        });

    assert_eq!(new_address, USE_MODULE_ADDRESS);

    world.current_block().block_nonce(10);

    world
}

pub fn propose(
    world: &mut ScenarioWorld,
    proposer: &Address,
    gov_token_amount: u64,
    dest_address: &Address,
    endpoint_name: &[u8],
    args: Vec<Vec<u8>>,
) -> usize {
    let mut proposal_id = 0;

    world
        .tx()
        .from(proposer)
        .to(USE_MODULE_ADDRESS)
        .payment(Payment::try_new(GOV_TOKEN_ID, 0, gov_token_amount).unwrap())
        .whitebox(use_module::contract_obj, |sc| {
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
        });

    proposal_id
}

#[test]
fn test_init() {
    setup();
}

#[test]
fn test_change_gov_config() {
    let mut world = setup();

    let mut current_block_nonce = 10;

    let proposal_id = propose(
        &mut world,
        &FIRST_USER_ADDRESS.to_address(),
        500,
        &USE_MODULE_ADDRESS.to_address(),
        b"changeQuorum",
        vec![1_000u64.to_be_bytes().to_vec()],
    );

    assert_eq!(proposal_id, 1);

    // vote too early
    world
        .tx()
        .from(SECOND_USER_ADDRESS)
        .to(USE_MODULE_ADDRESS)
        .payment(Payment::try_new(GOV_TOKEN_ID, 0, 999u64).unwrap())
        .returns(ExpectError(4u64, "Proposal is not active"))
        .whitebox(use_module::contract_obj, |sc| {
            sc.vote(proposal_id, VoteType::UpVote);
        });

    current_block_nonce += VOTING_DELAY_BLOCKS;
    world.current_block().block_nonce(current_block_nonce);

    world
        .tx()
        .from(SECOND_USER_ADDRESS)
        .to(USE_MODULE_ADDRESS)
        .payment(Payment::try_new(GOV_TOKEN_ID, 0, 999u64).unwrap())
        .whitebox(use_module::contract_obj, |sc| {
            sc.vote(proposal_id, VoteType::UpVote);
        });

    // try execute before queue
    world
        .tx()
        .from(FIRST_USER_ADDRESS)
        .to(USE_MODULE_ADDRESS)
        .returns(ExpectError(4u64, "Can only execute queued proposals"))
        .whitebox(use_module::contract_obj, |sc| {
            sc.execute(proposal_id);
        });

    // try queue before voting ends
    world
        .tx()
        .from(FIRST_USER_ADDRESS)
        .to(USE_MODULE_ADDRESS)
        .returns(ExpectError(4u64, "Can only queue succeeded proposals"))
        .whitebox(use_module::contract_obj, |sc| {
            sc.queue(proposal_id);
        });

    current_block_nonce += VOTING_PERIOD_BLOCKS;
    world.current_block().block_nonce(current_block_nonce);

    // try queue not enough votes
    world
        .tx()
        .from(FIRST_USER_ADDRESS)
        .to(USE_MODULE_ADDRESS)
        .returns(ExpectError(4u64, "Can only queue succeeded proposals"))
        .whitebox(use_module::contract_obj, |sc| {
            sc.queue(proposal_id);
        });

    // user 1 vote again
    current_block_nonce = 20;
    world.current_block().block_nonce(current_block_nonce);

    world
        .tx()
        .from(FIRST_USER_ADDRESS)
        .to(USE_MODULE_ADDRESS)
        .payment(Payment::try_new(GOV_TOKEN_ID, 0, 200u64).unwrap())
        .whitebox(use_module::contract_obj, |sc| {
            sc.vote(proposal_id, VoteType::UpVote);
        });

    // owner downvote
    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(USE_MODULE_ADDRESS)
        .payment(Payment::try_new(GOV_TOKEN_ID, 0, 200u64).unwrap())
        .whitebox(use_module::contract_obj, |sc| {
            sc.vote(proposal_id, VoteType::DownVote);
        });

    // try queue too many downvotes
    current_block_nonce = 45;
    world.current_block().block_nonce(current_block_nonce);

    world
        .tx()
        .from(FIRST_USER_ADDRESS)
        .to(USE_MODULE_ADDRESS)
        .returns(ExpectError(4u64, "Can only queue succeeded proposals"))
        .whitebox(use_module::contract_obj, |sc| {
            sc.queue(proposal_id);
        });

    // user 1 vote again
    current_block_nonce = 20;
    world.current_block().block_nonce(current_block_nonce);

    world
        .tx()
        .from(FIRST_USER_ADDRESS)
        .to(USE_MODULE_ADDRESS)
        .payment(Payment::try_new(GOV_TOKEN_ID, 0, 200u64).unwrap())
        .returns(ExpectError(4u64, "Already voted for this proposal"))
        .whitebox(use_module::contract_obj, |sc| {
            sc.vote(proposal_id, VoteType::UpVote);
        });

    // user 3 vote again
    world
        .tx()
        .from(THIRD_USER_ADDRESS)
        .to(USE_MODULE_ADDRESS)
        .payment(Payment::try_new(GOV_TOKEN_ID, 0, 200u64).unwrap())
        .whitebox(use_module::contract_obj, |sc| {
            sc.vote(proposal_id, VoteType::UpVote);
        });

    // queue ok
    current_block_nonce = 45;
    world.current_block().block_nonce(current_block_nonce);

    world
        .tx()
        .from(FIRST_USER_ADDRESS)
        .to(USE_MODULE_ADDRESS)
        .whitebox(use_module::contract_obj, |sc| {
            sc.queue(proposal_id);
        });

    // try execute too early
    world
        .tx()
        .from(FIRST_USER_ADDRESS)
        .to(USE_MODULE_ADDRESS)
        .returns(ExpectError(
            4u64,
            "Proposal is in timelock status. Try again later",
        ))
        .whitebox(use_module::contract_obj, |sc| sc.execute(proposal_id));

    // execute ok
    current_block_nonce += LOCKING_PERIOD_BLOCKS;
    world.current_block().block_nonce(current_block_nonce);

    world
        .tx()
        .from(FIRST_USER_ADDRESS)
        .to(USE_MODULE_ADDRESS)
        .whitebox(use_module::contract_obj, |sc| sc.execute(proposal_id));

    // after execution, quorum changed from 1_500 to the proposed 1_000
    world
        .query()
        .to(USE_MODULE_ADDRESS)
        .whitebox(use_module::contract_obj, |sc| {
            assert_eq!(sc.quorum().get(), managed_biguint!(1_000));
            assert!(sc.proposals().item_is_empty(1));
        });

    world
        .check_account(FIRST_USER_ADDRESS)
        .esdt_balance(GOV_TOKEN_ID, BigUint::from(300u64));
    world
        .check_account(SECOND_USER_ADDRESS)
        .esdt_balance(GOV_TOKEN_ID, BigUint::from(1u64));
    world
        .check_account(THIRD_USER_ADDRESS)
        .esdt_balance(GOV_TOKEN_ID, BigUint::from(800u64));
    world
        .check_account(OWNER_ADDRESS)
        .esdt_balance(GOV_TOKEN_ID, BigUint::from(800u64));
}

#[test]
fn test_down_veto_gov_config() {
    let mut world = setup();

    let mut current_block_nonce = 10;

    let proposal_id = propose(
        &mut world,
        &FIRST_USER_ADDRESS.to_address(),
        500,
        &USE_MODULE_ADDRESS.to_address(),
        b"changeQuorum",
        vec![1_000u64.to_be_bytes().to_vec()],
    );

    assert_eq!(proposal_id, 1);

    current_block_nonce += VOTING_DELAY_BLOCKS;
    world.current_block().block_nonce(current_block_nonce);

    world
        .tx()
        .from(FIRST_USER_ADDRESS)
        .to(USE_MODULE_ADDRESS)
        .payment(Payment::try_new(GOV_TOKEN_ID, 0, 300u64).unwrap())
        .whitebox(use_module::contract_obj, |sc| {
            sc.vote(proposal_id, VoteType::UpVote);
        });

    current_block_nonce = 20;
    world.current_block().block_nonce(current_block_nonce);

    world
        .tx()
        .from(SECOND_USER_ADDRESS)
        .to(USE_MODULE_ADDRESS)
        .payment(Payment::try_new(GOV_TOKEN_ID, 0, 200u64).unwrap())
        .whitebox(use_module::contract_obj, |sc| {
            sc.vote(proposal_id, VoteType::UpVote);
        });

    world
        .tx()
        .from(THIRD_USER_ADDRESS)
        .to(USE_MODULE_ADDRESS)
        .payment(Payment::try_new(GOV_TOKEN_ID, 0, 200u64).unwrap())
        .whitebox(use_module::contract_obj, |sc| {
            sc.vote(proposal_id, VoteType::DownVetoVote)
        });

    // Vote didn't succeed;
    current_block_nonce = 45;
    world.current_block().block_epoch(current_block_nonce);

    world
        .tx()
        .from(FIRST_USER_ADDRESS)
        .to(USE_MODULE_ADDRESS)
        .returns(ExpectError(4u64, "Can only queue succeeded proposals"))
        .whitebox(use_module::contract_obj, |sc| {
            sc.queue(proposal_id);
        });

    world
        .check_account(FIRST_USER_ADDRESS)
        .esdt_balance(GOV_TOKEN_ID, BigUint::from(200u64));

    world
        .check_account(SECOND_USER_ADDRESS)
        .esdt_balance(GOV_TOKEN_ID, BigUint::from(800u64));

    world
        .check_account(THIRD_USER_ADDRESS)
        .esdt_balance(GOV_TOKEN_ID, BigUint::from(800u64));
}

#[test]
fn test_abstain_vote_gov_config() {
    let mut world = setup();

    let mut current_block_nonce = 10;

    let proposal_id = propose(
        &mut world,
        &FIRST_USER_ADDRESS.to_address(),
        500,
        &USE_MODULE_ADDRESS.to_address(),
        b"changeQuorum",
        vec![1_000u64.to_be_bytes().to_vec()],
    );

    assert_eq!(proposal_id, 1);

    current_block_nonce += VOTING_DELAY_BLOCKS;
    world.current_block().block_nonce(current_block_nonce);

    world
        .tx()
        .from(FIRST_USER_ADDRESS)
        .to(USE_MODULE_ADDRESS)
        .payment(Payment::try_new(GOV_TOKEN_ID, 0, 500u64).unwrap())
        .whitebox(use_module::contract_obj, |sc| {
            sc.vote(proposal_id, VoteType::UpVote);
        });

    current_block_nonce = 20;
    world.current_block().block_nonce(current_block_nonce);

    world
        .tx()
        .from(SECOND_USER_ADDRESS)
        .to(USE_MODULE_ADDRESS)
        .payment(Payment::try_new(GOV_TOKEN_ID, 0, 400u64).unwrap())
        .whitebox(use_module::contract_obj, |sc| {
            sc.vote(proposal_id, VoteType::DownVote);
        });

    world
        .tx()
        .from(THIRD_USER_ADDRESS)
        .to(USE_MODULE_ADDRESS)
        .payment(Payment::try_new(GOV_TOKEN_ID, 0, 600u64).unwrap())
        .whitebox(use_module::contract_obj, |sc| {
            sc.vote(proposal_id, VoteType::AbstainVote);
        });

    // Vote didn't succeed;
    current_block_nonce = 45;
    world.current_block().block_nonce(current_block_nonce);

    world
        .tx()
        .from(FIRST_USER_ADDRESS)
        .to(USE_MODULE_ADDRESS)
        .whitebox(use_module::contract_obj, |sc| {
            sc.queue(proposal_id);
        });

    // execute ok
    current_block_nonce += LOCKING_PERIOD_BLOCKS;
    world.current_block().block_nonce(current_block_nonce);

    world
        .tx()
        .from(FIRST_USER_ADDRESS)
        .to(USE_MODULE_ADDRESS)
        .whitebox(use_module::contract_obj, |sc| {
            sc.execute(proposal_id);
        });

    // after execution, quorum changed from 1_500 to the proposed 1_000
    world
        .query()
        .to(USE_MODULE_ADDRESS)
        .whitebox(use_module::contract_obj, |sc| {
            assert_eq!(sc.quorum().get(), managed_biguint!(1_000));
            assert!(sc.proposals().item_is_empty(1));
        });

    world
        .check_account(FIRST_USER_ADDRESS)
        .esdt_balance(GOV_TOKEN_ID, BigUint::zero());
    world
        .check_account(SECOND_USER_ADDRESS)
        .esdt_balance(GOV_TOKEN_ID, BigUint::from(600u64));
    world
        .check_account(THIRD_USER_ADDRESS)
        .esdt_balance(GOV_TOKEN_ID, BigUint::from(400u64));
}

#[test]
fn test_gov_cancel_defeated_proposal() {
    let mut world = setup();

    let mut current_block_nonce = 10;

    let proposal_id = propose(
        &mut world,
        &FIRST_USER_ADDRESS.to_address(),
        500,
        &USE_MODULE_ADDRESS.to_address(),
        b"changeQuorum",
        vec![1_000u64.to_be_bytes().to_vec()],
    );

    assert_eq!(proposal_id, 1);

    current_block_nonce += VOTING_DELAY_BLOCKS;
    world.current_block().block_nonce(current_block_nonce);

    world
        .tx()
        .from(SECOND_USER_ADDRESS)
        .to(USE_MODULE_ADDRESS)
        .payment(Payment::try_new(GOV_TOKEN_ID, 0, 999u64).unwrap())
        .whitebox(use_module::contract_obj, |sc| {
            sc.vote(proposal_id, VoteType::DownVote)
        });

    // try cancel too early
    world
        .tx()
        .from(SECOND_USER_ADDRESS)
        .to(USE_MODULE_ADDRESS)
        .returns(ExpectError(4u64, "Action may not be cancelled"))
        .whitebox(use_module::contract_obj, |sc| {
            sc.cancel(proposal_id);
        });

    current_block_nonce += VOTING_PERIOD_BLOCKS;
    world.current_block().block_nonce(current_block_nonce);

    world
        .tx()
        .from(SECOND_USER_ADDRESS)
        .to(USE_MODULE_ADDRESS)
        .whitebox(use_module::contract_obj, |sc| sc.cancel(proposal_id));
}
