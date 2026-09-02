use multiversx_sc_scenario::imports::*;
use very_large_storage::very_large_storage_proxy;

const WASM_PATH: MxscPath = MxscPath::new("output/very-large-storage.mxsc.json");
const VLS_ADDRESS: TestSCAddress = TestSCAddress::new("vls");
const OWNER_ADDRESS: TestAddress = TestAddress::new("owner");

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/benchmarks/very-large-storage");

    blockchain.register_contract(WASM_PATH, very_large_storage::ContractBuilder);
    blockchain
}

fn setup() -> ScenarioWorld {
    let mut world = world();
    world
        .account(OWNER_ADDRESS)
        .nonce(1)
        .new_address(OWNER_ADDRESS, 1, VLS_ADDRESS);

    world
        .tx()
        .from(OWNER_ADDRESS)
        .typed(very_large_storage_proxy::VeryLargeStorageProxy)
        .init()
        .code(WASM_PATH)
        .run();
    world
}

#[test]
fn vls_deploy_blackbox() {
    let mut world = setup();

    world
        .query()
        .to(VLS_ADDRESS)
        .typed(very_large_storage_proxy::VeryLargeStorageProxy)
        .x_len()
        .returns(ExpectValue(0usize))
        .run();
}

#[test]
fn vls_append_blackbox() {
    let mut world = setup();

    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(VLS_ADDRESS)
        .typed(very_large_storage_proxy::VeryLargeStorageProxy)
        .append(100u64)
        .run();

    world
        .query()
        .to(VLS_ADDRESS)
        .typed(very_large_storage_proxy::VeryLargeStorageProxy)
        .x_len()
        .returns(ExpectValue(100usize))
        .run();
}

#[test]
fn vls_append_multiple_blackbox() {
    let mut world = setup();

    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(VLS_ADDRESS)
        .typed(very_large_storage_proxy::VeryLargeStorageProxy)
        .append(256u64)
        .run();

    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(VLS_ADDRESS)
        .typed(very_large_storage_proxy::VeryLargeStorageProxy)
        .append(256u64)
        .run();

    world
        .query()
        .to(VLS_ADDRESS)
        .typed(very_large_storage_proxy::VeryLargeStorageProxy)
        .x_len()
        .returns(ExpectValue(512usize))
        .run();
}

#[test]
fn vls_append_cycles_blackbox() {
    let mut world = setup();

    // Append exactly one full cycle (256 bytes)
    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(VLS_ADDRESS)
        .typed(very_large_storage_proxy::VeryLargeStorageProxy)
        .append(256u64)
        .run();

    world
        .query()
        .to(VLS_ADDRESS)
        .typed(very_large_storage_proxy::VeryLargeStorageProxy)
        .x_len()
        .returns(ExpectValue(256usize))
        .run();

    // Check stored bytes are 0x00..=0xff
    let stored = world
        .query()
        .to(VLS_ADDRESS)
        .typed(very_large_storage_proxy::VeryLargeStorageProxy)
        .get_x()
        .returns(ReturnsResult)
        .run();

    let bytes = stored.to_vec();
    assert_eq!(bytes.len(), 256);
    for (i, b) in bytes.into_iter().enumerate() {
        assert_eq!(b, i as u8, "Byte at index {i} is incorrect");
    }
}
