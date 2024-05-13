use multiversx_sc_scenario::imports::*;
use rust_testing_framework_tester::*;

const WASM_PATH_EXPR: &str = "mxsc:output/rust-testing-framework-tester.mxsc.json";

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain
        .set_current_dir_from_workspace("contracts/feature_tests/rust-testing-framework-tester");

    blockchain.register_contract(
        WASM_PATH_EXPR,
        rust_testing_framework_tester::ContractBuilder,
    );
    blockchain
}

#[test]
#[allow(deprecated)]
fn tester_deploy_test() {
    let mut world = world();
    let code = world.code_expression(WASM_PATH_EXPR);

    let owner_address = "address:owner";
    let mut adder_contract =
        ContractInfo::<rust_testing_framework_tester::Proxy<StaticApi>>::new("sc:contract");

    world
        .start_trace()
        .set_state_step(
            SetStateStep::new()
                .put_account(owner_address, Account::new())
                .new_address(owner_address, 0, &adder_contract),
        )
        .sc_deploy_use_result(
            ScDeployStep::new()
                .from(owner_address)
                .code(code)
                .call(adder_contract.init()),
            |address, tr: TypedResponse<String>| {
                assert_eq!(address, adder_contract.to_address());
                assert_eq!(tr.result.unwrap(), "constructor-result");
            },
        )
        .write_scenario_trace("scenarios/trace-deploy.scen.json");
}

#[test]
#[allow(deprecated)]
fn tester_deploy_test_spawned_thread() {
    let handler = std::thread::spawn(|| {
        let mut world = world();
        let code = world.code_expression(WASM_PATH_EXPR);

        let owner_address = "address:owner";
        let mut adder_contract =
            ContractInfo::<rust_testing_framework_tester::Proxy<StaticApi>>::new("sc:contract");

        world
            .start_trace()
            .set_state_step(
                SetStateStep::new()
                    .put_account(owner_address, Account::new())
                    .new_address(owner_address, 0, &adder_contract),
            )
            .sc_deploy_use_result(
                ScDeployStep::new()
                    .from(owner_address)
                    .code(code)
                    .call(adder_contract.init()),
                |address, tr: TypedResponse<String>| {
                    assert_eq!(address, adder_contract.to_address());
                    assert_eq!(tr.result.unwrap(), "constructor-result");
                },
            )
            .write_scenario_trace("scenarios/trace-deploy.scen.json");
    });

    handler.join().unwrap();
}
