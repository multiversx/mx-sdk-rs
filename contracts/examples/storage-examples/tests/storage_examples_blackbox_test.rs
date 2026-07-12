// tests/storage_mappers_blackbox_test.rs — one test per mapper this
// recipe covers, each proving the specific behavioral claim its README
// entry makes (not just "it compiles").

use multiversx_sc_scenario::imports::*;

use storage_examples::proxy;

const OWNER: TestAddress = TestAddress::new("owner");
const CONTRACT: TestSCAddress = TestSCAddress::new("storage-examples-contract");
const CODE_PATH: MxscPath = MxscPath::new("output/storage-examples.mxsc.json");

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.register_contract(CODE_PATH, storage_examples::ContractBuilder);
    blockchain
}

fn deploy(world: &mut ScenarioWorld) {
    world.account(OWNER).nonce(1);
    world
        .tx()
        .from(OWNER)
        .typed(proxy::StorageMappersProxy)
        .init()
        .code(CODE_PATH)
        .new_address(CONTRACT)
        .run();
}

#[test]
fn single_value_mapper_set_and_get() {
    let mut world = world();
    deploy(&mut world);

    world
        .tx()
        .from(OWNER)
        .to(CONTRACT)
        .typed(proxy::StorageMappersProxy)
        .set_counter(BigUint::<StaticApi>::from(42u64))
        .run();

    world
        .query()
        .to(CONTRACT)
        .typed(proxy::StorageMappersProxy)
        .counter()
        .returns(ExpectValue(BigUint::<StaticApi>::from(42u64)))
        .run();
}

#[test]
fn single_value_mapper_overwrite() {
    let mut world = world();
    deploy(&mut world);

    world
        .tx()
        .from(OWNER)
        .to(CONTRACT)
        .typed(proxy::StorageMappersProxy)
        .set_counter(BigUint::<StaticApi>::from(10u64))
        .run();

    // A second set must overwrite the first — not accumulate.
    world
        .tx()
        .from(OWNER)
        .to(CONTRACT)
        .typed(proxy::StorageMappersProxy)
        .set_counter(BigUint::<StaticApi>::from(99u64))
        .run();

    world
        .query()
        .to(CONTRACT)
        .typed(proxy::StorageMappersProxy)
        .counter()
        .returns(ExpectValue(BigUint::<StaticApi>::from(99u64)))
        .run();
}

#[test]
fn vec_mapper_is_one_indexed() {
    let mut world = world();
    deploy(&mut world);

    // Push returns the 1-based index of the newly appended item.
    world
        .tx()
        .from(OWNER)
        .to(CONTRACT)
        .typed(proxy::StorageMappersProxy)
        .push_item(ManagedBuffer::<StaticApi>::from(b"first"))
        .returns(ExpectValue(1usize))
        .run();
    world
        .tx()
        .from(OWNER)
        .to(CONTRACT)
        .typed(proxy::StorageMappersProxy)
        .push_item(ManagedBuffer::<StaticApi>::from(b"second"))
        .returns(ExpectValue(2usize))
        .run();

    world
        .query()
        .to(CONTRACT)
        .typed(proxy::StorageMappersProxy)
        .item_count()
        .returns(ExpectValue(2usize))
        .run();

    world
        .query()
        .to(CONTRACT)
        .typed(proxy::StorageMappersProxy)
        .get_item(1usize)
        .returns(ExpectValue(ManagedBuffer::<StaticApi>::from(b"first")))
        .run();

    world
        .query()
        .to(CONTRACT)
        .typed(proxy::StorageMappersProxy)
        .get_item(2usize)
        .returns(ExpectValue(ManagedBuffer::<StaticApi>::from(b"second")))
        .run();
}

#[test]
fn set_mapper_contains_and_ordering() {
    let mut world = world();
    deploy(&mut world);

    for value in [30u64, 10u64, 20u64] {
        world
            .tx()
            .from(OWNER)
            .to(CONTRACT)
            .typed(proxy::StorageMappersProxy)
            .add_to_ordered_set(value)
            // A fresh insert must return true.
            .returns(ExpectValue(true))
            .run();
    }

    world
        .query()
        .to(CONTRACT)
        .typed(proxy::StorageMappersProxy)
        .ordered_set_contains(10u64)
        .returns(ExpectValue(true))
        .run();

    world
        .query()
        .to(CONTRACT)
        .typed(proxy::StorageMappersProxy)
        .ordered_set_contains(99u64)
        .returns(ExpectValue(false))
        .run();

    world
        .query()
        .to(CONTRACT)
        .typed(proxy::StorageMappersProxy)
        .ordered_set_len()
        .returns(ExpectValue(3usize))
        .run();

    // Inserting a duplicate must return false and leave the length unchanged.
    world
        .tx()
        .from(OWNER)
        .to(CONTRACT)
        .typed(proxy::StorageMappersProxy)
        .add_to_ordered_set(10u64)
        .returns(ExpectValue(false))
        .run();

    world
        .query()
        .to(CONTRACT)
        .typed(proxy::StorageMappersProxy)
        .ordered_set_len()
        .returns(ExpectValue(3usize))
        .run();
}

