use multiversx_sc_scenario::imports::*;

use gas_tests::factorial_proxy;

const OWNER_ADDRESS: TestAddress = TestAddress::new("owner");
const SC_ADDRESS: TestSCAddress = TestSCAddress::new("factorial");
const CODE_PATH: MxscPath = MxscPath::new("factorial.mxsc.json");

#[allow(unused)]
fn factorial_gas_test(mut world: ScenarioWorld) {
    world.account(OWNER_ADDRESS).nonce(1);

    let (new_address, gas_used) = world
        .tx()
        .from(OWNER_ADDRESS)
        .gas(100)
        .typed(factorial_proxy::FactorialProxy)
        .init()
        .code(CODE_PATH)
        .new_address(SC_ADDRESS)
        .returns(ReturnsNewAddress)
        .returns(ReturnsGasUsed)
        .run();

    assert_eq!(gas_used, 45);
    assert_eq!(new_address, SC_ADDRESS.to_address());
}

#[test]
#[cfg_attr(not(feature = "wasmer-experimental"), ignore)]
fn factorial_gas_wasmer_experimental() {
    let world = ScenarioWorld::new()
        .executor_config(ScenarioExecutorConfig::Experimental)
        .gas_schedule(GasScheduleVersion::V8);
    factorial_gas_test(world);
}

#[test]
#[cfg_attr(not(feature = "wasmer-prod"), ignore)]
fn factorial_gas_wasmer_prod() {
    let world = ScenarioWorld::new()
        .executor_config(ScenarioExecutorConfig::WasmerProd)
        .gas_schedule(GasScheduleVersion::V8);
    factorial_gas_test(world);
}
