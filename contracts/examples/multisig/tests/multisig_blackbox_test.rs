use multiversx_sc::codec::top_encode_to_vec_u8_or_panic;
use multiversx_sc_scenario::imports::*;

use adder::adder_proxy;
use multisig::multisig_proxy;
use num_bigint::BigUint;

const SC_ADDER_EXPR: ScExpr = ScExpr("adder");
const_address_expr!(ADDER_OWNER_ADDRESS_EXPR = "adder-owner");
const ADDER_CODE_EXPR: MxscExpr = MxscExpr("test-contracts/adder.mxsc.json");
const_address_expr!(BOARD_MEMBER_ADDRESS_EXPR = "board-member");
const SC_MULTISIG_EXPR: ScExpr = ScExpr("multisig");
const MULTISIG_CODE_EXPR: MxscExpr = MxscExpr("output/multisig.mxsc.json");
const_address_expr!(OWNER_ADDRESS_EXPR = "owner");
const_address_expr!(PROPOSER_ADDRESS_EXPR = "proposer");
const PROPOSER_BALANCE_EXPR: &str = "100,000,000";
const QUORUM_SIZE: usize = 1;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/examples/multisig");

    blockchain.register_contract(MULTISIG_CODE_EXPR, multisig::ContractBuilder);
    blockchain.register_contract(ADDER_CODE_EXPR, adder::ContractBuilder);
    blockchain
}

struct MultisigTestState {
    world: ScenarioWorld,
}

impl MultisigTestState {
    fn new() -> Self {
        let mut world = world();

        world
            .account(OWNER_ADDRESS_EXPR)
            .nonce(1)
            .account(PROPOSER_ADDRESS_EXPR)
            .nonce(1)
            .balance(PROPOSER_BALANCE_EXPR)
            .account(BOARD_MEMBER_ADDRESS_EXPR)
            .nonce(1)
            .account(ADDER_OWNER_ADDRESS_EXPR)
            .nonce(1);

        Self { world }
    }

    fn deploy_multisig_contract(&mut self) -> &mut Self {
        let board_members = MultiValueVec::from(vec![BOARD_MEMBER_ADDRESS_EXPR.eval_to_array()]);

        self.world
            .tx()
            .from(OWNER_ADDRESS_EXPR)
            .typed(multisig_proxy::MultisigProxy)
            .init(QUORUM_SIZE, board_members)
            .code(MULTISIG_CODE_EXPR)
            .new_address(SC_MULTISIG_EXPR)
            .run();

        let action_id: usize = self
            .world
            .tx()
            .from(BOARD_MEMBER_ADDRESS_EXPR)
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
            .new_address(SC_ADDER_EXPR)
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
        to: ScExpr,
        egld_amount: u64,
        contract_call: FunctionCall<StaticApi>,
    ) -> usize {
        self.world
            .tx()
            .from(PROPOSER_ADDRESS_EXPR)
            .to(SC_MULTISIG_EXPR)
            .typed(multisig_proxy::MultisigProxy)
            .propose_transfer_execute(to.eval_to_array(), egld_amount, contract_call)
            .returns(ReturnsResult)
            .run()
    }

    fn propose_async_call(
        &mut self,
        to: ScExpr,
        egld_amount: u64,
        contract_call: FunctionCall<StaticApi>,
    ) -> usize {
        self.world
            .tx()
            .from(PROPOSER_ADDRESS_EXPR)
            .to(SC_MULTISIG_EXPR)
            .typed(multisig_proxy::MultisigProxy)
            .propose_async_call(to.eval_to_array(), egld_amount, contract_call)
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
        source: ScExpr,
        code_metadata: CodeMetadata,
        arguments: MultiValueVec<Vec<u8>>,
    ) -> usize {
        self.world
            .tx()
            .from(PROPOSER_ADDRESS_EXPR)
            .to(SC_MULTISIG_EXPR)
            .typed(multisig_proxy::MultisigProxy)
            .propose_sc_deploy_from_source(amount, source.eval_to_array(), code_metadata, arguments)
            .returns(ReturnsResult)
            .run()
    }

    fn propose_sc_upgrade_from_source(
        &mut self,
        sc_address: ScExpr,
        amount: u64,
        source: ScExpr,
        code_metadata: CodeMetadata,
        arguments: MultiValueVec<Vec<u8>>,
    ) -> usize {
        self.world
            .tx()
            .from(PROPOSER_ADDRESS_EXPR)
            .to(SC_MULTISIG_EXPR)
            .typed(multisig_proxy::MultisigProxy)
            .propose_sc_upgrade_from_source(
                sc_address.eval_to_array(),
                amount,
                source.eval_to_array(),
                code_metadata,
                arguments,
            )
            .returns(ReturnsResult)
            .run()
    }

    fn perform(&mut self, action_id: usize) {
        self.world
            .tx()
            .from(BOARD_MEMBER_ADDRESS_EXPR)
            .to(SC_MULTISIG_EXPR)
            .typed(multisig_proxy::MultisigProxy)
            .perform_action_endpoint(action_id)
            .run();
    }

