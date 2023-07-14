use adder::*;
use multiversx_sc::storage::mappers::SingleValue;
use multiversx_sc_scenario::{api::StaticApi, num_bigint::BigUint, scenario_model::*, *};

const ADDER_PATH_EXPR: &str = "file:output/adder.wasm";

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/examples/adder");

    blockchain.register_contract("file:output/adder.wasm", adder::ContractBuilder);
    blockchain
}

#[test]
fn adder_blackbox_with_values() {
    let mut world = world();
    let owner_address = "address:owner";
    let mut adder_contract = ContractInfo::<adder::Proxy<StaticApi>>::new("sc:adder");
    let adder_code = world.code_expression(ADDER_PATH_EXPR);

    world
        .start_trace()
        .set_state_step(
            SetStateStep::new()
                .put_account(owner_address, Account::new().nonce(1))
                .new_address(owner_address, 1, "sc:adder"),
        )
        .sc_deploy_use_result(
            ScDeployStep::new()
                .from(owner_address)
                .code(adder_code)
                .call(adder_contract.init(5u32)),
            |new_address, _: TypedResponse<()>| {
                assert_eq!(new_address, adder_contract.to_address());
            },
        )
        .sc_query(
            ScQueryStep::new()
                .to(&adder_contract)
                .call(adder_contract.sum())
                .expect_value(SingleValue::from(BigUint::from(5u32))),
        )
        .sc_call(
            ScCallStep::new()
                .from(owner_address)
                .to(&adder_contract)
                .call(adder_contract.add(3u32)),
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
