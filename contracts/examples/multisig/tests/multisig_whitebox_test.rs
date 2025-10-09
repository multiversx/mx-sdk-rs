mod adder_proxy;

use multiversx_sc_scenario::imports::*;

use multisig::{
    multisig_perform::MultisigPerformModule, multisig_propose::MultisigProposeModule,
    user_role::UserRole, Multisig,
};

const OWNER_ADDRESS: TestAddress = TestAddress::new("owner");
const PROPOSER_ADDRESS: TestAddress = TestAddress::new("proposer");
const BOARD_MEMBER_ADDRESS: TestAddress = TestAddress::new("board-member");
const MULTISIG_ADDRESS: TestSCAddress = TestSCAddress::new("multisig");
const MULTISIG_PATH_EXPR: MxscPath = MxscPath::new("mxsc:output/multisig.mxsc.json");
const QUORUM_SIZE: usize = 1;

const ADDER_OWNER_ADDRESS: TestAddress = TestAddress::new("adder-owner");
const ADDER_ADDRESS: TestSCAddress = TestSCAddress::new("adder");
const NEW_ADDER_ADDRESS: TestSCAddress = TestSCAddress::new("new-adder");
const ADDER_CODE_PATH: MxscPath = MxscPath::new("test-contracts/adder.mxsc.json");
const FACTORIAL_ADDRESS: TestSCAddress = TestSCAddress::new("factorial");
const FACTORIAL_PATH_EXPR: MxscPath = MxscPath::new("test-contracts/factorial.mxsc.json");

type RustBigUint = num_bigint::BigUint;

pub enum ActionRaw {
    _Nothing,
    AddBoardMember(Address),
    AddProposer(Address),
    RemoveUser(Address),
    ChangeQuorum(usize),
    SendTransferExecute(CallActionDataRaw),
    SendAsyncCall(CallActionDataRaw),
    SCDeployFromSource {
        amount: RustBigUint,
        source: Address,
        code_metadata: CodeMetadata,
        arguments: Vec<BoxedBytes>,
    },
    SCUpgradeFromSource {
        sc_address: Address,
        amount: RustBigUint,
        source: Address,
        code_metadata: CodeMetadata,
        arguments: Vec<BoxedBytes>,
    },
}

pub struct CallActionDataRaw {
    pub to: Address,
    pub egld_amount: RustBigUint,
    pub endpoint_name: BoxedBytes,
    pub arguments: Vec<BoxedBytes>,
}

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new()
        .executor_config(ExecutorConfig::Debugger.then(ExecutorConfig::Experimental));

    blockchain.set_current_dir_from_workspace("contracts/examples/multisig");
    blockchain.register_contract(MULTISIG_PATH_EXPR, multisig::ContractBuilder);
    blockchain
}

fn setup() -> ScenarioWorld {
    // setup
    let mut world = world();

    world.account(OWNER_ADDRESS).nonce(1);
    world
        .account(PROPOSER_ADDRESS)
        .nonce(1)
        .balance(100_000_000u64);
    world.account(BOARD_MEMBER_ADDRESS).nonce(1);

    // init multisig
    world
        .tx()
        .from(OWNER_ADDRESS)
        .raw_deploy()
        .code(MULTISIG_PATH_EXPR)
        .new_address(MULTISIG_ADDRESS)
        .whitebox(multisig::contract_obj, |sc| {
            let mut board_members = ManagedVec::new();
            board_members.push(BOARD_MEMBER_ADDRESS.to_managed_address());

            sc.init(QUORUM_SIZE, board_members.into());
            sc.change_user_role(0, PROPOSER_ADDRESS.to_managed_address(), UserRole::Proposer);
        });

    world
}

#[test]
fn whitebox_init() {
    setup();
}

