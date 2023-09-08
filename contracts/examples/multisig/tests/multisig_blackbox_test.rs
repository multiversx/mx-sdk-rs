use adder::ProxyTrait as _;
use multisig::{
    multisig_perform::ProxyTrait as _, multisig_propose::ProxyTrait as _, user_role::UserRole,
    ProxyTrait as _,
};
use multiversx_sc::{
    codec::{
        multi_types::{MultiValueVec, OptionalValue},
        test_util::top_encode_to_vec_u8_or_panic,
    },
    storage::mappers::SingleValue,
    types::{Address, CodeMetadata, ContractCallNoPayment},
};
use multiversx_sc_scenario::{
    api::StaticApi,
    scenario_model::{
        Account, AddressValue, CheckAccount, CheckStateStep, ScCallStep, ScDeployStep, ScQueryStep,
        SetStateStep, TxExpect,
    },
    ContractInfo, ScenarioWorld,
};
use num_bigint::BigUint;

const ADDER_ADDRESS_EXPR: &str = "sc:adder";
const ADDER_OWNER_ADDRESS_EXPR: &str = "address:adder-owner";
const ADDER_PATH_EXPR: &str = "file:test-contracts/adder.wasm";
const BOARD_MEMBER_ADDRESS_EXPR: &str = "address:board-member";
const MULTISIG_ADDRESS_EXPR: &str = "sc:multisig";
const MULTISIG_PATH_EXPR: &str = "file:output/multisig.wasm";
const OWNER_ADDRESS_EXPR: &str = "address:owner";
const PROPOSER_ADDRESS_EXPR: &str = "address:proposer";
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
    proposer_address: Address,
    board_member_address: Address,
    multisig_contract: MultisigContract,
    adder_contract: AdderContract,
    adder_address: Address,
}

impl MultisigTestState {
    fn new() -> Self {
        let mut world = world();
        world.set_state_step(
            SetStateStep::new()
                .put_account(OWNER_ADDRESS_EXPR, Account::new().nonce(1))
                .new_address(OWNER_ADDRESS_EXPR, 1, MULTISIG_ADDRESS_EXPR)
                .put_account(
                    PROPOSER_ADDRESS_EXPR,
                    Account::new().nonce(1).balance(PROPOSER_BALANCE_EXPR),
                )
                .put_account(BOARD_MEMBER_ADDRESS_EXPR, Account::new().nonce(1))
                .put_account(ADDER_OWNER_ADDRESS_EXPR, Account::new().nonce(1))
                .new_address(ADDER_OWNER_ADDRESS_EXPR, 1, ADDER_ADDRESS_EXPR),
        );

        let proposer_address = AddressValue::from(PROPOSER_ADDRESS_EXPR).to_address();
        let board_member_address = AddressValue::from(BOARD_MEMBER_ADDRESS_EXPR).to_address();
        let multisig_contract = MultisigContract::new(MULTISIG_ADDRESS_EXPR);
        let adder_contract = AdderContract::new(ADDER_ADDRESS_EXPR);
        let adder_address = AddressValue::from(ADDER_ADDRESS_EXPR).to_address();

        Self {
            world,
            proposer_address,
            board_member_address,
            multisig_contract,
            adder_contract,
            adder_address,
        }
    }

    fn deploy_multisig_contract(&mut self) -> &mut Self {
        let multisig_code = self.world.code_expression(MULTISIG_PATH_EXPR);
        let board_members = MultiValueVec::from(vec![self.board_member_address.clone()]);

        self.world.sc_deploy(
            ScDeployStep::new()
                .from(OWNER_ADDRESS_EXPR)
                .code(multisig_code)
                .call(self.multisig_contract.init(QUORUM_SIZE, board_members)),
        );

        let action_id: usize = self.world.sc_call_get_result(
            ScCallStep::new().from(BOARD_MEMBER_ADDRESS_EXPR).call(
                self.multisig_contract
                    .propose_add_proposer(self.proposer_address.clone()),
            ),
        );
        self.sign(action_id);
        self.perform(action_id);

        self.expect_user_role(&self.proposer_address.clone(), UserRole::Proposer);

        self
    }

