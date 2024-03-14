use adder::*;
use multiversx_sc::types::{AddressExpr, ScExpr, WithResultNewAddress, WithResultSimilar};
use multiversx_sc_scenario::{api::StaticApi, num_bigint::BigUint, scenario_model::*, *};

const ADDER_PATH_EXPR: &str = "mxsc:output/adder.mxsc.json";

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/examples/adder");

    blockchain.register_contract(ADDER_PATH_EXPR, adder::ContractBuilder);
    blockchain
}

#[test]
fn adder_blackbox_chained() {
    let mut world = world();
    let owner_address = "address:owner";
    let adder_contract = ContractInfo::<adder::Proxy<StaticApi>>::new("sc:adder");

    world
        .start_trace()
        .set_state_step(
            SetStateStep::new()
                .put_account(owner_address, Account::new().nonce(1))
                .new_address(owner_address, 1, "sc:adder"),
        )
        .chain_deploy(|tx| {
            tx.from(AddressExpr("owner"))
                .typed(temp_proxy::AdderProxy)
                .init(5u32)
                .code(MxscExpr("output/adder.mxsc.json"))
                .with_result(WithResultNewAddress::new(|new_address| {
                    assert_eq!(new_address.to_address(), adder_contract.to_address());
                }))
        })
        .chain_query(|tx| {
            tx.to(ScExpr("adder"))
                .typed(temp_proxy::AdderProxy)
                .sum()
                .with_result(WithResultSimilar::new(|value: BigUint| {
                    assert_eq!(value, BigUint::from(5u32));
                }))
        })
        .chain_call(|tx| {
            tx.from(AddressExpr("owner"))
                .to(ScExpr("adder"))
                .typed(temp_proxy::AdderProxy)
                .add(3u32)
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
        .write_scenario_trace("trace2.scen.json");
}
