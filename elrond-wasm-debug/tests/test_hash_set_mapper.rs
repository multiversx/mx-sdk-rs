use elrond_wasm::storage::mappers::SetMapper;
use elrond_wasm::storage::mappers::StorageMapper;
use elrond_wasm::BoxedBytes;
use elrond_wasm_debug::TxContext;

fn create_set() -> SetMapper<TxContext, u64> {
	SetMapper::new(TxContext::dummy(), BoxedBytes::from_concat(&[b"my_set"]))
}

fn check_set(set: &SetMapper<TxContext, u64>, expected: Vec<u64>) {
	assert_eq!(set.len(), expected.len());
	assert!(set.check_internal_consistency());
	let actual: Vec<u64> = set.iter().collect();
	assert_eq!(actual, expected);
}

#[test]
fn test_hash_set_simple() {
	let mut set = create_set();
	check_set(&set, vec![]);
	assert_eq!(set.insert(42), true);
	check_set(&set, vec![42]);
	assert_eq!(set.insert(42), false);
	check_set(&set, vec![42]);
	set.insert(43);
	check_set(&set, vec![42, 43]);
	set.insert(44);
	check_set(&set, vec![42, 43, 44]);
	assert_eq!(set.contains(&42), true);
	assert_eq!(set.contains(&50), false);
}

#[test]
fn test_set_removal() {
	let mut set = create_set();
	check_set(&set, vec![]);
	set.insert(42);
	check_set(&set, vec![42]);
	set.insert(43);
	check_set(&set, vec![42, 43]);
	assert_eq!(set.remove(&50), false);
	check_set(&set, vec![42, 43]);
	assert_eq!(set.remove(&42), true);
	check_set(&set, vec![43]);
	assert_eq!(set.contains(&42), false);
	assert_eq!(set.remove(&42), false);
	check_set(&set, vec![43]);
}

#[test]
fn test_set_removal_from_middle() {
	let mut set = create_set();
	set.insert(42);
	set.insert(43);
	set.insert(44);
	set.insert(45);
	check_set(&set, vec![42, 43, 44, 45]);
	assert_eq!(set.remove(&43), true);
	check_set(&set, vec![42, 44, 45]);
	assert_eq!(set.remove(&44), true);
	check_set(&set, vec![42, 45]);
	assert_eq!(set.remove(&45), true);
	check_set(&set, vec![42]);
	assert_eq!(set.remove(&42), true);
	check_set(&set, vec![]);
}