fn call_propose(
    world: &mut ScenarioWorld,
    action: ActionRaw,
    expected_message: Option<&str>,
) -> usize {
    let egld_amount = match &action {
        ActionRaw::SendTransferExecute(call_data) => call_data.egld_amount.clone(),
        ActionRaw::SendAsyncCall(call_data) => call_data.egld_amount.clone(),
        ActionRaw::SCDeployFromSource { amount, .. } => amount.clone(),
        ActionRaw::SCUpgradeFromSource { amount, .. } => amount.clone(),
        _ => rust_biguint!(0),
    };

    let mut action_id = 0;

    let transaction = world
        .tx()
        .from(PROPOSER_ADDRESS)
        .to(MULTISIG_ADDRESS)
        .egld(BigUint::from(egld_amount));

    let transaction_with_err = match expected_message {
        Some(message) => transaction.returns(ExpectError(4u64, message)),
        None => transaction.returns(ExpectError(0u64, "")),
    };

    transaction_with_err.whitebox(multisig::contract_obj, |sc| {
        action_id = match action {
            ActionRaw::_Nothing => panic!("Invalid action"),
            ActionRaw::AddBoardMember(addr) => sc.propose_add_board_member(managed_address!(&addr)),
            ActionRaw::AddProposer(addr) => sc.propose_add_proposer(managed_address!(&addr)),
            ActionRaw::RemoveUser(addr) => sc.propose_remove_user(managed_address!(&addr)),
            ActionRaw::ChangeQuorum(new_size) => sc.propose_change_quorum(new_size),
            ActionRaw::SendTransferExecute(call_data) => sc.propose_transfer_execute(
                managed_address!(&call_data.to),
                call_data.egld_amount.into(),
                FunctionCall {
                    function_name: call_data.endpoint_name.into(),
                    arg_buffer: call_data.arguments.into(),
                },
            ),
            ActionRaw::SendAsyncCall(call_data) => sc.propose_async_call(
                managed_address!(&call_data.to),
                call_data.egld_amount.into(),
                FunctionCall {
                    function_name: call_data.endpoint_name.into(),
                    arg_buffer: call_data.arguments.into(),
                },
            ),
            ActionRaw::SCDeployFromSource {
                amount,
                source,
                code_metadata,
                arguments,
            } => sc.propose_sc_deploy_from_source(
                amount.into(),
                managed_address!(&source),
                code_metadata,
                boxed_bytes_vec_to_managed(arguments).into(),
            ),
            ActionRaw::SCUpgradeFromSource {
                sc_address,
                amount,
                source,
                code_metadata,
                arguments,
            } => sc.propose_sc_upgrade_from_source(
                managed_address!(&sc_address),
                amount.into(),
                managed_address!(&source),
                code_metadata,
                boxed_bytes_vec_to_managed(arguments).into(),
            ),
        }
    });

    action_id
}

#[test]
fn whitebox_add_board_member() {
    let mut world = setup();

    const NEW_BOARD_MEMBER_ADDRESS: TestAddress = TestAddress::new("new-board-member");
    world.account(NEW_BOARD_MEMBER_ADDRESS).nonce(1);

    world
        .query()
        .to(MULTISIG_ADDRESS)
        .whitebox(multisig::contract_obj, |sc| {
            // check role before
            let user_role = sc.user_role(NEW_BOARD_MEMBER_ADDRESS.to_managed_address());
            assert_eq!(user_role, UserRole::None);
        });

    let action_id = call_propose(
        &mut world,
        ActionRaw::AddBoardMember(NEW_BOARD_MEMBER_ADDRESS.to_address()),
        None,
    );

    world
        .tx()
        .from(BOARD_MEMBER_ADDRESS)
        .to(MULTISIG_ADDRESS)
        .whitebox(multisig::contract_obj, |sc| sc.sign(action_id));

    world
        .tx()
        .from(BOARD_MEMBER_ADDRESS)
        .to(MULTISIG_ADDRESS)
        .whitebox(multisig::contract_obj, |sc| {
            let _ = sc.perform_action_endpoint(action_id);
        });

    world
        .query()
        .to(MULTISIG_ADDRESS)
        .whitebox(multisig::contract_obj, |sc| {
            // check role after
            let user_role = sc.user_role(NEW_BOARD_MEMBER_ADDRESS.to_managed_address());
            assert_eq!(user_role, UserRole::BoardMember);

            let board_members = sc.get_all_board_members().to_vec();
            assert_eq!(*board_members.get(0), BOARD_MEMBER_ADDRESS);
            assert_eq!(*board_members.get(1), NEW_BOARD_MEMBER_ADDRESS);
        });
}

