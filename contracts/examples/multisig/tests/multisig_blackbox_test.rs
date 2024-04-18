use multiversx_sc::codec::top_encode_to_vec_u8_or_panic;
use multiversx_sc_scenario::imports::*;

use adder::{adder_proxy, ProxyTrait as _};
use multisig::{multisig_perform::ProxyTrait as _, multisig_proxy};
use num_bigint::BigUint;

const ADDER_ADDRESS_EXPR: &str = "sc:adder";
const SC_ADDER_EXPR: ScExpr = ScExpr("adder");
const ADDER_OWNER_ADDRESS_EXPR: AddressExpr = AddressExpr("adder-owner");
const ADDER_PATH_EXPR: &str = "mxsc:test-contracts/adder.mxsc.json";
const ADDER_CODE_EXPR: MxscExpr = MxscExpr("test-contracts/adder.mxsc.json");
const BOARD_MEMBER_ADDRESS_EXPR: &str = "address:board-member";
const BOARD_MEMBER_ADDRESS_EXPR_REPL: AddressExpr = AddressExpr("board-member");
const MULTISIG_ADDRESS_EXPR: &str = "sc:multisig";
const SC_MULTISIG_EXPR: ScExpr = ScExpr("multisig");
const MULTISIG_PATH_EXPR: &str = "mxsc:output/multisig.mxsc.json";
const MULTISIG_CODE_EXPR: MxscExpr = MxscExpr("output/multisig.mxsc.json");
const OWNER_ADDRESS_EXPR_REPL: AddressExpr = AddressExpr("owner");
const PROPOSER_ADDRESS_EXPR: AddressExpr = AddressExpr("proposer");
const PROPOSER_BALANCE_EXPR: &str = "100,000,000";
const QUORUM_SIZE: usize = 1;

type MultisigContract = ContractInfo<multisig::Proxy<StaticApi>>;
type AdderContract = ContractInfo<adder::Proxy<StaticApi>>;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/examples/multisig");

    blockchain.register_contract(MULTISIG_PATH_EXPR, multisig::ContractBuilder);
    blockchain.register_contract(ADDER_PATH_EXPR, adder::ContractBuilder);
    blockchain
}

struct MultisigTestState {
    world: ScenarioWorld,
    multisig_contract: MultisigContract,
    adder_contract: AdderContract,
    adder_address: Address,
}

impl MultisigTestState {
    fn new() -> Self {
        let mut world = world();

        world
            .account(OWNER_ADDRESS_EXPR_REPL)
            .nonce(1)
            .account(PROPOSER_ADDRESS_EXPR)
            .nonce(1)
            .balance(PROPOSER_BALANCE_EXPR)
            .account(BOARD_MEMBER_ADDRESS_EXPR_REPL)
            .nonce(1)
            .account(ADDER_OWNER_ADDRESS_EXPR)
            .nonce(1);

        world.set_state_step(SetStateStep::new().new_address(
            OWNER_ADDRESS_EXPR_REPL.eval_to_expr().as_str(),
            1,
            SC_MULTISIG_EXPR.eval_to_expr().as_str(),
        ));

        world.set_state_step(SetStateStep::new().new_address(
            ADDER_OWNER_ADDRESS_EXPR.eval_to_expr().as_str(),
            1,
            SC_ADDER_EXPR.eval_to_expr().as_str(),
        ));

        let multisig_contract = MultisigContract::new(SC_MULTISIG_EXPR.eval_to_expr().as_str());
        let adder_contract = AdderContract::new(SC_ADDER_EXPR.eval_to_expr().as_str());
        let adder_address = AddressValue::from(SC_ADDER_EXPR.eval_to_expr().as_str()).to_address();

        Self {
            world,
            multisig_contract,
            adder_contract,
            adder_address,
        }
    }

