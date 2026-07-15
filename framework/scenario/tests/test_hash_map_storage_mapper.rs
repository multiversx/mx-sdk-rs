use multiversx_sc::storage::{
    StorageKey,
    mappers::{MapMapper, MapStorageMapper, StorageClearable, StorageMapper, VecMapper},
};
use multiversx_sc_scenario::api::SingleTxApi;

fn create_map_storage_custom_key(
    key: &[u8],
) -> MapStorageMapper<SingleTxApi, u64, MapMapper<SingleTxApi, u64, u64>> {
    MapStorageMapper::new(StorageKey::new(key))
}

fn create_map_storage() -> MapStorageMapper<SingleTxApi, u64, MapMapper<SingleTxApi, u64, u64>> {
    create_map_storage_custom_key(b"my_map_storage")
}

#[test]
fn test_map_storage_simple() {
    let mut map = create_map_storage();
    assert_eq!(map.len(), 0);
    assert!(map.insert_default(42));
    assert!(!map.insert_default(42));
    let map42_option = map.get(&42);
    assert!(map42_option.is_some());
    let mut map42 = map42_option.unwrap();
    assert_eq!(map42.insert(100, 111), None);
    assert_eq!(map42.insert(100, 200), Some(111));
    assert_eq!(map42.insert(101, 201), None);
    assert_eq!(map42.len(), 2);
    assert_eq!(map.len(), 1);
    map.insert_default(43);
    assert_eq!(map.len(), 2);
    map.insert_default(44);
    assert_eq!(map.len(), 3);
    assert!(map.contains_key(&42));
    assert!(!map.contains_key(&50));
}

#[test]
fn test_map_storage_remove() {
    let mut map = create_map_storage();
    map.insert_default(42);
    map.insert_default(43);
    assert_eq!(map.len(), 2);
    assert!(map.remove(&42));
    assert!(!map.remove(&42));
    assert_eq!(map.len(), 1);
}

#[test]
fn test_map_storage_clear() {
    let mut map = create_map_storage();
    map.insert_default(42);
    let mut nested_map = map.get(&42).unwrap();
    nested_map.insert(420, 421);
    nested_map.insert(422, 423);
    assert_eq!(nested_map.len(), 2);
    map.clear();
    assert_eq!(nested_map.len(), 0);
    assert_eq!(map.len(), 0);
    assert!(map.is_empty());
}

/// `MapStorageMapper::remove` must fully clear the nested storage namespace.
/// A mapper handle obtained before removal reflects the cleared state because it
/// points to the same deterministic storage prefix.
#[test]
fn test_remove_clears_nested_storage() {
    let mut map = create_map_storage_custom_key(b"remove_clears_test");

    map.insert_default(10);
    let mut nested = map.get(&10).unwrap();
    nested.insert(1, 100);
    nested.insert(2, 200);
    assert_eq!(nested.len(), 2);

    // remove() must clear the nested storage, not just drop the keys_set entry.
    assert!(map.remove(&10));
    assert!(!map.contains_key(&10));

    // The previously obtained handle still points to the same prefix; it should now be empty.
    assert_eq!(nested.len(), 0);
    assert!(nested.is_empty());
}

/// `OccupiedEntry` manipulation via `and_modify` followed by `MapStorageMapper::remove`
/// must fully clear the nested storage.
#[test]
fn test_entry_remove_clears_nested_storage() {
    let mut map = create_map_storage_custom_key(b"entry_remove_clears_test");

    // Use the entry API to insert and populate the nested mapper.
    let occupied = map.entry(20).or_insert_default();
    let mut nested = occupied.get();
    nested.insert(3, 300);
    assert_eq!(nested.len(), 1);

    // Remove via MapStorageMapper::remove (which OccupiedEntry::remove delegates to).
    assert!(map.remove(&20));
    assert!(!map.contains_key(&20));

    // Nested storage must be cleared; the previously obtained handle reflects this.
    assert_eq!(nested.len(), 0);
}

/// `insert_default` must clear any residual nested storage so that a (re-)inserted key
/// always starts with an empty nested mapper.
///
/// This test injects a stale `.len` value directly into the raw storage backend to
/// simulate residual data at the nested `VecMapper` namespace, bypassing the mapper's
/// own write path. `VecMapper` stores its length at `base_key + ".len"` as a
/// top-encoded `usize` (minimal big-endian, so `1` is `[0x01]`).
///
/// Without the fix, `insert_default` would leave that data intact and the nested mapper
/// would report a non-zero length. With the fix, `insert_default` calls `clear()` on
/// the nested mapper before inserting the key, wiping the stale length.
#[test]
fn test_insert_default_clears_residual_nested_storage() {
    // Unique base key for this test to avoid interfering with other tests.
    let base_key = b"stale_state_test_vec";

    // Build the raw storage key for the nested VecMapper's `.len` slot.
    // Layout: <base_key> + b".storage" + NestedEncode(99u64) + b".len"
    // NestedEncode of u64 is 8 big-endian bytes.
    let stale_len_key: Vec<u8> = {
        let mut k = base_key.to_vec();
        k.extend_from_slice(b".storage");
        k.extend_from_slice(&99u64.to_be_bytes());
        k.extend_from_slice(b".len");
        k
    };

    // TopEncode of `1usize` is a single byte `[0x01]` (minimal big-endian).
    // Writing this directly simulates residual data at the nested VecMapper namespace.
    SingleTxApi::with_global_default_account(|account| {
        account.storage.insert(stale_len_key, vec![0x01]);
    });

    // Now insert key 99 via the public API using a VecMapper as the nested type.
    // The fix ensures insert_default clears the nested namespace before registering
    // the key, so the nested mapper is empty regardless of any residual data.
    let mut map = MapStorageMapper::<SingleTxApi, u64, VecMapper<SingleTxApi, u64>>::new(
        StorageKey::new(&base_key[..]),
    );
    assert!(map.insert_default(99));

    let nested = map.get(&99).unwrap();
    assert_eq!(
        nested.len(),
        0,
        "insert_default must clear residual nested storage; nested mapper must start empty"
    );
    assert!(nested.is_empty());
}
