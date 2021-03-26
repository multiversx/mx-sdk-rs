use elrond_wasm::storage::mappers::{MapMapper, MapStorageMapper, StorageClearable, StorageMapper};
use elrond_wasm::types::BoxedBytes;
use elrond_wasm_debug::TxContext;

fn create_map_storage() -> MapStorageMapper<TxContext, u64, MapMapper<TxContext, u64, u64>> {
	MapStorageMapper::new(
		TxContext::dummy(),
		BoxedBytes::from_concat(&[b"my_map_storage"]),
	)
}

#[test]
fn test_map_storage_simple() {
	let mut map = create_map_storage();
	assert_eq!(map.len(), 0);
	assert_eq!(map.insert_default(42), true);
	assert_eq!(map.insert_default(42), false);
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
	assert_eq!(map.contains_key(&42), true);
	assert_eq!(map.contains_key(&50), false);
}

#[test]
fn test_map_storage_remove() {
	let mut map = create_map_storage();
	map.insert_default(42);
	map.insert_default(43);
	assert_eq!(map.len(), 2);
	assert_eq!(map.remove(&42), true);
	assert_eq!(map.remove(&42), false);
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