    fn deploy_multisig_contract(&mut self) -> &mut Self {
        let board_members =
            MultiValueVec::from(vec![BOARD_MEMBER_ADDRESS_EXPR_REPL.eval_to_array()]);

        self.world
            .tx()
            .from(OWNER_ADDRESS_EXPR_REPL)
            .typed(multisig_proxy::MultisigProxy)
            .init(QUORUM_SIZE, board_members)
            .code(MULTISIG_CODE_EXPR)
            .run();

        let action_id: usize = self
            .world
            .tx()
            .from(BOARD_MEMBER_ADDRESS_EXPR_REPL)
            .to(SC_MULTISIG_EXPR)
            .typed(multisig_proxy::MultisigProxy)
            .propose_add_proposer(PROPOSER_ADDRESS_EXPR.eval_to_array())
            .returns(ReturnsResult)
            .run();

        self.sign(action_id);
        self.perform(action_id);

        self.expect_user_role(PROPOSER_ADDRESS_EXPR, multisig_proxy::UserRole::Proposer);

        self
    }

    fn deploy_adder_contract(&mut self) {
        self.world
            .tx()
            .from(ADDER_OWNER_ADDRESS_EXPR)
            .typed(adder_proxy::AdderProxy)
            .init(5u64)
            .code(ADDER_CODE_EXPR)
            .run();
    }

    fn propose_add_board_member(&mut self, board_member_address: AddressExpr) -> usize {
        self.world
            .tx()
            .from(PROPOSER_ADDRESS_EXPR)
            .to(SC_MULTISIG_EXPR)
            .typed(multisig_proxy::MultisigProxy)
            .propose_add_board_member(board_member_address.eval_to_array())
            .returns(ReturnsResult)
            .run()
    }

    fn propose_add_proposer(&mut self, proposer_address: AddressExpr) -> usize {
        self.world
            .tx()
            .from(PROPOSER_ADDRESS_EXPR)
            .to(SC_MULTISIG_EXPR)
            .typed(multisig_proxy::MultisigProxy)
            .propose_add_proposer(proposer_address.eval_to_array())
            .returns(ReturnsResult)
            .run()
    }

    fn propose_change_quorum(&mut self, new_quorum: usize) -> usize {
        self.world
            .tx()
            .from(PROPOSER_ADDRESS_EXPR)
            .to(SC_MULTISIG_EXPR)
            .typed(multisig_proxy::MultisigProxy)
            .propose_change_quorum(new_quorum)
            .returns(ReturnsResult)
            .run()
    }

    fn propose_transfer_execute(
        &mut self,
        to: Address,
        egld_amount: u64,
        contract_call: FunctionCall<StaticApi>,
    ) -> usize {
        self.world
            .tx()
            .from(PROPOSER_ADDRESS_EXPR)
            .to(SC_MULTISIG_EXPR)
            .typed(multisig_proxy::MultisigProxy)
            .propose_transfer_execute(to, egld_amount, contract_call)
            .returns(ReturnsResult)
            .run()
    }

    fn propose_async_call(
        &mut self,
        to: Address,
        egld_amount: u64,
        contract_call: FunctionCall<StaticApi>,
    ) -> usize {
        self.world
            .tx()
            .from(PROPOSER_ADDRESS_EXPR)
            .to(SC_MULTISIG_EXPR)
            .typed(multisig_proxy::MultisigProxy)
            .propose_async_call(to, egld_amount, contract_call)
            .returns(ReturnsResult)
            .run()
    }

    fn propose_remove_user(&mut self, user_address: AddressExpr) -> usize {
        self.world
            .tx()
            .from(PROPOSER_ADDRESS_EXPR)
            .to(SC_MULTISIG_EXPR)
            .typed(multisig_proxy::MultisigProxy)
            .propose_remove_user(user_address.eval_to_array())
            .returns(ReturnsResult)
            .run()
    }

    fn propose_sc_deploy_from_source(
        &mut self,
        amount: u64,
        source: Address,
        code_metadata: CodeMetadata,
        arguments: MultiValueVec<Vec<u8>>,
    ) -> usize {
        self.world
            .tx()
            .from(PROPOSER_ADDRESS_EXPR)
            .to(SC_MULTISIG_EXPR)
            .typed(multisig_proxy::MultisigProxy)
            .propose_sc_deploy_from_source(amount, source, code_metadata, arguments)
            .returns(ReturnsResult)
            .run()
    }