#[test]
fn unordered_set_mapper_contains_after_insert_and_absent_value() {
    let mut world = world();
    deploy(&mut world);

    world
        .tx()
        .from(OWNER)
        .to(CONTRACT)
        .typed(proxy::StorageMappersProxy)
        .add_to_unordered_set(7u64)
        // A fresh insert must return true.
        .returns(ExpectValue(true))
        .run();

    world
        .query()
        .to(CONTRACT)
        .typed(proxy::StorageMappersProxy)
        .unordered_set_contains(7u64)
        .returns(ExpectValue(true))
        .run();

    world
        .query()
        .to(CONTRACT)
        .typed(proxy::StorageMappersProxy)
        .unordered_set_contains(8u64)
        .returns(ExpectValue(false))
        .run();

    world
        .query()
        .to(CONTRACT)
        .typed(proxy::StorageMappersProxy)
        .unordered_set_len()
        .returns(ExpectValue(1usize))
        .run();

    // Inserting a duplicate must return false and leave the length unchanged.
    world
        .tx()
        .from(OWNER)
        .to(CONTRACT)
        .typed(proxy::StorageMappersProxy)
        .add_to_unordered_set(7u64)
        .returns(ExpectValue(false))
        .run();

    world
        .query()
        .to(CONTRACT)
        .typed(proxy::StorageMappersProxy)
        .unordered_set_len()
        .returns(ExpectValue(1usize))
        .run();
}

#[test]
fn whitelist_mapper_membership_only() {
    let mut world = world();
    deploy(&mut world);
    let allowed: TestAddress = TestAddress::new("allowed-user");
    let stranger: TestAddress = TestAddress::new("stranger");
    world.account(allowed).nonce(1);
    world.account(stranger).nonce(1);

    world
        .tx()
        .from(OWNER)
        .to(CONTRACT)
        .typed(proxy::StorageMappersProxy)
        .add_to_whitelist(allowed.to_address())
        .run();

    world
        .query()
        .to(CONTRACT)
        .typed(proxy::StorageMappersProxy)
        .is_whitelisted(allowed.to_address())
        .returns(ExpectValue(true))
        .run();

    world
        .query()
        .to(CONTRACT)
        .typed(proxy::StorageMappersProxy)
        .is_whitelisted(stranger.to_address())
        .returns(ExpectValue(false))
        .run();
}

#[test]
fn map_mapper_insert_get_and_contains_key() {
    let mut world = world();
    deploy(&mut world);
    let holder: TestAddress = TestAddress::new("balance-holder");
    let nobody: TestAddress = TestAddress::new("no-balance");
    world.account(holder).nonce(1);
    world.account(nobody).nonce(1);

    world
        .tx()
        .from(OWNER)
        .to(CONTRACT)
        .typed(proxy::StorageMappersProxy)
        .set_balance(holder.to_address(), BigUint::<StaticApi>::from(500u64))
        .run();

    world
        .query()
        .to(CONTRACT)
        .typed(proxy::StorageMappersProxy)
        .get_balance(holder.to_address())
        .returns(ExpectValue(BigUint::<StaticApi>::from(500u64)))
        .run();

    world
        .query()
        .to(CONTRACT)
        .typed(proxy::StorageMappersProxy)
        .has_balance_entry(holder.to_address())
        .returns(ExpectValue(true))
        .run();

    // A key that was never inserted: contains_key is false, and the
    // convenience getter's unwrap_or_default() reads as zero rather than
    // erroring — a real design choice worth testing explicitly, since it
    // means "balance of zero" and "never had an entry" are
    // indistinguishable through get_balance alone.
    world
        .query()
        .to(CONTRACT)
        .typed(proxy::StorageMappersProxy)
        .has_balance_entry(nobody.to_address())
        .returns(ExpectValue(false))
        .run();

    world
        .query()
        .to(CONTRACT)
        .typed(proxy::StorageMappersProxy)
        .get_balance(nobody.to_address())
        .returns(ExpectValue(BigUint::<StaticApi>::from(0u64)))
        .run();
}
