use elrond_wasm::storage::mappers::SetMapper;
use elrond_wasm::storage::mappers::StorageMapper;
use elrond_wasm::BoxedBytes;
use elrond_wasm_debug::TxContext;

fn create_set() -> SetMapper<TxContext, u64> {
	SetMapper::new(TxContext::dummy(), BoxedBytes::from_concat(&[b"my_set"]))
}

#[test]
fn test_hash_set_simple() {
	let mut set = create_set();
	assert_eq!(set.len(), 0);
	assert_eq!(set.insert(42), true);
	assert_eq!(set.insert(42), false);
	assert_eq!(set.len(), 1);
	set.insert(43);
	assert_eq!(set.len(), 2);
	set.insert(44);
	assert_eq!(set.len(), 3);
	assert_eq!(set.contains(&42), true);
	assert_eq!(set.contains(&50), false);
}

#[test]
fn test_set_removal() {
	let mut set = create_set();
	assert_eq!(set.len(), 0);
	set.insert(42);
	set.insert(43);
	assert_eq!(set.len(), 2);
	assert_eq!(set.remove(&50), false);
	assert_eq!(set.len(), 2);
	assert_eq!(set.remove(&42), true);
	assert_eq!(set.contains(&42), false);
	assert_eq!(set.len(), 1);
	assert_eq!(set.remove(&42), false);
}