    fn propose_sc_upgrade_from_source(
        &mut self,
        sc_address: Address,
        amount: u64,
        source: Address,
        code_metadata: CodeMetadata,
        arguments: MultiValueVec<Vec<u8>>,
    ) -> usize {
        self.world
            .tx()
            .from(PROPOSER_ADDRESS_EXPR)
            .to(SC_MULTISIG_EXPR)
            .typed(multisig_proxy::MultisigProxy)
            .propose_sc_upgrade_from_source(sc_address, amount, source, code_metadata, arguments)
            .returns(ReturnsResult)
            .run()
    }

    fn perform(&mut self, action_id: usize) {
        self.world
            .tx()
            .from(BOARD_MEMBER_ADDRESS_EXPR_REPL)
            .to(SC_MULTISIG_EXPR)
            .typed(multisig_proxy::MultisigProxy)
            .perform_action_endpoint(action_id)
            .run();
    }

    fn perform_and_expect_err(&mut self, action_id: usize, err_message: &str) {
        self.world
            .tx()
            .from(BOARD_MEMBER_ADDRESS_EXPR_REPL)
            .to(SC_MULTISIG_EXPR)
            .typed(multisig_proxy::MultisigProxy)
            .perform_action_endpoint(action_id)
            .with_result(ExpectError(4, err_message))
            .run();
    }

    fn sign(&mut self, action_id: usize) {
        self.world
            .tx()
            .from(BOARD_MEMBER_ADDRESS_EXPR_REPL)
            .to(SC_MULTISIG_EXPR)
            .typed(multisig_proxy::MultisigProxy)
            .sign(action_id)
            .run();
    }

    fn expect_user_role(
        &mut self,
        user: AddressExpr,
        expected_user_role: multisig_proxy::UserRole,
    ) {
        self.world
            .query()
            .to(SC_MULTISIG_EXPR)
            .typed(multisig_proxy::MultisigProxy)
            .user_role(user.eval_to_array())
            .returns(ExpectValue(expected_user_role))
            .run();
    }
}

#[test]
fn test_add_board_member() {
    let mut state = MultisigTestState::new();
    state.deploy_multisig_contract();

    let new_board_member_expr: AddressExpr = AddressExpr("new-board-member");

    state.world.account(new_board_member_expr).nonce(1);

    state.expect_user_role(new_board_member_expr, multisig_proxy::UserRole::None);

    let action_id = state.propose_add_board_member(new_board_member_expr);
    state.sign(action_id);
    state.perform(action_id);

    let expected_value = MultiValueVec::from(vec![
        BOARD_MEMBER_ADDRESS_EXPR_REPL.eval_to_array(),
        new_board_member_expr.eval_to_array(),
    ]);

    state.expect_user_role(new_board_member_expr, multisig_proxy::UserRole::BoardMember);
    state
        .world
        .query()
        .to(SC_MULTISIG_EXPR)
        .typed(multisig_proxy::MultisigProxy)
        .get_all_board_members()
        .returns(ExpectValue(expected_value))
        .run()
}

#[test]
fn test_add_proposer() {
    let mut state = MultisigTestState::new();
    state.deploy_multisig_contract();

    let new_proposer_address_expr: AddressExpr = AddressExpr("new-proposer");

    state.world.account(new_proposer_address_expr).nonce(1);

    state.expect_user_role(new_proposer_address_expr, multisig_proxy::UserRole::None);

    let action_id = state.propose_add_proposer(new_proposer_address_expr);
    state.sign(action_id);
    state.perform(action_id);

    state.expect_user_role(
        new_proposer_address_expr,
        multisig_proxy::UserRole::Proposer,
    );

    let expected_value = MultiValueVec::from(vec![
        PROPOSER_ADDRESS_EXPR.eval_to_array(),
        new_proposer_address_expr.eval_to_array(),
    ]);
    state
        .world
        .query()
        .to(SC_MULTISIG_EXPR)
        .typed(multisig_proxy::MultisigProxy)
        .get_all_proposers()
        .returns(ExpectValue(expected_value))
        .run();
}

#[test]
fn test_remove_proposer() {
    let mut state = MultisigTestState::new();
    state.deploy_multisig_contract();

    state.expect_user_role(PROPOSER_ADDRESS_EXPR, multisig_proxy::UserRole::Proposer);

    let action_id = state.propose_remove_user(PROPOSER_ADDRESS_EXPR);
    state.sign(action_id);
    state.perform(action_id);

    state.expect_user_role(PROPOSER_ADDRESS_EXPR, multisig_proxy::UserRole::None);
    state
        .world
        .query()
        .to(SC_MULTISIG_EXPR)
        .typed(multisig_proxy::MultisigProxy)
        .get_all_proposers()
        .returns(ExpectValue(MultiValueVec::<Address>::new()))
        .run();
}