    fn perform_and_expect_err(&mut self, action_id: usize, err_message: &str) {
        self.world
            .tx()
            .from(BOARD_MEMBER_ADDRESS_EXPR)
            .to(SC_MULTISIG_EXPR)
            .typed(multisig_proxy::MultisigProxy)
            .perform_action_endpoint(action_id)
            .with_result(ExpectError(4, err_message))
            .run();
    }

    fn sign(&mut self, action_id: usize) {
        self.world
            .tx()
            .from(BOARD_MEMBER_ADDRESS_EXPR)
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

    let new_board_member_expr: AddressExpr = AddressExpr::new("new-board-member");

    state.world.account(new_board_member_expr).nonce(1);

    state.expect_user_role(new_board_member_expr, multisig_proxy::UserRole::None);

    let action_id = state.propose_add_board_member(new_board_member_expr);
    state.sign(action_id);
    state.perform(action_id);

    let expected_value = MultiValueVec::from(vec![
        BOARD_MEMBER_ADDRESS_EXPR.eval_to_array(),
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

    let new_proposer_address_expr = AddressExpr::new("new-proposer");

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

    let action_id = state.propose_remove_user(BOARD_MEMBER_ADDRESS_EXPR);
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
        .from(BOARD_MEMBER_ADDRESS_EXPR)
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
        .from(BOARD_MEMBER_ADDRESS_EXPR)
        .to(SC_MULTISIG_EXPR)
        .typed(multisig_proxy::MultisigProxy)
        .unsign(action_id)
        .run();

    state
        .world
        .tx()
        .from(BOARD_MEMBER_ADDRESS_EXPR)
        .to(SC_MULTISIG_EXPR)
        .typed(multisig_proxy::MultisigProxy)
        .discard_action(action_id)
        .run();

    // try sign discarded action
    state
        .world
        .tx()
        .from(BOARD_MEMBER_ADDRESS_EXPR)
        .to(SC_MULTISIG_EXPR)
        .typed(multisig_proxy::MultisigProxy)
        .sign(action_id)
        .with_result(ExpectError(4, "action does not exist"))
        .run();

    // add another board member
    let new_board_member_address_expr = AddressExpr::new("new-board-member");

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

    let new_user_address_expr = AddressExpr::new("new-user");
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

    let action_id = state.propose_transfer_execute(SC_ADDER_EXPR, 0u64, adder_call);
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

    let adder_call = state
        .world
        .tx()
        .typed(adder_proxy::AdderProxy)
        .add(5u64)
        .into_function_call();

    let action_id = state.propose_async_call(SC_ADDER_EXPR, 0u64, adder_call);
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

    let new_adder_address_expr: ScExpr = ScExpr("new-adder");

    state
        .world
        .new_address(SC_MULTISIG_EXPR, 0, new_adder_address_expr);

    let action_id = state.propose_sc_deploy_from_source(
        0u64,
        SC_ADDER_EXPR,
        CodeMetadata::all(),
        MultiValueVec::from([top_encode_to_vec_u8_or_panic(&5u64)]),
    );
    state.sign(action_id);
    state
        .world
        .tx()
        .from(BOARD_MEMBER_ADDRESS_EXPR)
        .to(SC_MULTISIG_EXPR)
        .typed(multisig_proxy::MultisigProxy)
        .perform_action_endpoint(action_id)
        .returns(ExpectValue(OptionalValue::Some(
            new_adder_address_expr.to_address(),
        )))
        .run();

    let adder_call = state
        .world
        .tx()
        .to(SC_ADDER_EXPR)
        .typed(adder_proxy::AdderProxy)
        .add(5u64)
        .into_function_call();

    let action_id = state.propose_transfer_execute(new_adder_address_expr, 0u64, adder_call);
    state.sign(action_id);
    state.perform(action_id);

    state
        .world
        .query()
        .to(new_adder_address_expr)
        .typed(adder_proxy::AdderProxy)
        .sum()
        .returns(ExpectValue(BigUint::from(10u64)))
        .run();

    let factorial_address_expr: ScExpr = ScExpr("factorial");
    let factorial_path_expr: MxscExpr = MxscExpr("test-contracts/factorial.mxsc.json");

    let factorial_code = state
        .world
        .code_expression(factorial_path_expr.eval_to_expr().as_str());

    state
        .world
        .register_contract(factorial_path_expr, factorial::ContractBuilder);

    state
        .world
        .account(factorial_address_expr)
        .code(factorial_code.clone());

    let action_id = state.propose_sc_upgrade_from_source(
        SC_ADDER_EXPR,
        0u64,
        factorial_address_expr,
        CodeMetadata::all(),
        MultiValueVec::new(),
    );
    state.sign(action_id);
    state.perform(action_id);

    state
        .world
        .check_account(SC_ADDER_EXPR)
        .code(factorial_path_expr.eval_to_expr().as_str());
}
