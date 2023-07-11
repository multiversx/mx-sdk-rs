use empty::*;
use multiversx_sc_scenario::{api::StaticApi, scenario_model::*, *};

const CWD: &str = "contracts/examples/empty";
const EMPTY_WASM: &str = "file:output/empty.wasm";
const OWNER_ADDRESS: &str = "address:owner";
const EMPTY_SC_ADDRESS: &str = "sc:empty";
const GAS_LIMIT: &str = "5,000,000";

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace(CWD);

    blockchain.register_contract(EMPTY_WASM, empty::ContractBuilder);
    blockchain
}

#[test]
fn empty_blackbox_init_raw() {
    let mut world = world();
    let ic = world.interpreter_context();

    world
        .set_state_step(
            SetStateStep::new()
                .put_account(OWNER_ADDRESS, Account::new().nonce(1))
                .new_address(OWNER_ADDRESS, 1, EMPTY_SC_ADDRESS),
        )
        .sc_deploy_step(
            ScDeployStep::new()
                .from(OWNER_ADDRESS)
                .contract_code(EMPTY_WASM, &ic)
                .gas_limit(GAS_LIMIT)
                .expect(TxExpect::ok().no_result()),
        )
        .check_state_step(
            CheckStateStep::new()
                .put_account(OWNER_ADDRESS, CheckAccount::new())
                .put_account(EMPTY_SC_ADDRESS, CheckAccount::new()),
        );
}

#[test]
fn empty_blackbox_init_call() {
    let mut world = world();
    let ic = world.interpreter_context();

    let owner_address = OWNER_ADDRESS;
    let mut empty_contract = ContractInfo::<empty::Proxy<StaticApi>>::new(EMPTY_SC_ADDRESS);

    world
        .start_trace()
        .set_state_step(
            SetStateStep::new()
                .put_account(owner_address, Account::new().nonce(1))
                .new_address(owner_address, 1, &empty_contract),
        )
        .sc_deploy_step(
            ScDeployStep::new()
                .from(owner_address)
                .contract_code(EMPTY_WASM, &ic)
                .call(empty_contract.init())
                .gas_limit(GAS_LIMIT)
                .expect(TxExpect::ok().no_result()),
        )
        .check_state_step(
            CheckStateStep::new()
                .put_account(owner_address, CheckAccount::new())
                .put_account(&empty_contract, CheckAccount::new()),
        )
        .write_scenario_trace("empty_blackbox_init_call.scen.json");
}

#[test]
fn empty_blackbox_init_result() {
    let mut world = world();
    let ic = world.interpreter_context();

    let owner_address = OWNER_ADDRESS;
    let mut empty_contract = ContractInfo::<empty::Proxy<StaticApi>>::new(EMPTY_SC_ADDRESS);

    world.start_trace().set_state_step(
        SetStateStep::new()
            .put_account(owner_address, Account::new().nonce(1))
            .new_address(owner_address, 1, &empty_contract),
    );

    let (new_address, ()) = empty_contract
        .init()
        .into_blockchain_call()
        .from(owner_address)
        .contract_code(EMPTY_WASM, &ic)
        .gas_limit(GAS_LIMIT)
        .expect(TxExpect::ok().no_result())
        .execute(&mut world);
    assert_eq!(new_address, empty_contract.to_address());

    world
        .check_state_step(
            CheckStateStep::new()
                .put_account(owner_address, CheckAccount::new())
                .put_account(&empty_contract, CheckAccount::new()),
        )
        .write_scenario_trace("empty_blackbox_init_result.scen.json");
}