    fn deploy_adder_contract(&mut self) -> &mut Self {
        let adder_code = self.world.code_expression(ADDER_PATH_EXPR);

        self.world.sc_deploy(
            ScDeployStep::new()
                .from(ADDER_OWNER_ADDRESS_EXPR)
                .code(adder_code)
                .call(self.adder_contract.init(5u64)),
        );

        self
    }

    fn propose_add_board_member(&mut self, board_member_address: Address) -> usize {
        self.world.sc_call_get_result(
            ScCallStep::new().from(PROPOSER_ADDRESS_EXPR).call(
                self.multisig_contract
                    .propose_add_board_member(board_member_address),
            ),
        )
    }

    fn propose_add_proposer(&mut self, proposer_address: Address) -> usize {
        self.world.sc_call_get_result(
            ScCallStep::new().from(PROPOSER_ADDRESS_EXPR).call(
                self.multisig_contract
                    .propose_add_proposer(proposer_address),
            ),
        )
    }

    fn propose_change_quorum(&mut self, new_quorum: usize) -> usize {
        self.world.sc_call_get_result(
            ScCallStep::new()
                .from(PROPOSER_ADDRESS_EXPR)
                .call(self.multisig_contract.propose_change_quorum(new_quorum)),
        )
    }

    fn propose_transfer_execute(
        &mut self,
        to: Address,
        egld_amount: u64,
        contract_call: ContractCallNoPayment<StaticApi, ()>,
    ) -> usize {
        self.world
            .sc_call_get_result(ScCallStep::new().from(PROPOSER_ADDRESS_EXPR).call(
                self.multisig_contract.propose_transfer_execute(
                    to,
                    egld_amount,
                    contract_call.endpoint_name,
                    contract_call.arg_buffer.into_multi_value_encoded(),
                ),
            ))
    }

    fn propose_async_call(
        &mut self,
        to: Address,
        egld_amount: u64,
        contract_call: ContractCallNoPayment<StaticApi, ()>,
    ) -> usize {
        self.world
            .sc_call_get_result(ScCallStep::new().from(PROPOSER_ADDRESS_EXPR).call(
                self.multisig_contract.propose_async_call(
                    to,
                    egld_amount,
                    contract_call.endpoint_name,
                    contract_call.arg_buffer.into_multi_value_encoded(),
                ),
            ))
    }

    fn propose_remove_user(&mut self, user_address: Address) -> usize {
        self.world.sc_call_get_result(
            ScCallStep::new()
                .from(PROPOSER_ADDRESS_EXPR)
                .call(self.multisig_contract.propose_remove_user(user_address)),
        )
    }

    fn propose_sc_deploy_from_source(
        &mut self,
        amount: u64,
        source: Address,
        code_metadata: CodeMetadata,
        arguments: MultiValueVec<Vec<u8>>,
    ) -> usize {
        self.world
            .sc_call_get_result(ScCallStep::new().from(PROPOSER_ADDRESS_EXPR).call(
                self.multisig_contract.propose_sc_deploy_from_source(
                    amount,
                    source,
                    code_metadata,
                    arguments,
                ),
            ))
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
            .sc_call_get_result(ScCallStep::new().from(PROPOSER_ADDRESS_EXPR).call(
                self.multisig_contract.propose_sc_upgrade_from_source(
                    sc_address,
                    amount,
                    source,
                    code_metadata,
                    arguments,
                ),
            ))
    }

    fn perform(&mut self, action_id: usize) {
        self.world.sc_call(
            ScCallStep::new()
                .from(BOARD_MEMBER_ADDRESS_EXPR)
                .call(self.multisig_contract.perform_action_endpoint(action_id)),
        );
    }

    fn perform_and_expect_err(&mut self, action_id: usize, err_message: &str) {
        self.world.sc_call(
            ScCallStep::new()
                .from(BOARD_MEMBER_ADDRESS_EXPR)
                .call(self.multisig_contract.perform_action_endpoint(action_id))
                .expect(TxExpect::user_error("str:".to_string() + err_message)),
        );
    }

    fn sign(&mut self, action_id: usize) {
        self.world.sc_call(
            ScCallStep::new()
                .from(BOARD_MEMBER_ADDRESS_EXPR)
                .call(self.multisig_contract.sign(action_id)),
        );
    }

