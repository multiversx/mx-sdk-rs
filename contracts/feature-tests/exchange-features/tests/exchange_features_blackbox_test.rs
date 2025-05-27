use multiversx_sc_scenario::imports::*;

const EXCHANGE_FEATURES_PATH_EXPR: &str = "mxsc:output/exchange-features.mxsc.json";

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.set_current_dir_from_workspace("contracts/feature-tests/exchange-features");
    blockchain.register_contract(
        EXCHANGE_FEATURES_PATH_EXPR,
        exchange_features::ContractBuilder,
    );
    blockchain
}

#[test]
fn exchange_features_blackbox_raw() {
    let mut world = world();
    let exchange_features_code = world.code_expression(EXCHANGE_FEATURES_PATH_EXPR);

    world
        .set_state_step(
            SetStateStep::new()
                .put_account("address:owner", Account::new().nonce(1))
                .new_address("address:owner", 1, "sc:exchange-features"),
        )
        .sc_deploy(
            ScDeployStep::new()
                .from("address:owner")
                .code(&exchange_features_code)
                .argument("5")
                .expect(TxExpect::ok().no_result()),
        )
        .sc_call(
            ScCallStep::new()
                .from("address:owner")
                .to("sc:exchange-features")
                .function("get_supply")
                .expect(TxExpect::ok().result("5")),
        )
        .sc_call(
            ScCallStep::new()
                .from("address:owner")
                .to("sc:exchange-features")
                .function("merge")
                .argument("3")
                .expect(TxExpect::ok().no_result()),
        )
        .sc_call(
            ScCallStep::new()
                .from("address:owner")
                .to("sc:exchange-features")
                .function("get_supply")
                .expect(TxExpect::ok().result("8")),
        )
        .sc_call(
            ScCallStep::new()
                .from("address:owner")
                .to("sc:exchange-features")
                .function("upgradeContract")
                .argument(&exchange_features_code)
                .argument("0x0502") // codeMetadata
                .argument("0") // contract argument
                .expect(TxExpect::user_error("str:Zero amount")),
        )
        .sc_call(
            ScCallStep::new()
                .from("address:owner")
                .to("sc:exchange-features")
                .function("upgradeContract")
                .argument(exchange_features_code)
                .argument("0x0502") // codeMetadata
                .argument("3") // contract argument
                .expect(TxExpect::ok().no_result()),
        )
        .sc_call(
            ScCallStep::new()
                .from("address:owner")
                .to("sc:exchange-features")
                .function("get_supply")
                .expect(TxExpect::ok().result("3")),
        );
}
