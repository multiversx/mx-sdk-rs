use basic_features::basic_features_proxy;
use imports::{MxscPath, TestAddress, TestSCAddress};
use multiversx_sc::types::{BigUint, ReturnsResult};
use multiversx_sc_scenario::{api::StaticApi, imports, ScenarioTxRun, ScenarioWorld};

const OWNER_ADDRESS: TestAddress = TestAddress::new("owner");
const BASIC_FEATURES_ADDRESS: TestSCAddress = TestSCAddress::new("basic-features");
const OTHER_BASIC_FEATURES_ADDRESS: TestSCAddress = TestSCAddress::new("other-basic-features");

const BASIC_FEATURES_PATH: MxscPath = MxscPath::new("output/basic-features.mxsc.json");

struct BasicFeaturesState {
    world: ScenarioWorld,
}

impl BasicFeaturesState {
    fn new() -> Self {
        let mut world = world();

        world.start_trace();
        world.account(OWNER_ADDRESS).nonce(1).balance(100);
        world
            .account(BASIC_FEATURES_ADDRESS)
            .nonce(1)
            .code(BASIC_FEATURES_PATH);

        world
            .account(OTHER_BASIC_FEATURES_ADDRESS)
            .nonce(1)
            .code(BASIC_FEATURES_PATH);

        Self { world }
    }

    fn set_initial_value(&mut self, initial_value: BigUint<StaticApi>) {
        self.world
            .tx()
            .from(OWNER_ADDRESS)
            .to(BASIC_FEATURES_ADDRESS)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .timelock_set_initial_value(initial_value)
            .run();
    }

    fn unlock(&mut self) {
        self.world
            .tx()
            .from(OWNER_ADDRESS)
            .to(BASIC_FEATURES_ADDRESS)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .timelock_unlock()
            .run();
    }

    fn set_unlock_timestamp(&mut self, unlock_timestamp: u64, future_value: BigUint<StaticApi>) {
        self.world
            .tx()
            .from(OWNER_ADDRESS)
            .to(BASIC_FEATURES_ADDRESS)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .timelock_set_unlock_timestamp(unlock_timestamp, future_value)
            .run();
    }

    fn commit(&mut self) -> bool {
        self.world
            .tx()
            .from(OWNER_ADDRESS)
            .to(BASIC_FEATURES_ADDRESS)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .timelock_commit_action()
            .returns(ReturnsResult)
            .run()
    }

    fn get_current_value(&mut self) -> BigUint<StaticApi> {
        self.world
            .tx()
            .from(OWNER_ADDRESS)
            .to(BASIC_FEATURES_ADDRESS)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .timelock_mapper()
            .returns(ReturnsResult)
            .run()
    }

    fn get_unlock_timestamp(&mut self) -> u64 {
        self.world
            .tx()
            .from(OWNER_ADDRESS)
            .to(BASIC_FEATURES_ADDRESS)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .timelock_get_unlock_timestamp()
            .returns(ReturnsResult)
            .run()
    }

    fn get_future_value(&mut self) -> BigUint<StaticApi> {
        self.world
            .tx()
            .from(OWNER_ADDRESS)
            .to(BASIC_FEATURES_ADDRESS)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .timelock_get_future_value()
            .returns(ReturnsResult)
            .run()
    }

    fn get_current_value_at_address(&mut self, address: TestSCAddress) -> BigUint<StaticApi> {
        self.world
            .tx()
            .from(OWNER_ADDRESS)
            .to(OTHER_BASIC_FEATURES_ADDRESS)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .timelock_get_current_value_at_address(address.to_managed_address())
            .returns(ReturnsResult)
            .run()
    }

    fn get_unlock_timestamp_at_address(&mut self, address: TestSCAddress) -> u64 {
        self.world
            .tx()
            .from(OWNER_ADDRESS)
            .to(OTHER_BASIC_FEATURES_ADDRESS)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .timelock_get_unlock_timestamp_at_address(address.to_managed_address())
            .returns(ReturnsResult)
            .run()
    }

