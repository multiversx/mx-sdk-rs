use multiversx_sc_scenario::{imports::*, scenario_model::U64Value};
use num_bigint::BigUint;

use adder::*;

const ADDER_PATH_EXPR: &str = "mxsc:output/adder.mxsc.json";

const OWNER: AddressExpr = AddressExpr("owner");
const SC_ADDER: ScExpr = ScExpr("adder");
const CODE_EXPR: MxscExpr = MxscExpr("output/adder.mxsc.json");

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/examples/adder");

    blockchain.register_contract(ADDER_PATH_EXPR, adder::ContractBuilder);
    blockchain
}

#[test]
fn adder_blackbox() {
    let mut world = world();
    let owner_address = "address:owner";
    let other_address = "address:other";

    let adder_contract = ContractInfo::<adder::Proxy<StaticApi>>::new("sc:adder");

    world.start_trace();

    // world.set_state_step(
    //     SetStateStep::new()
    //         .put_account(owner_address, Account::new().nonce(1))
    //         .new_address(owner_address, 1, "sc:adder"),
    // );

    world
        .set_state()
        .account(owner_address)
        .nonce(1)
        .balance("100")
        .new_address(owner_address, 1, "sc:adder")
        .account(other_address)
        .nonce(2)
        .balance("300")
        .esdt_balance("str:TOKEN-123456", "500")
        .commit();

    world.check_state_step(
        CheckStateStep::new()
            .put_account(
                owner_address,
                CheckAccount::new()
                    .nonce(U64Value::from(1u64))
                    .balance("100"),
            )
            .put_account(
                other_address,
                CheckAccount::new()
                    .nonce(U64Value::from(2u64))
                    .balance("300")
                    .esdt_balance("str:TOKEN-123456", "500"),
            ),
    );

    let new_address = world
        .tx()
        .from(OWNER)
        .typed(adder_proxy::AdderProxy)
        .init(5u32)
        .code(CODE_EXPR)
        .returns(ReturnsNewAddress)
        .run();

    assert_eq!(new_address, adder_contract.to_address());

    let value = world
        .query()
        .to(SC_ADDER)
        .typed(adder_proxy::AdderProxy)
        .sum()
        .returns(ReturnsResultConv::<BigUint>::new())
        .run();
    assert_eq!(value, BigUint::from(5u32));

    world
        .tx()
        .from(OWNER)
        .to(SC_ADDER)
        .typed(adder_proxy::AdderProxy)
        .add(1u32)
        .run();

    world.check_state_step(
        CheckStateStep::new()
            .put_account(owner_address, CheckAccount::new())
            .put_account(
                &adder_contract,
                CheckAccount::new().check_storage("str:sum", "6"),
            ),
    );

    world.write_scenario_trace("trace1.scen.json");
}
