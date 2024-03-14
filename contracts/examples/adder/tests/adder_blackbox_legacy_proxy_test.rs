use adder::*;
use multiversx_sc::{
    storage::mappers::SingleValue,
    types::{AddressExpr, ReturnsSimilar, WithResultNewAddress},
};
use multiversx_sc_scenario::{api::StaticApi, num_bigint::BigUint, scenario_model::*, *};

const ADDER_PATH_EXPR: &str = "mxsc:output/adder.mxsc.json";

const OWNER: AddressExpr = AddressExpr("owner");
const CODE_EXPR: MxscExpr = MxscExpr("output/adder.mxsc.json");

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/examples/adder");

    blockchain.register_contract(ADDER_PATH_EXPR, adder::ContractBuilder);
    blockchain
}

#[test]
fn adder_blackbox_legacy_proxy() {
    let mut world = world();
    let owner_address = "address:owner";
    let mut adder_contract = ContractInfo::<adder::Proxy<StaticApi>>::new("sc:adder");

    world.start_trace();

    world.set_state_step(
        SetStateStep::new()
            .put_account(owner_address, Account::new().nonce(1))
            .new_address(owner_address, 1, "sc:adder"),
    );

    world
        .tx()
        .from(OWNER)
        .typed(temp_proxy::AdderProxy)
        .init(5u32)
        .code(CODE_EXPR)
        .with_result(WithResultNewAddress::new(|new_address| {
            assert_eq!(new_address.to_address(), adder_contract.to_address());
        }))
        .run();

    world.sc_query(
        ScQueryStep::new()
            .to(&adder_contract)
            .call(adder_contract.sum())
            .expect_value(SingleValue::from(BigUint::from(5u32))),
    );

    let value = world
        .query()
        .call(adder_contract.sum())
        .returns(ReturnsSimilar::<SingleValue<BigUint>>::new())
        .run();
    assert_eq!(value.into(), BigUint::from(5u32));

    world
        .tx()
        .from(OWNER)
        .call(adder_contract.add(3u32))
        .with_result(WithRawTxResponse(|response| {
            assert!(response.tx_error.is_success());
        }))
        .run();

    world.check_state_step(
        CheckStateStep::new()
            .put_account(owner_address, CheckAccount::new())
            .put_account(
                &adder_contract,
                CheckAccount::new().check_storage("str:sum", "8"),
            ),
    );

    world.write_scenario_trace("trace1.scen.json");
}
