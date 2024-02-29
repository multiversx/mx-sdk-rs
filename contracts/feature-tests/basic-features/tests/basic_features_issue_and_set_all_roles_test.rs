use multiversx_sc_scenario::{scenario_model::*, *};

const BASIC_FEATURES_PATH_EXPR: &str = "mxsc:output/basic-features.mxsc.json";

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/feature-tests/basic-features");

    blockchain.register_contract(
        "mxsc:output/basic-features.mxsc.json",
        basic_features::ContractBuilder,
    );
    blockchain
}

#[test]
fn basic_features_blackbox() {
    let mut world = world();
    let basic_features_code = world.code_expression(BASIC_FEATURES_PATH_EXPR);

    world
        .set_state_step(
            SetStateStep::new()
                .put_account(
                    "address:owner",
                    Account::new().nonce(1).balance("100,000"),
                )
                .new_address("address:owner", 1, "sc:basic_features"),
        )
        .sc_deploy(
            ScDeployStep::new()
                .from("address:owner")
                .code(basic_features_code)
                .expect(TxExpect::ok().no_result()),
        )
        .sc_call(
            ScCallStep::new()
                .from("address:owner")
                .to("sc:basic_features")
                .egld_value(BigUintValue::from(50_000_000_000_000_000u64)) // should be 0.05 egld
                .function("issue_and_set_all_roles_fungible")
                .argument(
                    "str:TICKER",
                )
                .expect(TxExpect::ok().no_result()),
        )
        .sc_query(
            ScQueryStep::new()
                .to("sc:basic_features")
                .function("getTokenMapperState")
                .expect(TxExpect::ok().result(
                    "1",
                )),
        )
        // .sc_query(
        //     ScQueryStep::new()
        //         .to("sc:basic_features")
        //         .function("getFungibleTokenId")
        //         .expect(TxExpect::ok().result(
        //             "str:TICKER",
        //         )),
        // )
        ;
}
