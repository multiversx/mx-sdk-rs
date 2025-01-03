use multiversx_sc_scenario::imports::*;
use rust_testing_framework_tester::*;

const CODE_PATH: MxscPath = MxscPath::new("output/rust-testing-framework-tester.mxsc.json");
const OWNER_ADDRESS: TestAddress = TestAddress::new("owner");
const RUST_TESTING_FRAMEWORK_TESTER_ADDRESS: TestSCAddress =
    TestSCAddress::new("rust-testing-framework-tester");

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain
        .set_current_dir_from_workspace("contracts/feature-tests/rust-testing-framework-tester");
    blockchain.register_contract(CODE_PATH, rust_testing_framework_tester::ContractBuilder);

    blockchain
}

#[test]
fn tester_deploy_test() {
    let mut world = world();

    world.start_trace();

    world.account(OWNER_ADDRESS).new_address(
        OWNER_ADDRESS,
        0,
        RUST_TESTING_FRAMEWORK_TESTER_ADDRESS,
    );

    let (returned_value, contract_address) = world
        .tx()
        .from(OWNER_ADDRESS)
        .typed(rust_testing_framework_tester_proxy::RustTestingFrameworkTesterProxy)
        .init()
        .code(CODE_PATH)
        .returns(ReturnsResult)
        .new_address(RUST_TESTING_FRAMEWORK_TESTER_ADDRESS)
        .returns(ReturnsNewAddress)
        .run();

    assert_eq!(returned_value.to_string(), "constructor-result");
    assert_eq!(contract_address, RUST_TESTING_FRAMEWORK_TESTER_ADDRESS);

    world.write_scenario_trace("scenarios/trace-deploy.scen.json");
}

#[test]
fn tester_deploy_test_spawned_thread() {
    let handler = std::thread::spawn(tester_deploy_test);

    handler.join().unwrap();
}