    fn expect_user_role(&mut self, user: &Address, expected_user_role: UserRole) {
        self.world.sc_query(
            ScQueryStep::new()
                .call(self.multisig_contract.user_role(user.clone()))
                .expect_value(expected_user_role),
        );
    }
}

#[test]
fn test_add_board_member() {
    let mut state = MultisigTestState::new();
    state.deploy_multisig_contract();

    const NEW_BOARD_MEMBER_ADDRESS_EXPR: &str = "address:new-board-member";
    let new_board_member_address = AddressValue::from(NEW_BOARD_MEMBER_ADDRESS_EXPR).to_address();

    state.world.set_state_step(
        SetStateStep::new().put_account(NEW_BOARD_MEMBER_ADDRESS_EXPR, Account::new().nonce(1)),
    );

    state.expect_user_role(&new_board_member_address, UserRole::None);

    let action_id = state.propose_add_board_member(new_board_member_address.clone());
    state.sign(action_id);
    state.perform(action_id);

    state.expect_user_role(&new_board_member_address, UserRole::BoardMember);
    state.world.sc_query(
        ScQueryStep::new()
            .call(state.multisig_contract.get_all_board_members())
            .expect_value(MultiValueVec::<Address>::from(vec![
                state.board_member_address.clone(),
                new_board_member_address.clone(),
            ])),
    );
}

#[test]
fn test_add_proposer() {
    let mut state = MultisigTestState::new();
    state.deploy_multisig_contract();

    const NEW_PROPOSER_ADDRESS_EXPR: &str = "address:new-proposer";
    let new_proposer_address = AddressValue::from(NEW_PROPOSER_ADDRESS_EXPR).to_address();

    state.world.set_state_step(
        SetStateStep::new().put_account(NEW_PROPOSER_ADDRESS_EXPR, Account::new().nonce(1)),
    );

    state.expect_user_role(&new_proposer_address, UserRole::None);

    let action_id = state.propose_add_proposer(new_proposer_address.clone());
    state.sign(action_id);
    state.perform(action_id);

    state.expect_user_role(&new_proposer_address, UserRole::Proposer);
    state.world.sc_query(
        ScQueryStep::new()
            .call(state.multisig_contract.get_all_proposers())
            .expect_value(MultiValueVec::<Address>::from(vec![
                state.proposer_address.clone(),
                new_proposer_address.clone(),
            ])),
    );
}

#[test]
fn test_remove_proposer() {
    let mut state = MultisigTestState::new();
    state.deploy_multisig_contract();

    state.expect_user_role(&state.proposer_address.clone(), UserRole::Proposer);

    let action_id = state.propose_remove_user(state.proposer_address.clone());
    state.sign(action_id);
    state.perform(action_id);

    state.expect_user_role(&state.proposer_address.clone(), UserRole::None);
    state.world.sc_query(
        ScQueryStep::new()
            .call(state.multisig_contract.get_all_proposers())
            .expect_value(MultiValueVec::<Address>::new()),
    );
}

