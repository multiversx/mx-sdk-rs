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
        .storage_mandos("str:num_entries", "3")
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
fn managed_map_get_set_remove_test() {
    let mut world = world();

    let result = world
        .query()
        .to(SC_ADDRESS)
        .typed(managed_map_features_proxy::ManagedMapFeaturesProxy)
        .mm_get("key1")
        .returns(ReturnsResultUnmanaged)
        .run();

    assert_eq!(result, "value1".as_bytes());

    let result = world
        .query()
        .to(SC_ADDRESS)
        .typed(managed_map_features_proxy::ManagedMapFeaturesProxy)
        .mm_get("unknown-key")
        .returns(ReturnsResult)
        .run();

    assert!(result.is_empty());

    let result = world
        .query()
        .to(SC_ADDRESS)
        .typed(managed_map_features_proxy::ManagedMapFeaturesProxy)
        .mm_contains("unknown-key")
        .returns(ReturnsResult)
        .run();

    assert!(!result);

    let (removed_value, get_value) = world
        .query()
        .to(SC_ADDRESS)
        .typed(managed_map_features_proxy::ManagedMapFeaturesProxy)
        .mm_remove_get("key1", "key1")
        .returns(ReturnsResultUnmanaged)
        .run()
        .into_tuple();

    assert_eq!(removed_value, "value1".as_bytes());
    assert!(get_value.is_empty());

    let (removed_value, get_value) = world
        .query()
        .to(SC_ADDRESS)
        .typed(managed_map_features_proxy::ManagedMapFeaturesProxy)
        .mm_remove_get("unknown-key", "unknown-key")
        .returns(ReturnsResultUnmanaged)
        .run()
        .into_tuple();

    assert!(removed_value.is_empty());
    assert!(get_value.is_empty());
}