#[test]
fn test_try_remove_all_board_members() {
    let mut state = MultisigTestState::new();
    state.deploy_multisig_contract();

    let action_id = state.propose_remove_user(BOARD_MEMBER_ADDRESS_EXPR_REPL);
    state.sign(action_id);
    state.perform_and_expect_err(action_id, "quorum cannot exceed board size")
}

#[test]
fn test_change_quorum() {
    let mut state = MultisigTestState::new();
    state.deploy_multisig_contract();

    let new_quorum = 2;
    // try change quorum > board size
    let action_id = state.propose_change_quorum(new_quorum);
    state.sign(action_id);
    state.perform_and_expect_err(action_id, "quorum cannot exceed board size");

    // try discard before unsigning
    state
        .world
        .tx()
        .from(BOARD_MEMBER_ADDRESS_EXPR_REPL)
        .to(SC_MULTISIG_EXPR)
        .typed(multisig_proxy::MultisigProxy)
        .discard_action(action_id)
        .with_result(ExpectError(
            4,
            "cannot discard action with valid signatures",
        ))
        .run();

    // unsign and discard action
    state
        .world
        .tx()
        .from(BOARD_MEMBER_ADDRESS_EXPR_REPL)
        .to(SC_MULTISIG_EXPR)
        .typed(multisig_proxy::MultisigProxy)
        .unsign(action_id)
        .run();

    state
        .world
        .tx()
        .from(BOARD_MEMBER_ADDRESS_EXPR_REPL)
        .to(SC_MULTISIG_EXPR)
        .typed(multisig_proxy::MultisigProxy)
        .discard_action(action_id)
        .run();

    // try sign discarded action
    state
        .world
        .tx()
        .from(BOARD_MEMBER_ADDRESS_EXPR_REPL)
        .to(SC_MULTISIG_EXPR)
        .typed(multisig_proxy::MultisigProxy)
        .sign(action_id)
        .with_result(ExpectError(4, "action does not exist"))
        .run();

    // add another board member
    let new_board_member_address_expr: AddressExpr = AddressExpr("new-board-member");

    state.world.account(new_board_member_address_expr).nonce(1);

    let action_id = state.propose_add_board_member(new_board_member_address_expr);
    state.sign(action_id);
    state.perform(action_id);

    // change quorum to 2
    let action_id = state.propose_change_quorum(new_quorum);
    state.sign(action_id);
    state.perform(action_id);
}

#[test]
fn test_transfer_execute_to_user() {
    let mut state = MultisigTestState::new();
    state.deploy_multisig_contract();

    let new_user_address_expr: AddressExpr = AddressExpr("new-user");
    state.world.account(new_user_address_expr).nonce(1);

    let amount: u64 = 100;

    state
        .world
        .tx()
        .from(PROPOSER_ADDRESS_EXPR)
        .to(SC_MULTISIG_EXPR)
        .typed(multisig_proxy::MultisigProxy)
        .deposit()
        .egld(amount)
        .run();

    state
        .world
        .check_account(SC_MULTISIG_EXPR)
        .balance(amount.to_string().as_str());

    // failed attempt
    state
        .world
        .tx()
        .from(PROPOSER_ADDRESS_EXPR)
        .to(SC_MULTISIG_EXPR)
        .typed(multisig_proxy::MultisigProxy)
        .propose_transfer_execute(
            new_user_address_expr.eval_to_array(),
            0u64,
            FunctionCall::empty(),
        )
        .with_result(ExpectError(4, "proposed action has no effect"))
        .run();

    // propose
    let action_id = state
        .world
        .tx()
        .from(PROPOSER_ADDRESS_EXPR)
        .to(SC_MULTISIG_EXPR)
        .typed(multisig_proxy::MultisigProxy)
        .propose_transfer_execute(
            new_user_address_expr.eval_to_array(),
            amount,
            FunctionCall::empty(),
        )
        .returns(ReturnsResult)
        .run();
    state.sign(action_id);
    state.perform(action_id);

    state
        .world
        .check_account(new_user_address_expr)
        .balance(amount.to_string().as_str());
}