#[test]
fn whitebox_add_proposer() {
    let mut world = setup();

    const NEW_PROPOSER_ADDRESS: TestAddress = TestAddress::new("new-proposer");
    world.account(NEW_PROPOSER_ADDRESS).nonce(1);

    world
        .query()
        .to(MULTISIG_ADDRESS)
        .whitebox(multisig::contract_obj, |sc| {
            // check role before
            let user_role = sc.user_role(NEW_PROPOSER_ADDRESS.to_managed_address());
            assert_eq!(user_role, UserRole::None);
        });

    let action_id = call_propose(
        &mut world,
        ActionRaw::AddProposer(NEW_PROPOSER_ADDRESS.to_address()),
        None,
    );

    world
        .tx()
        .from(BOARD_MEMBER_ADDRESS)
        .to(MULTISIG_ADDRESS)
        .whitebox(multisig::contract_obj, |sc| sc.sign(action_id));

    world
        .tx()
        .from(BOARD_MEMBER_ADDRESS)
        .to(MULTISIG_ADDRESS)
        .whitebox(multisig::contract_obj, |sc| {
            let _ = sc.perform_action_endpoint(action_id);
        });

    world
        .query()
        .to(MULTISIG_ADDRESS)
        .whitebox(multisig::contract_obj, |sc| {
            // check role after
            let user_role = sc.user_role(NEW_PROPOSER_ADDRESS.to_managed_address());
            assert_eq!(user_role, UserRole::Proposer);

            let proposers = sc.get_all_proposers().to_vec();
            assert_eq!(*proposers.get(0), PROPOSER_ADDRESS);
            assert_eq!(*proposers.get(1), NEW_PROPOSER_ADDRESS);
        });
}

#[test]
fn whitebox_remove_proposer() {
    let mut world = setup();

    world
        .query()
        .to(MULTISIG_ADDRESS)
        .whitebox(multisig::contract_obj, |sc| {
            // check role before
            let user_role = sc.user_role(PROPOSER_ADDRESS.to_managed_address());
            assert_eq!(user_role, UserRole::Proposer);
        });

    let action_id = call_propose(
        &mut world,
        ActionRaw::RemoveUser(PROPOSER_ADDRESS.to_address()),
        None,
    );

    world
        .tx()
        .from(BOARD_MEMBER_ADDRESS)
        .to(MULTISIG_ADDRESS)
        .whitebox(multisig::contract_obj, |sc| sc.sign(action_id));

    world
        .tx()
        .from(BOARD_MEMBER_ADDRESS)
        .to(MULTISIG_ADDRESS)
        .whitebox(multisig::contract_obj, |sc| {
            let _ = sc.perform_action_endpoint(action_id);
        });

    world
        .query()
        .to(MULTISIG_ADDRESS)
        .whitebox(multisig::contract_obj, |sc| {
            // check role after
            let user_role = sc.user_role(PROPOSER_ADDRESS.to_managed_address());
            assert_eq!(user_role, UserRole::None);

            let proposers = sc.get_all_proposers();
            assert!(proposers.is_empty());
        });
}

#[test]
fn whitebox_try_remove_all_board_members() {
    let mut world = setup();

    let action_id = call_propose(
        &mut world,
        ActionRaw::RemoveUser(BOARD_MEMBER_ADDRESS.to_address()),
        None,
    );

    world
        .tx()
        .from(BOARD_MEMBER_ADDRESS)
        .to(MULTISIG_ADDRESS)
        .whitebox(multisig::contract_obj, |sc| sc.sign(action_id));

    world
        .tx()
        .from(BOARD_MEMBER_ADDRESS)
        .to(MULTISIG_ADDRESS)
        .returns(ExpectError(4u64, "quorum cannot exceed board size"))
        .whitebox(multisig::contract_obj, |sc| {
            let _ = sc.perform_action_endpoint(action_id);
        });
}

