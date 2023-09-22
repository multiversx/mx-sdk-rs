use multiversx_sc::types::{Address, ManagedVec, MultiValueEncoded};
use multiversx_sc_modules::governance::{
    governance_configurable::GovernanceConfigurablePropertiesModule, governance_proposal::VoteType,
    GovernanceModule,
};
use multiversx_sc_scenario::{
    managed_address, managed_biguint, managed_buffer, managed_token_id,
    scenario_model::{
        Account, AddressValue, CheckAccount, CheckStateStep, ScCallStep, ScDeployStep, SetStateStep,
    },
    ScenarioWorld, WhiteboxContract,
};

const GOV_TOKEN_ID_EXPR: &str = "str:GOV-123456";
const GOV_TOKEN_ID: &[u8] = b"GOV-123456";
const QUORUM: u64 = 1_500;
const MIN_BALANCE_PROPOSAL: u64 = 500;
const VOTING_DELAY_BLOCKS: u64 = 10;
const VOTING_PERIOD_BLOCKS: u64 = 20;
const LOCKING_PERIOD_BLOCKS: u64 = 30;

const INITIAL_GOV_TOKEN_BALANCE: u64 = 1_000;
const GAS_LIMIT: u64 = 1_000_000;

const USE_MODULE_ADDRESS_EXPR: &str = "sc:use-module";
const USE_MODULE_PATH_EXPR: &str = "file:output/use-module.wasm";

const OWNER_ADDRESS_EXPR: &str = "address:owner";
const FIRST_USER_ADDRESS_EXPR: &str = "address:first-user";
const SECOND_USER_ADDRESS_EXPR: &str = "address:second-user";
const THIRD_USER_ADDRESS_EXPR: &str = "address:third-user";

pub struct Payment {
    pub token: Vec<u8>,
    pub nonce: u64,
    pub amount: u64,
}

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/features-tests/use-module");

    blockchain.register_contract(USE_MODULE_PATH_EXPR, use_module::ContractBuilder);
    blockchain
}

