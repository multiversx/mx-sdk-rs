use managed_map_features::*;
use multiversx_sc_scenario::imports::*;

const MANAGED_MAP_CODE_PATH: MxscPath = MxscPath::new("output/managed-map-features.mxsc.json");
const OWNER_ADDRESS: TestAddress = TestAddress::new("owner");
const SC_ADDRESS: TestSCAddress = TestSCAddress::new("managed-map-features");

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new().executor_config(ExecutorConfig::full_suite());

    blockchain.set_current_dir_from_workspace("contracts/feature-tests/managed-map-features");
    blockchain.register_contract(MANAGED_MAP_CODE_PATH, managed_map_features::ContractBuilder);

    blockchain
        .account(OWNER_ADDRESS)
        .nonce(1)
        .balance(100)
        .commit();

    blockchain
        .account(SC_ADDRESS)
        .nonce(1)
        .balance(100)
        .owner(OWNER_ADDRESS)
        .code(MANAGED_MAP_CODE_PATH)
        .storage_mandos("str:num_entries", "2")
        .storage_mandos("str:key|u32:0", "str:key0")
        .storage_mandos("str:key|u32:1", "str:key1")
        .storage_mandos("str:key|u32:2", "str:key2")
        .storage_mandos("str:value|u32:0", "str:value0")
        .storage_mandos("str:value|u32:1", "str:value1")
        .storage_mandos("str:value|u32:2", "str:value2")
        .commit();

    blockchain
}

#[test]
fn key_mutability_test() {
    let mut world = world();

    let mut key = "key1".to_string();

    let result = world
        .tx()
        .from(OWNER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(managed_map_features_proxy::ManagedMapFeaturesProxy)
        .mm_get(&key)
        .returns(ReturnsResult)
        .run();

    assert_eq!(result, ManagedBuffer::from(b"value1"));

    key.push('y');

    let result = world
        .tx()
        .from(OWNER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(managed_map_features_proxy::ManagedMapFeaturesProxy)
        .mm_get(&key)
        .returns(ReturnsResult)
        .run();

    assert!(result.is_empty());

    let result = world
        .tx()
        .from(OWNER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(managed_map_features_proxy::ManagedMapFeaturesProxy)
        .mm_contains(&key)
        .returns(ReturnsResult)
        .run();

    assert!(!result);

    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(managed_map_features_proxy::ManagedMapFeaturesProxy)
        .mm_remove(&key)
        .returns(ReturnsResult)
        .run();

    key = key[..key.len() - 1].to_string();

    let result = world
        .tx()
        .from(OWNER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(managed_map_features_proxy::ManagedMapFeaturesProxy)
        .mm_get(&key)
        .returns(ReturnsResult)
        .run();

    assert_eq!(result, ManagedBuffer::from(b"value1"));
}
