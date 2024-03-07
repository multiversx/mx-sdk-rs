use adder::*;
use multiversx_sc::{
    storage::mappers::SingleValue,
    types::{AddressExpr, ScExpr, WithResultNewAddress},
};
use multiversx_sc_scenario::{api::StaticApi, num_bigint::BigUint, scenario_model::*, *};

const ADDER_PATH_EXPR: &str = "mxsc:output/adder.mxsc.json";

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/examples/adder");

    blockchain.register_contract(ADDER_PATH_EXPR, adder::ContractBuilder);
    blockchain
}

#[test]
fn adder_blackbox_with_values() {
    let mut world = world();
    let owner_address = "address:owner";
    let mut adder_contract = ContractInfo::<adder::Proxy<StaticApi>>::new("sc:adder");

    world
        .start_trace()
        .set_state_step(
            SetStateStep::new()
                .put_account(owner_address, Account::new().nonce(1))
                .new_address(owner_address, 1, "sc:adder"),
        )
        .tx(|tx| {
            tx.from(AddressExpr("owner"))
                .typed_v2(temp_proxy_v2::TxProxy)
                .init(5u32)
                .code(MxscExpr("output/adder.mxsc.json"))
                .with_result(WithResultNewAddress::new(|new_address| {
                    assert_eq!(new_address.to_address(), adder_contract.to_address());
                }))
        })
        .sc_query(
            ScQueryStep::new()
                .to(&adder_contract)
                .call(adder_contract.sum())
                .expect_value(SingleValue::from(BigUint::from(5u32))),
        )
        .tx(|tx| {
            tx.from(AddressExpr("owner"))
                .to(ScExpr("adder"))
                .typed_v1(temp_proxy::TxProxy, |p| p.add(2u32))
                .with_result(WithRawTxResponse(|response| {
                    assert!(response.tx_error.is_success());
                }))
        })
        .tx(|tx| {
            tx.from(AddressExpr("owner"))
                .to(ScExpr("adder"))
                .typed_v2(temp_proxy_v2::TxProxy)
                .add(1u32)
                .with_result(WithRawTxResponse(|response| {
                    assert!(response.tx_error.is_success());
                }))
        })
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