#[test]
fn whitebox_change_quorum() {
    let mut world = setup();

    let new_quorum_size = 2;

    // try change quorum > board size
    let action_id = call_propose(&mut world, ActionRaw::ChangeQuorum(new_quorum_size), None);

    world
        .tx()
        .from(BOARD_MEMBER_ADDRESS)
        .to(MULTISIG_ADDRESS)
        .whitebox(multisig::contract_obj, |sc| sc.sign(action_id));

    world
        .tx()
        .from(BOARD_MEMBER_ADDRESS)
        .to(MULTISIG_ADDRESS)
        .returns(ExpectError(4u64, "quorum cannot exceed board size"))
        .whitebox(multisig::contract_obj, |sc| {
            let _ = sc.perform_action_endpoint(action_id);
        });

    // try discard before unsigning
    world
        .tx()
        .from(BOARD_MEMBER_ADDRESS)
        .to(MULTISIG_ADDRESS)
        .returns(ExpectError(
            4u64,
            "cannot discard action with valid signatures",
        ))
        .whitebox(multisig::contract_obj, |sc| sc.discard_action(action_id));

    // unsign and discard action
    world
        .tx()
        .from(BOARD_MEMBER_ADDRESS)
        .to(MULTISIG_ADDRESS)
        .whitebox(multisig::contract_obj, |sc| sc.unsign(action_id));

    world
        .tx()
        .from(BOARD_MEMBER_ADDRESS)
        .to(MULTISIG_ADDRESS)
        .whitebox(multisig::contract_obj, |sc| sc.discard_action(action_id));

    // try sign discarded action
    world
        .tx()
        .from(BOARD_MEMBER_ADDRESS)
        .to(MULTISIG_ADDRESS)
        .returns(ExpectError(4u64, "action does not exist"))
        .whitebox(multisig::contract_obj, |sc| {
            sc.sign(action_id);
        });

    // add another board member
    const NEW_BOARD_MEMBER_ADDRESS: TestAddress = TestAddress::new("new-board-member");
    world.account(NEW_BOARD_MEMBER_ADDRESS).nonce(1);

    let action_id = call_propose(
        &mut world,
        ActionRaw::AddBoardMember(NEW_BOARD_MEMBER_ADDRESS.to_address()),
        None,
    );

    world
        .tx()
        .from(BOARD_MEMBER_ADDRESS)
        .to(MULTISIG_ADDRESS)
        .whitebox(multisig::contract_obj, |sc| sc.sign(action_id));

    world
        .tx()
        .from(BOARD_MEMBER_ADDRESS)
        .to(MULTISIG_ADDRESS)
        .whitebox(multisig::contract_obj, |sc| {
            let _ = sc.perform_action_endpoint(action_id);
        });

    // change quorum to 2
    let action_id = call_propose(&mut world, ActionRaw::ChangeQuorum(new_quorum_size), None);

    world
        .tx()
        .from(BOARD_MEMBER_ADDRESS)
        .to(MULTISIG_ADDRESS)
        .whitebox(multisig::contract_obj, |sc| sc.sign(action_id));

    world
        .tx()
        .from(BOARD_MEMBER_ADDRESS)
        .to(MULTISIG_ADDRESS)
        .whitebox(multisig::contract_obj, |sc| {
            let _ = sc.perform_action_endpoint(action_id);
        });
}

