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
        .gas(2500)
        .typed(factorial_proxy::FactorialProxy)
        .init()
        .code(CODE_PATH)
        .new_address(SC_ADDRESS)
        .returns(ReturnsNewAddress)
        .returns(ReturnsGasUsed)
        .run();

    assert_eq!(gas_used, 2045);
    assert_eq!(new_address, SC_ADDRESS.to_address());
}

#[allow(unused)]
fn factorial_user_error(mut world: ScenarioWorld) {
    world.account(OWNER_ADDRESS).nonce(1);

    world
        .tx()
        .from(OWNER_ADDRESS)
        .gas(30000)
        .raw_deploy()
        .argument(&0)
        .code(CODE_PATH)
        .returns(ExpectError(4, "wrong number of arguments"))
        .run();
}

#[allow(unused)]
fn factorial_out_of_gas(mut world: ScenarioWorld) {
    world.account(OWNER_ADDRESS).nonce(1);

    world
        .tx()
        .from(OWNER_ADDRESS)
        .gas(100)
        .typed(factorial_proxy::FactorialProxy)
        .init()
        .code(CODE_PATH)
        .new_address(SC_ADDRESS)
        .returns(ExpectError(5, "not enough gas"))
        .run();
}

#[test]
#[cfg_attr(not(feature = "wasmer-experimental"), ignore)]
fn factorial_gas_wasmer_experimental() {
    let world = ScenarioWorld::new()
        .executor_config(ExecutorConfig::Experimental)
        .gas_schedule(GasScheduleVersion::V8);
    factorial_gas_test(world);
}

#[test]
#[cfg_attr(not(feature = "wasmer-experimental"), ignore)]
fn factorial_user_error_wasmer_experimental() {
    let world = ScenarioWorld::new()
        .executor_config(ExecutorConfig::Experimental)
        .gas_schedule(GasScheduleVersion::V8);
    factorial_user_error(world);
}

#[test]
#[cfg_attr(not(feature = "wasmer-experimental"), ignore)]
fn factorial_out_of_gas_wasmer_experimental() {
    let world = ScenarioWorld::new()
        .executor_config(ExecutorConfig::Experimental)
        .gas_schedule(GasScheduleVersion::V8);
    factorial_out_of_gas(world);
}

#[test]
#[cfg(feature = "wasmer-prod")]
fn factorial_gas_wasmer_prod() {
    let world = ScenarioWorld::new()
        .executor_config(ExecutorConfig::Custom(
            multiversx_chain_vm_wasmer_prod::new_prod_executor,
        ))
        .gas_schedule(GasScheduleVersion::V8);
    factorial_gas_test(world);
}

#[test]
#[cfg(feature = "wasmer-prod")]
fn factorial_user_error_wasmer_prod() {
    let world = ScenarioWorld::new()
        .executor_config(ExecutorConfig::Custom(
            multiversx_chain_vm_wasmer_prod::new_prod_executor,
        ))
        .gas_schedule(GasScheduleVersion::V8);
    factorial_user_error(world);
}

#[test]
#[cfg(feature = "wasmer-prod")]
fn factorial_out_of_gas_wasmer_prod() {
    let world = ScenarioWorld::new()
        .executor_config(ExecutorConfig::Custom(
            multiversx_chain_vm_wasmer_prod::new_prod_executor,
        ))
        .gas_schedule(GasScheduleVersion::V8);
    factorial_out_of_gas(world);
}
