use elrond_wasm::storage::mappers::{MapMapper, StorageClearable, StorageMapper};
use elrond_wasm::types::BoxedBytes;
use elrond_wasm_debug::TxContext;

fn create_map() -> MapMapper<TxContext, u64, u64> {
	MapMapper::new(TxContext::dummy(), BoxedBytes::from_concat(&[b"my_map"]))
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