#[test]
fn whitebox_transfer_execute_to_user() {
    let mut world = setup();

    const NEW_USER_ADDRESS: TestAddress = TestAddress::new("new-user");
    world.account(NEW_USER_ADDRESS).nonce(1);

    const EGLD_AMOUNT: u64 = 100;

    world
        .tx()
        .from(PROPOSER_ADDRESS)
        .to(MULTISIG_ADDRESS)
        .egld(EGLD_AMOUNT)
        .whitebox(multisig::contract_obj, |sc| {
            sc.deposit();
        });

    world.check_account(MULTISIG_ADDRESS).balance(EGLD_AMOUNT);

    // failed attempt
    let _ = call_propose(
        &mut world,
        ActionRaw::SendTransferExecute(CallActionDataRaw {
            to: NEW_USER_ADDRESS.to_address(),
            egld_amount: rust_biguint!(0),
            endpoint_name: BoxedBytes::empty(),
            arguments: Vec::new(),
        }),
        Some("proposed action has no effect"),
    );

    // propose
    let action_id = call_propose(
        &mut world,
        ActionRaw::SendTransferExecute(CallActionDataRaw {
            to: NEW_USER_ADDRESS.to_address(),
            egld_amount: rust_biguint!(EGLD_AMOUNT),
            endpoint_name: BoxedBytes::empty(),
            arguments: Vec::new(),
        }),
        None,
    );

    world
        .tx()
        .from(BOARD_MEMBER_ADDRESS)
        .to(MULTISIG_ADDRESS)
        .whitebox(multisig::contract_obj, |sc| sc.sign(action_id));

    world
        .tx()
        .from(BOARD_MEMBER_ADDRESS)
        .to(MULTISIG_ADDRESS)
        .whitebox(multisig::contract_obj, |sc| {
            let _ = sc.perform_action_endpoint(action_id);
        });

    world.check_account(NEW_USER_ADDRESS).balance(EGLD_AMOUNT);
}

fn deploy_adder_contract(world: &mut ScenarioWorld, initial_value: u64) {
    world
        .tx()
        .from(ADDER_OWNER_ADDRESS)
        .typed(adder_proxy::AdderProxy)
        .init(initial_value)
        .code(ADDER_CODE_PATH)
        .new_address(ADDER_ADDRESS)
        .run();
}

fn query_adder_contract(world: &mut ScenarioWorld, address: TestSCAddress, expected_value: u64) {
    world
        .query()
        .to(address)
        .typed(adder_proxy::AdderProxy)
        .sum()
        .returns(ExpectValue(expected_value))
        .run();
}

fn deploy_factorial_contract(world: &mut ScenarioWorld) {
    world
        .tx()
        .raw_deploy()
        .from(OWNER_ADDRESS)
        .code(FACTORIAL_PATH_EXPR)
        .new_address(FACTORIAL_ADDRESS)
        .run();
}

#[test]
fn whitebox_transfer_execute_sc_all() {
    let mut world = setup();

    world.account(ADDER_OWNER_ADDRESS).nonce(1);

    deploy_adder_contract(&mut world, 5u64);

    let action_id = call_propose(
        &mut world,
        ActionRaw::SendTransferExecute(CallActionDataRaw {
            to: ADDER_ADDRESS.to_address(),
            egld_amount: 0u64.into(),
            endpoint_name: BoxedBytes::from(&b"add"[..]),
            arguments: vec![BoxedBytes::from(&[5u8][..])],
        }),
        None,
    );

    world
        .tx()
        .from(BOARD_MEMBER_ADDRESS)
        .to(MULTISIG_ADDRESS)
        .whitebox(multisig::contract_obj, |sc| sc.sign(action_id));

    world
        .tx()
        .from(BOARD_MEMBER_ADDRESS)
        .to(MULTISIG_ADDRESS)
        .whitebox(multisig::contract_obj, |sc| {
            let _ = sc.perform_action_endpoint(action_id);
        });

    query_adder_contract(&mut world, ADDER_ADDRESS, 10);
}

#[test]
fn whitebox_async_call_to_sc() {
    let mut world = setup();

    world.account(ADDER_OWNER_ADDRESS).nonce(1);

    deploy_adder_contract(&mut world, 5u64);

    let action_id = call_propose(
        &mut world,
        ActionRaw::SendAsyncCall(CallActionDataRaw {
            to: ADDER_ADDRESS.to_address(),
            egld_amount: 0u64.into(),
            endpoint_name: BoxedBytes::from(&b"add"[..]),
            arguments: vec![BoxedBytes::from(&[5u8][..])],
        }),
        None,
    );

    world
        .tx()
        .from(BOARD_MEMBER_ADDRESS)
        .to(MULTISIG_ADDRESS)
        .whitebox(multisig::contract_obj, |sc| sc.sign(action_id));

    world
        .tx()
        .from(BOARD_MEMBER_ADDRESS)
        .to(MULTISIG_ADDRESS)
        .whitebox(multisig::contract_obj, |sc| {
            let _ = sc.perform_action_endpoint(action_id);
        });

    query_adder_contract(&mut world, ADDER_ADDRESS, 10);
}