fn setup() -> ScenarioWorld {
    let mut world = world();

    world.set_state_step(
        SetStateStep::new()
            .put_account(
                OWNER_ADDRESS_EXPR,
                Account::new()
                    .nonce(1)
                    .esdt_balance(GOV_TOKEN_ID_EXPR, INITIAL_GOV_TOKEN_BALANCE),
            )
            .new_address(OWNER_ADDRESS_EXPR, 1, USE_MODULE_ADDRESS_EXPR)
            .put_account(
                FIRST_USER_ADDRESS_EXPR,
                Account::new()
                    .nonce(1)
                    .esdt_balance(GOV_TOKEN_ID_EXPR, INITIAL_GOV_TOKEN_BALANCE),
            )
            .put_account(
                SECOND_USER_ADDRESS_EXPR,
                Account::new()
                    .nonce(1)
                    .esdt_balance(GOV_TOKEN_ID_EXPR, INITIAL_GOV_TOKEN_BALANCE),
            )
            .put_account(
                THIRD_USER_ADDRESS_EXPR,
                Account::new()
                    .nonce(1)
                    .esdt_balance(GOV_TOKEN_ID_EXPR, INITIAL_GOV_TOKEN_BALANCE),
            ),
    );

    // init
    let use_module_whitebox =
        WhiteboxContract::new(USE_MODULE_ADDRESS_EXPR, use_module::contract_obj);
    let use_module_code = world.code_expression(USE_MODULE_PATH_EXPR);

    world.whitebox_deploy(
        &use_module_whitebox,
        ScDeployStep::new()
            .from(OWNER_ADDRESS_EXPR)
            .code(use_module_code),
        |sc| {
            sc.init_governance_module(
                managed_token_id!(GOV_TOKEN_ID),
                managed_biguint!(QUORUM),
                managed_biguint!(MIN_BALANCE_PROPOSAL),
                VOTING_DELAY_BLOCKS,
                VOTING_PERIOD_BLOCKS,
                LOCKING_PERIOD_BLOCKS,
            );
        },
    );

    world.set_state_step(SetStateStep::new().block_nonce(10));

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
    let use_module_whitebox =
        WhiteboxContract::new(USE_MODULE_ADDRESS_EXPR, use_module::contract_obj);

    let mut proposal_id = 0;

    world.whitebox_call(
        &use_module_whitebox,
        ScCallStep::new().from(proposer).esdt_transfer(
            GOV_TOKEN_ID,
            0,
            gov_token_amount,
        ),
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

    proposal_id
}

#[test]
fn test_init() {
    setup();
}

#[test]
fn test_change_gov_config() {
    let mut world = setup();
    let use_module_whitebox =
        WhiteboxContract::new(USE_MODULE_ADDRESS_EXPR, use_module::contract_obj);

    let mut current_block_nonce = 10;

    let proposal_id = propose(
        &mut world,
        &address_expr_to_address(FIRST_USER_ADDRESS_EXPR),
        500,
        &address_expr_to_address(USE_MODULE_ADDRESS_EXPR),
        b"changeQuorum",
        vec![1_000u64.to_be_bytes().to_vec()],
    );

    assert_eq!(proposal_id, 1);

    // vote too early
    world.whitebox_call_check(
        &use_module_whitebox,
        ScCallStep::new()
            .from(SECOND_USER_ADDRESS_EXPR)
            .esdt_transfer(GOV_TOKEN_ID, 0, "999")
            .no_expect(),
        |sc| {
            sc.vote(proposal_id, VoteType::UpVote);
        },
        |r| {
            r.assert_user_error("Proposal is not active");
        },
    );

    current_block_nonce += VOTING_DELAY_BLOCKS;
    world.set_state_step(SetStateStep::new().block_nonce(current_block_nonce));

    world.whitebox_call(
        &use_module_whitebox,
        ScCallStep::new()
            .from(SECOND_USER_ADDRESS_EXPR)
            .esdt_transfer(GOV_TOKEN_ID, 0, "999"),
        |sc| {
            sc.vote(proposal_id, VoteType::UpVote);
        },
    );

    // try execute before queue
    world.whitebox_call_check(
        &use_module_whitebox,
        ScCallStep::new().from(FIRST_USER_ADDRESS_EXPR).no_expect(),
        |sc| {
            sc.execute(proposal_id);
        },
        |r| {
            r.assert_user_error("Can only execute queued proposals");
        },
    );

    // try queue before voting ends
    world.whitebox_call_check(
        &use_module_whitebox,
        ScCallStep::new().from(FIRST_USER_ADDRESS_EXPR).no_expect(),
        |sc| {
            sc.queue(proposal_id);
        },
        |r| {
            r.assert_user_error("Can only queue succeeded proposals");
        },
    );

    current_block_nonce += VOTING_PERIOD_BLOCKS;
    world.set_state_step(SetStateStep::new().block_nonce(current_block_nonce));

    // try queue not enough votes
    world.whitebox_call_check(
        &use_module_whitebox,
        ScCallStep::new().from(FIRST_USER_ADDRESS_EXPR).no_expect(),
        |sc| {
            sc.queue(proposal_id);
        },
        |r| {
            r.assert_user_error("Can only queue succeeded proposals");
        },
    );

    // user 1 vote again
    current_block_nonce = 20;
    world.set_state_step(SetStateStep::new().block_nonce(current_block_nonce));
    world.whitebox_call(
        &use_module_whitebox,
        ScCallStep::new()
            .from(FIRST_USER_ADDRESS_EXPR)
            .esdt_transfer(GOV_TOKEN_ID, 0, "200"),
        |sc| {
            sc.vote(proposal_id, VoteType::UpVote);
        },
    );

    // owner downvote
    world.whitebox_call(
        &use_module_whitebox,
        ScCallStep::new().from(OWNER_ADDRESS_EXPR).esdt_transfer(
            GOV_TOKEN_ID,
            0,
            "200",
        ),
        |sc| {
            sc.vote(proposal_id, VoteType::DownVote);
        },
    );

    // try queue too many downvotes
    current_block_nonce = 45;
    world.set_state_step(SetStateStep::new().block_nonce(current_block_nonce));
    world.whitebox_call_check(
        &use_module_whitebox,
        ScCallStep::new().from(FIRST_USER_ADDRESS_EXPR).no_expect(),
        |sc| {
            sc.queue(proposal_id);
        },
        |r| {
            r.assert_user_error("Can only queue succeeded proposals");
        },
    );

    // user 1 vote again
    current_block_nonce = 20;
    world.set_state_step(SetStateStep::new().block_nonce(current_block_nonce));
    world.whitebox_call_check(
        &use_module_whitebox,
        ScCallStep::new()
            .from(FIRST_USER_ADDRESS_EXPR)
            .esdt_transfer(GOV_TOKEN_ID, 0, "200")
            .no_expect(),
        |sc| {
            sc.vote(proposal_id, VoteType::UpVote);
        },
        |r| {
            r.assert_user_error("Already voted for this proposal");
        },
    );

    // user 3 vote again
    world.whitebox_call(
        &use_module_whitebox,
        ScCallStep::new()
            .from(THIRD_USER_ADDRESS_EXPR)
            .esdt_transfer(GOV_TOKEN_ID, 0, "200"),
        |sc| {
            sc.vote(proposal_id, VoteType::UpVote);
        },
    );

    // queue ok
    current_block_nonce = 45;
    world.set_state_step(SetStateStep::new().block_nonce(current_block_nonce));
    world.whitebox_call(
        &use_module_whitebox,
        ScCallStep::new().from(FIRST_USER_ADDRESS_EXPR).no_expect(),
        |sc| {
            sc.queue(proposal_id);
        },
    );

    // try execute too early
    world.whitebox_call_check(
        &use_module_whitebox,
        ScCallStep::new().from(FIRST_USER_ADDRESS_EXPR).no_expect(),
        |sc| {
            sc.execute(proposal_id);
        },
        |r| {
            r.assert_user_error("Proposal is in timelock status. Try again later");
        },
    );

    // execute ok
    current_block_nonce += LOCKING_PERIOD_BLOCKS;
    world.set_state_step(SetStateStep::new().block_nonce(current_block_nonce));
    world.whitebox_call(
        &use_module_whitebox,
        ScCallStep::new().from(FIRST_USER_ADDRESS_EXPR).no_expect(),
        |sc| {
            sc.execute(proposal_id);
        },
    );

    // after execution, quorum changed from 1_500 to the proposed 1_000
    world.whitebox_query(&use_module_whitebox, |sc| {
        assert_eq!(sc.quorum().get(), managed_biguint!(1_000));
        assert!(sc.proposals().item_is_empty(1));
    });

    world.check_state_step(CheckStateStep::new().put_account(
        FIRST_USER_ADDRESS_EXPR,
        CheckAccount::new().esdt_balance(GOV_TOKEN_ID_EXPR, "300"),
    ));
    world.check_state_step(CheckStateStep::new().put_account(
        SECOND_USER_ADDRESS_EXPR,
        CheckAccount::new().esdt_balance(GOV_TOKEN_ID_EXPR, "1"),
    ));
    world.check_state_step(CheckStateStep::new().put_account(
        THIRD_USER_ADDRESS_EXPR,
        CheckAccount::new().esdt_balance(GOV_TOKEN_ID_EXPR, "800"),
    ));
    world.check_state_step(CheckStateStep::new().put_account(
        OWNER_ADDRESS_EXPR,
        CheckAccount::new().esdt_balance(GOV_TOKEN_ID_EXPR, "800"),
    ));
}

#[test]
fn test_down_veto_gov_config() {
    let mut world = setup();
    let use_module_whitebox =
        WhiteboxContract::new(USE_MODULE_ADDRESS_EXPR, use_module::contract_obj);

    let mut current_block_nonce = 10;

    let proposal_id = propose(
        &mut world,
        &address_expr_to_address(FIRST_USER_ADDRESS_EXPR),
        500,
        &address_expr_to_address(USE_MODULE_ADDRESS_EXPR),
        b"changeQuorum",
        vec![1_000u64.to_be_bytes().to_vec()],
    );

    assert_eq!(proposal_id, 1);

    current_block_nonce += VOTING_DELAY_BLOCKS;
    world.set_state_step(SetStateStep::new().block_nonce(current_block_nonce));

    world.whitebox_call(
        &use_module_whitebox,
        ScCallStep::new()
            .from(FIRST_USER_ADDRESS_EXPR)
            .esdt_transfer(GOV_TOKEN_ID, 0, "300"),
        |sc| {
            sc.vote(proposal_id, VoteType::UpVote);
        },
    );

    current_block_nonce = 20;
    world.set_state_step(SetStateStep::new().block_nonce(current_block_nonce));
    world.whitebox_call(
        &use_module_whitebox,
        ScCallStep::new()
            .from(SECOND_USER_ADDRESS_EXPR)
            .esdt_transfer(GOV_TOKEN_ID, 0, "200"),
        |sc| {
            sc.vote(proposal_id, VoteType::UpVote);
        },
    );

    world.whitebox_call(
        &use_module_whitebox,
        ScCallStep::new()
            .from(THIRD_USER_ADDRESS_EXPR)
            .esdt_transfer(GOV_TOKEN_ID, 0, "200"),
        |sc| {
            sc.vote(proposal_id, VoteType::DownVetoVote);
        },
    );

    // Vote didn't succeed;
    current_block_nonce = 45;
    world.set_state_step(SetStateStep::new().block_nonce(current_block_nonce));
    world.whitebox_call_check(
        &use_module_whitebox,
        ScCallStep::new().from(FIRST_USER_ADDRESS_EXPR).no_expect(),
        |sc| {
            sc.queue(proposal_id);
        },
        |r| {
            r.assert_user_error("Can only queue succeeded proposals");
        },
    );

    world.check_state_step(CheckStateStep::new().put_account(
        FIRST_USER_ADDRESS_EXPR,
        CheckAccount::new().esdt_balance(GOV_TOKEN_ID_EXPR, "200"),
    ));
    world.check_state_step(CheckStateStep::new().put_account(
        SECOND_USER_ADDRESS_EXPR,
        CheckAccount::new().esdt_balance(GOV_TOKEN_ID_EXPR, "800"),
    ));
    world.check_state_step(CheckStateStep::new().put_account(
        THIRD_USER_ADDRESS_EXPR,
        CheckAccount::new().esdt_balance(GOV_TOKEN_ID_EXPR, "800"),
    ));
}

#[test]
fn test_abstain_vote_gov_config() {
    let mut world = setup();
    let use_module_whitebox =
        WhiteboxContract::new(USE_MODULE_ADDRESS_EXPR, use_module::contract_obj);

    let mut current_block_nonce = 10;

    let proposal_id = propose(
        &mut world,
        &address_expr_to_address(FIRST_USER_ADDRESS_EXPR),
        500,
        &address_expr_to_address(USE_MODULE_ADDRESS_EXPR),
        b"changeQuorum",
        vec![1_000u64.to_be_bytes().to_vec()],
    );

    assert_eq!(proposal_id, 1);

    current_block_nonce += VOTING_DELAY_BLOCKS;
    world.set_state_step(SetStateStep::new().block_nonce(current_block_nonce));

    world.whitebox_call(
        &use_module_whitebox,
        ScCallStep::new()
            .from(FIRST_USER_ADDRESS_EXPR)
            .esdt_transfer(GOV_TOKEN_ID, 0, "500"),
        |sc| {
            sc.vote(proposal_id, VoteType::UpVote);
        },
    );

    current_block_nonce = 20;
    world.set_state_step(SetStateStep::new().block_nonce(current_block_nonce));
    world.whitebox_call(
        &use_module_whitebox,
        ScCallStep::new()
            .from(SECOND_USER_ADDRESS_EXPR)
            .esdt_transfer(GOV_TOKEN_ID, 0, "400"),
        |sc| {
            sc.vote(proposal_id, VoteType::DownVote);
        },
    );

    world.whitebox_call(
        &use_module_whitebox,
        ScCallStep::new()
            .from(THIRD_USER_ADDRESS_EXPR)
            .esdt_transfer(GOV_TOKEN_ID, 0, "600"),
        |sc| {
            sc.vote(proposal_id, VoteType::AbstainVote);
        },
    );

    // Vote didn't succeed;
    current_block_nonce = 45;
    world.set_state_step(SetStateStep::new().block_nonce(current_block_nonce));
    world.whitebox_call(
        &use_module_whitebox,
        ScCallStep::new().from(FIRST_USER_ADDRESS_EXPR).no_expect(),
        |sc| {
            sc.queue(proposal_id);
        },
    );

    // execute ok
    current_block_nonce += LOCKING_PERIOD_BLOCKS;
    world.set_state_step(SetStateStep::new().block_nonce(current_block_nonce));
    world.whitebox_call(
        &use_module_whitebox,
        ScCallStep::new().from(FIRST_USER_ADDRESS_EXPR).no_expect(),
        |sc| {
            sc.execute(proposal_id);
        },
    );

    // after execution, quorum changed from 1_500 to the proposed 1_000
    world.whitebox_query(&use_module_whitebox, |sc| {
        assert_eq!(sc.quorum().get(), managed_biguint!(1_000));
        assert!(sc.proposals().item_is_empty(1));
    });

    world.check_state_step(CheckStateStep::new().put_account(
        FIRST_USER_ADDRESS_EXPR,
        CheckAccount::new().esdt_balance(GOV_TOKEN_ID_EXPR, "0"),
    ));
    world.check_state_step(CheckStateStep::new().put_account(
        SECOND_USER_ADDRESS_EXPR,
        CheckAccount::new().esdt_balance(GOV_TOKEN_ID_EXPR, "600"),
    ));
    world.check_state_step(CheckStateStep::new().put_account(
        THIRD_USER_ADDRESS_EXPR,
        CheckAccount::new().esdt_balance(GOV_TOKEN_ID_EXPR, "400"),
    ));
}

#[test]
fn test_gov_cancel_defeated_proposal() {
    let mut world = setup();
    let use_module_whitebox =
        WhiteboxContract::new(USE_MODULE_ADDRESS_EXPR, use_module::contract_obj);

    let mut current_block_nonce = 10;

    let proposal_id = propose(
        &mut world,
        &address_expr_to_address(FIRST_USER_ADDRESS_EXPR),
        500,
        &address_expr_to_address(USE_MODULE_ADDRESS_EXPR),
        b"changeQuorum",
        vec![1_000u64.to_be_bytes().to_vec()],
    );

    assert_eq!(proposal_id, 1);

    current_block_nonce += VOTING_DELAY_BLOCKS;
    world.set_state_step(SetStateStep::new().block_nonce(current_block_nonce));

    world.whitebox_call(
        &use_module_whitebox,
        ScCallStep::new()
            .from(SECOND_USER_ADDRESS_EXPR)
            .esdt_transfer(GOV_TOKEN_ID, 0, "999"),
        |sc| {
            sc.vote(proposal_id, VoteType::DownVote);
        },
    );

    // try cancel too early
    world.whitebox_call_check(
        &use_module_whitebox,
        ScCallStep::new().from(SECOND_USER_ADDRESS_EXPR).no_expect(),
        |sc| {
            sc.cancel(proposal_id);
        },
        |r| {
            r.assert_user_error("Action may not be cancelled");
        },
    );

    current_block_nonce += VOTING_PERIOD_BLOCKS;
    world.set_state_step(SetStateStep::new().block_nonce(current_block_nonce));

    world.whitebox_call(
        &use_module_whitebox,
        ScCallStep::new().from(SECOND_USER_ADDRESS_EXPR).no_expect(),
        |sc| {
            sc.cancel(proposal_id);
        },
    );
}

fn address_expr_to_address(address_expr: &str) -> Address {
    AddressValue::from(address_expr).to_address()
}
