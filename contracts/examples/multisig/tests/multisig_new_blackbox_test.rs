#![allow(unused)]

use adder::ProxyTrait as _;
use multisig::{
    multisig_perform::ProxyTrait as _, multisig_propose::ProxyTrait as _, user_role::UserRole,
    ProxyTrait as _,
};
use multiversx_sc::{
    codec::{multi_types::MultiValueVec, test_util::top_encode_to_vec_u8_or_panic},
    storage::mappers::{SingleValue, SingleValueMapper},
    types::{Address, CodeMetadata},
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

enum Action {
    AddBoardMember(Address),
    AddProposer(Address),
    RemoveUser(Address),
    ChangeQuorum(usize),
    SendTransferExecute(Address, u64, String, MultiValueVec<Vec<u8>>),
    SendAsyncCall(Address, u64, String, MultiValueVec<Vec<u8>>),
    SCDeployFromSource(u64, Address, CodeMetadata, MultiValueVec<Vec<u8>>),
    SCUpgradeFromSource(Address, u64, Address, CodeMetadata, MultiValueVec<Vec<u8>>),
}

struct MultisigTestState {
    world: ScenarioWorld,
    owner_address: Address,
    proposer_address: Address,
    board_member_address: Address,
    multisig_contract: MultisigContract,
    adder_contract: AdderContract,
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
                    Account::new().nonce(1).balance("100,000,000"),
                )
                .put_account(BOARD_MEMBER_ADDRESS_EXPR, Account::new().nonce(1))
                .put_account(ADDER_OWNER_ADDRESS_EXPR, Account::new().nonce(1))
                .new_address(ADDER_OWNER_ADDRESS_EXPR, 1, ADDER_ADDRESS_EXPR),
        );

        let owner_address = AddressValue::from(OWNER_ADDRESS_EXPR).to_address();
        let proposer_address = AddressValue::from(PROPOSER_ADDRESS_EXPR).to_address();
        let board_member_address = AddressValue::from(BOARD_MEMBER_ADDRESS_EXPR).to_address();
        let multisig_contract = MultisigContract::new(MULTISIG_ADDRESS_EXPR);
        let adder_contract = AdderContract::new(ADDER_ADDRESS_EXPR);

        Self {
            world,
            owner_address,
            proposer_address,
            board_member_address,
            multisig_contract,
            adder_contract,
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

        self.check_user_role(&self.proposer_address.clone(), UserRole::Proposer);

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

    fn propose(&mut self, action: Action) -> usize {
        match action {
            Action::AddBoardMember(board_member_address) => self.world.sc_call_get_result(
                ScCallStep::new().from(PROPOSER_ADDRESS_EXPR).call(
                    self.multisig_contract
                        .propose_add_board_member(board_member_address),
                ),
            ),
            Action::AddProposer(proposer_address) => self.world.sc_call_get_result(
                ScCallStep::new().from(PROPOSER_ADDRESS_EXPR).call(
                    self.multisig_contract
                        .propose_add_proposer(proposer_address),
                ),
            ),
            Action::RemoveUser(user_address) => self.world.sc_call_get_result(
                ScCallStep::new()
                    .from(PROPOSER_ADDRESS_EXPR)
                    .call(self.multisig_contract.propose_remove_user(user_address)),
            ),
            Action::ChangeQuorum(new_quorum) => self.world.sc_call_get_result(
                ScCallStep::new()
                    .from(PROPOSER_ADDRESS_EXPR)
                    .call(self.multisig_contract.propose_change_quorum(new_quorum)),
            ),
            Action::SendTransferExecute(to, egld_amount, opt_function, arguments) => self
                .world
                .sc_call_get_result(ScCallStep::new().from(PROPOSER_ADDRESS_EXPR).call(
                    self.multisig_contract.propose_transfer_execute(
                        to,
                        egld_amount,
                        opt_function,
                        arguments,
                    ),
                )),
            Action::SendAsyncCall(to, egld_amount, opt_function, arguments) => self
                .world
                .sc_call_get_result(ScCallStep::new().from(PROPOSER_ADDRESS_EXPR).call(
                    self.multisig_contract.propose_async_call(
                        to,
                        egld_amount,
                        opt_function,
                        arguments,
                    ),
                )),
            Action::SCDeployFromSource(amount, source, code_metadata, arguments) => self
                .world
                .sc_call_get_result(ScCallStep::new().from(PROPOSER_ADDRESS_EXPR).call(
                    self.multisig_contract.propose_sc_deploy_from_source(
                        amount,
                        source,
                        code_metadata,
                        arguments,
                    ),
                )),
            Action::SCUpgradeFromSource(sc_address, amount, source, code_metadata, arguments) => {
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
            },
        }
    }

    fn perform(&mut self, action_id: usize) {
        self.world.sc_call(
            ScCallStep::new()
                .from(BOARD_MEMBER_ADDRESS_EXPR)
                .call(self.multisig_contract.perform_action_endpoint(action_id)),
        );
    }

    fn perform_with_expect(&mut self, action_id: usize, err_message: &str) {
        self.world.sc_call(
            ScCallStep::new()
                .from(BOARD_MEMBER_ADDRESS_EXPR)
                .call(self.multisig_contract.perform_action_endpoint(action_id))
                .expect(TxExpect::err(4, "str:".to_string() + err_message)),
        );
    }

    fn sign(&mut self, action_id: usize) {
        self.world.sc_call(
            ScCallStep::new()
                .from(BOARD_MEMBER_ADDRESS_EXPR)
                .call(self.multisig_contract.sign(action_id)),
        );
    }

    fn check_user_role(&mut self, user: &Address, expected_user_role: UserRole) {
        self.world.sc_query_use_result(
            ScQueryStep::new().call(self.multisig_contract.user_role(user.clone())),
            |r| {
                let user_role: UserRole = r.result.unwrap();
                assert_eq!(user_role, expected_user_role);
            },
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

    state.check_user_role(&new_board_member_address, UserRole::None);

    let action_id = state.propose(Action::AddBoardMember(new_board_member_address.clone()));
    state.sign(action_id);
    state.perform(action_id);

    state.check_user_role(&new_board_member_address, UserRole::BoardMember);

    state.world.sc_query_use_result(
        ScQueryStep::new().call(state.multisig_contract.get_all_board_members()),
        |r| {
            let board_members: MultiValueVec<Address> = r.result.unwrap();
            assert_eq!(board_members.len(), 2);
            assert_eq!(board_members.as_slice()[0], state.board_member_address);
            assert_eq!(board_members.as_slice()[1], new_board_member_address);
        },
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

    state.check_user_role(&new_proposer_address, UserRole::None);

    let action_id = state.propose(Action::AddProposer(new_proposer_address.clone()));
    state.sign(action_id);
    state.perform(action_id);

    state.check_user_role(&new_proposer_address, UserRole::Proposer);

    state.world.sc_query_use_result(
        ScQueryStep::new().call(state.multisig_contract.get_all_proposers()),
        |r| {
            let proposers: MultiValueVec<Address> = r.result.unwrap();
            assert_eq!(proposers.len(), 2);
            assert_eq!(proposers.as_slice()[0], state.proposer_address);
            assert_eq!(proposers.as_slice()[1], new_proposer_address);
        },
    );
}

#[test]
fn test_remove_proposer() {
    let mut state = MultisigTestState::new();
    state.deploy_multisig_contract();

    state.check_user_role(&state.proposer_address.clone(), UserRole::Proposer);

    let action_id = state.propose(Action::RemoveUser(state.proposer_address.clone()));
    state.sign(action_id);
    state.perform(action_id);

    state.check_user_role(&state.proposer_address.clone(), UserRole::None);

    state.world.sc_query_use_result(
        ScQueryStep::new().call(state.multisig_contract.get_all_proposers()),
        |r| {
            let proposers: MultiValueVec<Address> = r.result.unwrap();
            assert_eq!(proposers.len(), 0);
        },
    );
}

#[test]
fn test_try_remove_all_board_members() {
    let mut state = MultisigTestState::new();
    state.deploy_multisig_contract();

    let action_id = state.propose(Action::RemoveUser(state.board_member_address.clone()));
    state.sign(action_id);
    state.perform_with_expect(action_id, "quorum cannot exceed board size")
}

#[test]
fn test_change_quorum() {
    let mut state = MultisigTestState::new();
    state.deploy_multisig_contract();

    let new_quorum = 2;
    // try change quorum > board size
    let action_id = state.propose(Action::ChangeQuorum(new_quorum));
    state.sign(action_id);
    state.perform_with_expect(action_id, "quorum cannot exceed board size");

    // try discard before unsigning
    state.world.sc_call(
        ScCallStep::new()
            .from(BOARD_MEMBER_ADDRESS_EXPR)
            .call(state.multisig_contract.discard_action(action_id))
            .expect(TxExpect::err(
                4,
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
            .expect(TxExpect::err(4, "str:action does not exist")),
    );

    // add another board member
    const NEW_BOARD_MEMBER_ADDRESS_EXPR: &str = "address:new-board-member";
    let new_board_member_address = AddressValue::from(NEW_BOARD_MEMBER_ADDRESS_EXPR).to_address();

    state.world.set_state_step(
        SetStateStep::new().put_account(NEW_BOARD_MEMBER_ADDRESS_EXPR, Account::new().nonce(1)),
    );

    let action_id = state.propose(Action::AddBoardMember(new_board_member_address.clone()));
    state.sign(action_id);
    state.perform(action_id);

    // change quorum to 2
    let action_id = state.propose(Action::ChangeQuorum(new_quorum));
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

    state
        .world
        .check_state_step(CheckStateStep::new().put_account(
            PROPOSER_ADDRESS_EXPR,
            CheckAccount::new().balance("99,999,900"),
        ));

    state.world.check_state_step(
        CheckStateStep::new()
            .put_account(MULTISIG_ADDRESS_EXPR, CheckAccount::new().balance(AMOUNT)),
    );
}

#[test]
fn test_transfer_execute_sc_all() {
    let mut state = MultisigTestState::new();
    state.deploy_multisig_contract().deploy_adder_contract();

    let adder_contract_address = AddressValue::from(ADDER_ADDRESS_EXPR).to_address();
    let action_id = state.propose(Action::SendTransferExecute(
        adder_contract_address.clone(),
        0u64,
        "add".to_string(),
        MultiValueVec::from([top_encode_to_vec_u8_or_panic(&5u64)]),
    ));
    state.sign(action_id);
    state.perform(action_id);

    state
        .world
        .sc_query_use_result(ScQueryStep::new().call(state.adder_contract.sum()), |r| {
            let result: SingleValue<BigUint> = r.result.unwrap();
            let expected_sum = 10u64;
            assert_eq!(result.into(), expected_sum.into());
        });
}

#[test]
fn test_async_call_to_sc() {
    let mut state = MultisigTestState::new();
    state.deploy_multisig_contract().deploy_adder_contract();

    let adder_contract_address = AddressValue::from(ADDER_ADDRESS_EXPR).to_address();
    let action_id = state.propose(Action::SendAsyncCall(
        adder_contract_address.clone(),
        0u64,
        "add".to_string(),
        MultiValueVec::from([top_encode_to_vec_u8_or_panic(&5u64)]),
    ));
    state.sign(action_id);
    state.perform(action_id);

    state
        .world
        .sc_query_use_result(ScQueryStep::new().call(state.adder_contract.sum()), |r| {
            let result: SingleValue<BigUint> = r.result.unwrap();
            let expected_sum = 10u64;
            assert_eq!(result.into(), expected_sum.into());
        });
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

    // state.propose()
}
