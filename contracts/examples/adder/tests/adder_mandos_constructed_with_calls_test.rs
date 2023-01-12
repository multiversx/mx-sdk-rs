use adder::*;
use multiversx_sc::storage::mappers::SingleValue;
use multiversx_sc_scenario::{num_bigint::BigUint, scenario_model::*, *};

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/examples/adder");

    blockchain.register_contract("file:output/adder.wasm", adder::ContractBuilder);
    blockchain
}

#[test]
fn adder_scenario_constructed_raw() {
    let _ = DebugApi::dummy();
    let mut world = world();
    let ic = world.interpreter_context();
    let owner_address = "address:owner";
    let mut adder_contract = ContractInfo::<adder::Proxy<DebugApi>>::new("sc:adder");

    world
        .set_state_step(
            SetStateStep::new()
                .put_account(owner_address, Account::new().nonce(1))
                .new_address(owner_address, 1, "sc:adder"),
        )
        .sc_deploy_step(
            ScDeployStep::new()
                .from(owner_address)
                .contract_code("file:output/adder.wasm", &ic)
                .call(adder_contract.init(5u32))
                .gas_limit("5,000,000")
                .expect(TxExpect::ok().no_result()),
        )
        .sc_query_step(
            ScQueryStep::new()
                .to(&adder_contract)
                .call_expect(adder_contract.sum(), SingleValue::from(BigUint::from(5u32))),
        )
        .sc_call_step(
            ScCallStep::new()
                .from(owner_address)
                .to(&adder_contract)
                .call(adder_contract.add(3u32))
                .expect(TxExpect::ok().no_result()),
        )
        .check_state_step(
            CheckStateStep::new()
                .put_account(owner_address, CheckAccount::new())
                .put_account(
                    &adder_contract,
                    CheckAccount::new().check_storage("str:sum", "8"),
                ),
        )
        .write_scenario_trace("trace1.scen.json");
}