#[test]
fn whitebox_deploy_and_upgrade_from_source() {
    let mut world = setup();

    world.new_address(MULTISIG_ADDRESS, 0, NEW_ADDER_ADDRESS);

    world.account(ADDER_OWNER_ADDRESS).nonce(1);

    deploy_adder_contract(&mut world, 5u64);

    let action_id = call_propose(
        &mut world,
        ActionRaw::SCDeployFromSource {
            amount: 0u64.into(),
            source: ADDER_ADDRESS.to_address(),
            code_metadata: CodeMetadata::all(),
            arguments: vec![BoxedBytes::from(&[5u8][..])],
        },
        None,
    );

    world
        .tx()
        .from(BOARD_MEMBER_ADDRESS)
        .to(MULTISIG_ADDRESS)
        .whitebox(multisig::contract_obj, |sc| sc.sign(action_id));

    world
        .tx()
        .from(BOARD_MEMBER_ADDRESS)
        .to(MULTISIG_ADDRESS)
        .whitebox(multisig::contract_obj, |sc| {
            let opt_address = sc.perform_action_endpoint(action_id);
            let addr = opt_address.into_option().unwrap().to_address();

            assert_eq!(NEW_ADDER_ADDRESS.to_address(), addr);
        });

    let action_id = call_propose(
        &mut world,
        ActionRaw::SendTransferExecute(CallActionDataRaw {
            to: NEW_ADDER_ADDRESS.to_address(),
            egld_amount: 0u64.into(),
            endpoint_name: BoxedBytes::from(&b"add"[..]),
            arguments: vec![BoxedBytes::from(&[5u8][..])],
        }),
        None,
    );

    world
        .tx()
        .from(BOARD_MEMBER_ADDRESS)
        .to(MULTISIG_ADDRESS)
        .whitebox(multisig::contract_obj, |sc| sc.sign(action_id));

    world
        .tx()
        .from(BOARD_MEMBER_ADDRESS)
        .to(MULTISIG_ADDRESS)
        .whitebox(multisig::contract_obj, |sc| {
            let _ = sc.perform_action_endpoint(action_id);
        });

    query_adder_contract(&mut world, NEW_ADDER_ADDRESS, 10);

    deploy_factorial_contract(&mut world);

    let action_id = call_propose(
        &mut world,
        ActionRaw::SCUpgradeFromSource {
            source: FACTORIAL_ADDRESS.to_address(),
            amount: 0u64.into(),
            code_metadata: CodeMetadata::all(),
            arguments: Vec::new(),
            sc_address: ADDER_ADDRESS.to_address(),
        },
        None,
    );

    world
        .tx()
        .from(BOARD_MEMBER_ADDRESS)
        .to(MULTISIG_ADDRESS)
        .whitebox(multisig::contract_obj, |sc| sc.sign(action_id));

    world
        .tx()
        .from(BOARD_MEMBER_ADDRESS)
        .to(MULTISIG_ADDRESS)
        .whitebox(multisig::contract_obj, |sc| {
            let _ = sc.perform_action_endpoint(action_id);
        });

    world.check_account(ADDER_ADDRESS).code(FACTORIAL_PATH_EXPR);
}

fn boxed_bytes_vec_to_managed<M: ManagedTypeApi>(
    raw_vec: Vec<BoxedBytes>,
) -> ManagedVec<M, ManagedBuffer<M>> {
    let mut managed = ManagedVec::new();
    for elem in raw_vec {
        managed.push(ManagedBuffer::new_from_bytes(elem.as_slice()));
    }

    managed
}
