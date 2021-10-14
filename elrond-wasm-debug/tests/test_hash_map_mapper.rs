use elrond_wasm::storage::{
    mappers::{MapMapper, StorageClearable, StorageMapper},
    StorageKey,
};
use elrond_wasm_debug::DebugApi;

fn create_map() -> MapMapper<DebugApi, u64, u64> {
    let api = DebugApi::dummy();
    let base_key = StorageKey::new(api.clone(), &b"my_map"[..]);
    MapMapper::new(api, base_key)
}

#[test]
fn test_map_simple() {
    let mut map = create_map();
    assert_eq!(map.len(), 0);
    assert_eq!(map.get(&42), None);
    assert_eq!(map.insert(42, 142), None);
    assert_eq!(map.len(), 1);
    assert_eq!(map.contains_key(&42), true);
    assert_eq!(map.contains_key(&50), false);
    assert_eq!(map.insert(42, 242), Some(142));
    assert_eq!(map.len(), 1);
    assert_eq!(map.get(&42), Some(242));
}

#[test]
fn test_map_remove() {
    let mut map = create_map();
    map.insert(42, 142);
    map.insert(43, 143);
    assert_eq!(map.len(), 2);
    assert_eq!(map.remove(&42), Some(142));
    assert_eq!(map.remove(&42), None);
    assert_eq!(map.len(), 1);
}

#[test]
fn test_map_clear() {
    let mut map = create_map();
    map.insert(420, 421);
    map.insert(422, 423);
    assert_eq!(map.len(), 2);
    map.clear();
    assert_eq!(map.len(), 0);
    assert!(map.is_empty());
}
