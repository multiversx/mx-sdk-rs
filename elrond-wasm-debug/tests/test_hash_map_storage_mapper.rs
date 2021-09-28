use elrond_wasm::storage::{
    mappers::{MapMapper, StorageClearable, StorageMapper},
    StorageKey,
};
use elrond_wasm_debug::TxContext;

fn create_map_storage() -> MapMapper<TxContext, u64, MapMapper<TxContext, u64, u64>> {
    let api = TxContext::dummy();
    let base_key = StorageKey::new(api.clone(), &b"my_map_storage"[..]);
    MapMapper::new(api, base_key)
}

#[test]
fn test_map_storage_simple() {
    let mut map = create_map_storage();
    assert_eq!(map.len(), 0);
    let mut nested_map = map.insert_nested(42);
    nested_map.insert(50, 100);
    assert_eq!(nested_map.len(), 1);
    assert!(map.insert_nested(42).is_empty());
    let map42_option = map.get_nested(&42);
    assert!(map42_option.is_some());
    let mut map42 = map42_option.unwrap();
    assert_eq!(map42.insert(100, 111), None);
    assert_eq!(map42.insert(100, 200), Some(111));
    assert_eq!(map42.insert(101, 201), None);
    assert_eq!(map42.len(), 2);
    assert_eq!(map.len(), 1);
    map.insert_nested(43);
    assert_eq!(map.len(), 2);
    map.insert_nested(44);
    assert_eq!(map.len(), 3);
    assert_eq!(map.contains_key(&42), true);
    assert_eq!(map.contains_key(&50), false);
}

#[test]
fn test_map_storage_remove() {
    let mut map = create_map_storage();
    map.insert_nested(42);
    map.insert_nested(43);
    assert_eq!(map.len(), 2);
    assert_eq!(map.remove_nested(&42), true);
    assert_eq!(map.remove_nested(&42), false);
    assert_eq!(map.len(), 1);
}

#[test]
fn test_map_storage_clear() {
    let mut map = create_map_storage();
    map.insert_nested(42);
    let mut nested_map = map.get_nested(&42).unwrap();
    nested_map.insert(420, 421);
    nested_map.insert(422, 423);
    assert_eq!(nested_map.len(), 2);
    map.clear();
    assert_eq!(nested_map.len(), 0);
    assert_eq!(map.len(), 0);
    assert!(map.is_empty());
}
