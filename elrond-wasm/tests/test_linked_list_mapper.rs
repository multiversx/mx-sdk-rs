use elrond_wasm_debug::TxContext;
use elrond_wasm::BoxedBytes;
use elrond_wasm::storage::mappers::StorageMapper;
use elrond_wasm::storage::mappers::LinkedListMapper;

fn create_list() -> LinkedListMapper<TxContext, u64> {
	LinkedListMapper::new(TxContext::dummy(), BoxedBytes::from_concat(&[ b"my_list"]))
}

#[test]
fn test_linked_list_simple() {
	let mut list  = create_list();
	assert_eq!(list.len(), 0);
	list.push_back(42);
	assert_eq!(list.len(), 1);
	list.push_back(43);
	assert_eq!(list.len(), 2);
	list.push_back(44);
	assert_eq!(list.len(), 3);
	assert_eq!(list.front(), Some(42));
	let mut it = list.iter();
	assert_eq!(it.next(), Some(42));
	assert_eq!(it.next(), Some(43));
	assert_eq!(it.next(), Some(44));
	assert_eq!(it.next(), None);
}

fn assert_list_eq(list : &LinkedListMapper<TxContext, u64>, expected : Vec<u64>) {
	assert_eq!(list.len(), expected.len());
	let vec : Vec<u64> = list.iter().collect();
	assert_eq!(vec, expected);
}

#[test]
fn test_linked_list_indexed_removal() {
	let mut list  = create_list();
	(42..45).for_each(|value | list.push_back(value));
	assert_list_eq(&list, vec![42, 43, 44]);

	list.remove_at_index(2);
	assert_list_eq(&list, vec![42, 44]);

	list.remove_at_index(1);
	assert_list_eq(&list, vec![44]);

	list.remove_at_index(3);
	assert_eq!(list.len(), 0);
}

#[test]
fn test_linked_list_iter_processing() {
	let mut list  = create_list();
	let range = 40..45;
	range.for_each(|value | list.push_back(value));
	let processed : Vec<u64> = list.iter().map(|val| val + 10).collect();
	let expected : Vec<u64> = (50..55).collect();
	assert_eq!(processed, expected);
}