#[test]
fn test_transfer_execute_sc_all() {
    let mut state = MultisigTestState::new();
    state.deploy_multisig_contract().deploy_adder_contract();

    let adder_call = state
        .world
        .tx()
        .typed(adder_proxy::AdderProxy)
        .add(5u64)
        .into_function_call();
    let adder_address = AddressValue::from(SC_ADDER_EXPR.eval_to_expr().as_str()).to_address();

    let action_id = state.propose_transfer_execute(adder_address, 0u64, adder_call);
    state.sign(action_id);
    state.perform(action_id);

    state
        .world
        .query()
        .to(SC_ADDER_EXPR)
        .typed(adder_proxy::AdderProxy)
        .sum()
        .with_result(ExpectValue(BigUint::from(10u64)))
        .run();
}

#[test]
fn test_async_call_to_sc() {
    let mut state = MultisigTestState::new();
    state.deploy_multisig_contract().deploy_adder_contract();

    let adder_call = state.adder_contract.add(5u64).into_function_call();

    let action_id = state.propose_async_call(state.adder_address.clone(), 0u64, adder_call);
    state.sign(action_id);
    state.perform(action_id);

    state
        .world
        .query()
        .to(SC_ADDER_EXPR)
        .typed(adder_proxy::AdderProxy)
        .sum()
        .returns(ExpectValue(10u64))
        .run();
}

#[test]
fn test_deploy_and_upgrade_from_source() {
    let mut state = MultisigTestState::new();
    state.deploy_multisig_contract().deploy_adder_contract();

    const NEW_ADDER_ADDRESS_EXPR: &str = "sc:new-adder";
    state.world.set_state_step(SetStateStep::new().new_address(
        MULTISIG_ADDRESS_EXPR,
        0,
        NEW_ADDER_ADDRESS_EXPR,
    ));

    let new_adder_address = AddressValue::from(NEW_ADDER_ADDRESS_EXPR).to_address();

    let action_id = state.propose_sc_deploy_from_source(
        0u64,
        state.adder_address.clone(),
        CodeMetadata::all(),
        MultiValueVec::from([top_encode_to_vec_u8_or_panic(&5u64)]),
    );
    state.sign(action_id);
    state.world.sc_call(
        ScCallStep::new()
            .from(BOARD_MEMBER_ADDRESS_EXPR)
            .call(state.multisig_contract.perform_action_endpoint(action_id))
            .expect_value(OptionalValue::Some(new_adder_address.clone())),
    );

    let adder_call = state.adder_contract.add(5u64).into_function_call();

    let action_id = state.propose_transfer_execute(new_adder_address, 0u64, adder_call);
    state.sign(action_id);
    state.perform(action_id);

    let mut new_adder_contract = AdderContract::new(NEW_ADDER_ADDRESS_EXPR);

    state.world.sc_query(
        ScQueryStep::new()
            .call(new_adder_contract.sum())
            .expect_value(SingleValue::from(BigUint::from(10u64))),
    );

    const FACTORIAL_ADDRESS_EXPR: &str = "sc:factorial";
    const FACTORIAL_PATH_EXPR: &str = "mxsc:test-contracts/factorial.mxsc.json";

    let factorial_code = state.world.code_expression(FACTORIAL_PATH_EXPR);
    let factorial_address = AddressValue::from(FACTORIAL_ADDRESS_EXPR).to_address();

    state
        .world
        .register_contract(FACTORIAL_PATH_EXPR, factorial::ContractBuilder);
    state.world.set_state_step(SetStateStep::new().put_account(
        FACTORIAL_ADDRESS_EXPR,
        Account::new().nonce(1).code(factorial_code.clone()),
    ));

    let action_id = state.propose_sc_upgrade_from_source(
        state.adder_address.clone(),
        0u64,
        factorial_address,
        CodeMetadata::all(),
        MultiValueVec::new(),
    );
    state.sign(action_id);
    state.perform(action_id);

    state.world.check_state_step(
        CheckStateStep::new()
            .put_account(ADDER_ADDRESS_EXPR, CheckAccount::new().code(factorial_code)),
    );
}