    fn get_future_value_at_address(&mut self, address: TestSCAddress) -> BigUint<StaticApi> {
        self.world
            .tx()
            .from(OWNER_ADDRESS)
            .to(OTHER_BASIC_FEATURES_ADDRESS)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .timelock_get_future_value_at_address(address.to_managed_address())
            .returns(ReturnsResult)
            .run()
    }

    fn set_env_timestamp(&mut self, new_timestamp: u64) {
        self.world.current_block().block_timestamp(new_timestamp);
    }
}
fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.set_current_dir_from_workspace("contracts/feature-tests/basic-features");
    blockchain.register_contract(BASIC_FEATURES_PATH, basic_features::ContractBuilder);
    blockchain
}

#[test]
fn timelock_mapper_test() {
    let mut state = BasicFeaturesState::new();
    let initial_value = BigUint::from(10u64);
    let future_value = BigUint::from(15u64);

    // start now at 0
    state.set_env_timestamp(0u64);

    state.set_initial_value(initial_value.clone());

    // unlocks for commit at 10
    state.set_unlock_timestamp(10u64, future_value.clone());

    // unlock timestamp and future value are now filled
    assert!(&state.get_future_value() == &future_value);
    assert!(state.get_unlock_timestamp() == 10u64);

    // current value still initial value
    assert!(&state.get_current_value() == &initial_value);

    // move now at 9
    state.set_env_timestamp(9u64);

    // not yet able to commit the value
    assert!(!state.commit());

    // move now at 10
    state.set_env_timestamp(10u64);

    // commit is now possible
    assert!(state.commit());

    // current value is now future value
    assert!(state.get_current_value() == future_value);

    // future value is empty
    assert!(state.get_future_value() == BigUint::zero());

    // reset
    state.set_unlock_timestamp(18u64, initial_value);

    // unlock the mapper, turns into a single value mapper, deletes future and timestamp entries
    state.unlock();

    // future value and unlock timestamp are empty
    assert!(state.get_future_value() == BigUint::zero());
    assert!(state.get_unlock_timestamp() == 0u64);

    state
        .world
        .write_scenario_trace("scenarios/timelock_mapper.scen.json");
}

#[test]
fn timelock_mapper_at_address_test() {
    let mut state = BasicFeaturesState::new();
    let initial_value = BigUint::from(10u64);
    let future_value = BigUint::from(15u64);

    // start now at 0
    state.set_env_timestamp(0u64);

    // setup bf
    state.set_initial_value(initial_value.clone());
    assert!(&state.get_current_value() == &initial_value);

    // unlocks for commit at 10
    state.set_unlock_timestamp(10u64, future_value.clone());

    // check bf values from other-bf
    assert_eq!(
        &state.get_current_value_at_address(BASIC_FEATURES_ADDRESS),
        &initial_value
    );
    assert!(&state.get_future_value_at_address(BASIC_FEATURES_ADDRESS) == &future_value);
    assert!(state.get_unlock_timestamp_at_address(BASIC_FEATURES_ADDRESS) == 10u64);

    // move now to 12
    state.set_env_timestamp(12u64);

    // commit in bf
    state.commit();

    // check bf values from other-bf
    assert!(&state.get_current_value_at_address(BASIC_FEATURES_ADDRESS) == &future_value);
    assert!(state.get_future_value_at_address(BASIC_FEATURES_ADDRESS) == BigUint::zero());
    assert!(state.get_unlock_timestamp_at_address(BASIC_FEATURES_ADDRESS) == 10u64);

    // unlock mapper in bf
    state.unlock();

    // check bf values from other-bf
    assert!(state.get_current_value_at_address(BASIC_FEATURES_ADDRESS) == future_value);
    assert!(state.get_future_value_at_address(BASIC_FEATURES_ADDRESS) == BigUint::zero());
    assert!(state.get_unlock_timestamp_at_address(BASIC_FEATURES_ADDRESS) == 0u64);

    state
        .world
        .write_scenario_trace("scenarios/timelock_mapper_at_address.scen.json");
}