#[test]
fn test_try_remove_all_board_members() {
    let mut state = MultisigTestState::new();
    state.deploy_multisig_contract();

    let action_id = state.propose_remove_user(state.board_member_address.clone());
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
    state.world.sc_call(
        ScCallStep::new()
            .from(BOARD_MEMBER_ADDRESS_EXPR)
            .call(state.multisig_contract.discard_action(action_id))
            .expect(TxExpect::user_error(
                "str:cannot discard action with valid signatures",
            )),
    );

    // unsign and discard action
    state.world.sc_call(
        ScCallStep::new()
            .from(BOARD_MEMBER_ADDRESS_EXPR)
            .call(state.multisig_contract.unsign(action_id)),
    );

    state.world.sc_call(
        ScCallStep::new()
            .from(BOARD_MEMBER_ADDRESS_EXPR)
            .call(state.multisig_contract.discard_action(action_id)),
    );

    // try sign discarded action
    state.world.sc_call(
        ScCallStep::new()
            .from(BOARD_MEMBER_ADDRESS_EXPR)
            .call(state.multisig_contract.sign(action_id))
            .expect(TxExpect::user_error("str:action does not exist")),
    );

    // add another board member
    const NEW_BOARD_MEMBER_ADDRESS_EXPR: &str = "address:new-board-member";
    let new_board_member_address = AddressValue::from(NEW_BOARD_MEMBER_ADDRESS_EXPR).to_address();

    state.world.set_state_step(
        SetStateStep::new().put_account(NEW_BOARD_MEMBER_ADDRESS_EXPR, Account::new().nonce(1)),
    );

    let action_id = state.propose_add_board_member(new_board_member_address.clone());
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

    const NEW_USER_ADDRESS_EXPR: &str = "address:new-user";
    state.world.set_state_step(
        SetStateStep::new().put_account(NEW_USER_ADDRESS_EXPR, Account::new().nonce(1)),
    );

    const AMOUNT: &str = "100";

    state.world.sc_call(
        ScCallStep::new()
            .from(PROPOSER_ADDRESS_EXPR)
            .egld_value(AMOUNT)
            .call(state.multisig_contract.deposit()),
    );

    state.world.check_state_step(
        CheckStateStep::new()
            .put_account(MULTISIG_ADDRESS_EXPR, CheckAccount::new().balance(AMOUNT)),
    );

    // failed attempt
    let new_user_address = AddressValue::from(NEW_USER_ADDRESS_EXPR).to_address();

    state.world.sc_call(
        ScCallStep::new()
            .from(PROPOSER_ADDRESS_EXPR)
            .call(state.multisig_contract.propose_transfer_execute(
                new_user_address.clone(),
                0u64,
                OptionalValue::<String>::None,
                MultiValueVec::<Vec<u8>>::new(),
            ))
            .expect(TxExpect::user_error("str:proposed action has no effect")),
    );

    // propose
    let action_id =
        state
            .world
            .sc_call_get_result(ScCallStep::new().from(PROPOSER_ADDRESS_EXPR).call(
                state.multisig_contract.propose_transfer_execute(
                    new_user_address.clone(),
                    AMOUNT.parse::<u64>().unwrap(),
                    OptionalValue::<String>::None,
                    MultiValueVec::<Vec<u8>>::new(),
                ),
            ));
    state.sign(action_id);
    state.perform(action_id);

    state.world.check_state_step(
        CheckStateStep::new()
            .put_account(NEW_USER_ADDRESS_EXPR, CheckAccount::new().balance(AMOUNT)),
    );
}

#[test]
fn test_transfer_execute_sc_all() {
    let mut state = MultisigTestState::new();
    state.deploy_multisig_contract().deploy_adder_contract();

    let adder_call = state.adder_contract.add(5u64);

    let action_id = state.propose_transfer_execute(state.adder_address.clone(), 0u64, adder_call);
    state.sign(action_id);
    state.perform(action_id);

    state.world.sc_query(
        ScQueryStep::new()
            .call(state.adder_contract.sum())
            .expect_value(SingleValue::from(BigUint::from(10u64))),
    );
}

#[test]
fn test_async_call_to_sc() {
    let mut state = MultisigTestState::new();
    state.deploy_multisig_contract().deploy_adder_contract();

    let adder_call = state.adder_contract.add(5u64);

    let action_id = state.propose_async_call(state.adder_address.clone(), 0u64, adder_call);
    state.sign(action_id);
    state.perform(action_id);

    state.world.sc_query(
        ScQueryStep::new()
            .call(state.adder_contract.sum())
            .expect_value(SingleValue::from(BigUint::from(10u64))),
    );
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

    let adder_call = state.adder_contract.add(5u64);

    let action_id = state.propose_transfer_execute(new_adder_address.clone(), 0u64, adder_call);
    state.sign(action_id);
    state.perform(action_id);

    let mut new_adder_contract = AdderContract::new(NEW_ADDER_ADDRESS_EXPR);

    state.world.sc_query(
        ScQueryStep::new()
            .call(new_adder_contract.sum())
            .expect_value(SingleValue::from(BigUint::from(10u64))),
    );

    const FACTORIAL_ADDRESS_EXPR: &str = "sc:factorial";
    const FACTORIAL_PATH_EXPR: &str = "file:test-contracts/factorial.wasm";

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
        factorial_address.clone(),
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
