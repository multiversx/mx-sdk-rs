use multisig::{multisig_perform::MultisigPerformModule, user_role::UserRole, Multisig};
use multiversx_sc::types::{Address, BoxedBytes, CodeMetadata, ManagedVec};
use multiversx_sc_scenario::{
    managed_address,
    scenario_model::{Account, AddressValue, ScDeployStep, SetStateStep},
    ScenarioWorld, WhiteboxContract,
};

const OWNER_ADDRESS_EXPR: &str = "address:owner";
const PROPOSER_ADDRESS_EXPR: &str = "address:proposer";
const BOARD_MEMBER_ADDRESS_EXPR: &str = "address:board-member";
const MULTISIG_ADDRESS_EXPR: &str = "sc:multisig";
const MULTISIG_PATH_EXPR: &str = "file:output/multisig.wasm";
const QUORUM_SIZE: usize = 1;
const EGLD_TOKEN_ID: &[u8] = b"EGLD";

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
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/examples/multisig");

    blockchain.register_contract(MULTISIG_PATH_EXPR, multisig::ContractBuilder);
    blockchain
}

fn setup() -> ScenarioWorld {
    // setup
    let mut world = world();
    let multisig_whitebox = WhiteboxContract::new(MULTISIG_ADDRESS_EXPR, multisig::contract_obj);
    let multisig_code = world.code_expression(MULTISIG_PATH_EXPR);

    world.set_state_step(
        SetStateStep::new()
            .put_account(OWNER_ADDRESS_EXPR, Account::new().nonce(1))
            .new_address(OWNER_ADDRESS_EXPR, 1, MULTISIG_ADDRESS_EXPR)
            .put_account(
                PROPOSER_ADDRESS_EXPR,
                Account::new().nonce(1).balance(100_000_000u64),
            )
            .put_account(BOARD_MEMBER_ADDRESS_EXPR, Account::new().nonce(1)),
    );

    // init multisig
    world.whitebox_deploy(
        &multisig_whitebox,
        ScDeployStep::new()
            .from(OWNER_ADDRESS_EXPR)
            .code(multisig_code),
        |sc| {
            let mut board_members = ManagedVec::new();
            board_members.push(managed_address!(&address_expr_to_address(
                BOARD_MEMBER_ADDRESS_EXPR
            )));

            sc.init(QUORUM_SIZE, board_members.into());
            sc.change_user_role(
                0,
                managed_address!(&address_expr_to_address(PROPOSER_ADDRESS_EXPR)),
                UserRole::Proposer,
            );
        },
    );

    world
}

#[test]
fn test_init() {
    let mut world = setup();
}

fn address_expr_to_address(address_expr: &str) -> Address {
    AddressValue::from(address_expr).to_address()
}
