#![allow(unused)]

use multisig::{
    multisig_perform::ProxyTrait as _, multisig_propose::ProxyTrait as _, user_role::UserRole,
    ProxyTrait as _,
};
use multiversx_sc::{codec::multi_types::MultiValueVec, types::Address};
use multiversx_sc_scenario::{
    api::StaticApi,
    scenario_model::{Account, AddressValue, ScCallStep, ScDeployStep, ScQueryStep, SetStateStep},
    ContractInfo, ScenarioWorld,
};

const BOARD_MEMBER_ADDRESS_EXPR: &str = "address:board-member";
const MULTISIG_ADDRESS_EXPR: &str = "sc:multisig";
const MULTISIG_PATH_EXPR: &str = "file:output/multisig.wasm";
const OWNER_ADDRESS_EXPR: &str = "address:owner";
const PROPOSER_ADDRESS_EXPR: &str = "address:proposer";
const QUORUM_SIZE: usize = 1;

type MultisigContract = ContractInfo<multisig::Proxy<StaticApi>>;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/examples/multisig");

    blockchain.register_contract(MULTISIG_PATH_EXPR, multisig::ContractBuilder);
    blockchain
}

enum Action {
    AddBoardMember(Address),
    AddProposer(Address),
    RemoveUser(Address),
    ChangeQuorum(usize),
}

struct MultisigTestState {
    world: ScenarioWorld,
    owner_address: Address,
    proposer_address: Address,
    board_member_address: Address,
    multisig_contract: MultisigContract,
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
                    Account::new().nonce(1).balance("100_000_000"),
                )
                .put_account(BOARD_MEMBER_ADDRESS_EXPR, Account::new().nonce(1)),
        );

        let owner_address = AddressValue::from(OWNER_ADDRESS_EXPR).to_address();
        let proposer_address = AddressValue::from(PROPOSER_ADDRESS_EXPR).to_address();
        let board_member_address = AddressValue::from(BOARD_MEMBER_ADDRESS_EXPR).to_address();
        let multisig_contract = MultisigContract::new(MULTISIG_ADDRESS_EXPR);

        Self {
            world,
            owner_address,
            proposer_address,
            board_member_address,
            multisig_contract,
        }
    }

    fn deploy(&mut self) -> &mut Self {
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
        }
    }

    fn perform(&mut self, action_id: usize) {
        self.world.sc_call(
            ScCallStep::new()
                .from(BOARD_MEMBER_ADDRESS_EXPR)
                .call(self.multisig_contract.perform_action_endpoint(action_id)),
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
    state.deploy();

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
    state.deploy();

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
fn test_remove_proposer() {}

#[test]
fn test_try_remove_all_board_members() {}

#[test]
fn test_change_quorum() {}

#[test]
fn test_transfer_execute_to_user() {}

#[test]
fn test_transfer_execute_sc_all() {}

#[test]
fn test_async_call_to_sc() {}

#[test]
fn test_deploy_and_upgrade_from_source() {}
