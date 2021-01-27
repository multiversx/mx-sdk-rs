use elrond_wasm::storage::mappers::LinkedListMapper;
use elrond_wasm::storage::mappers::StorageMapper;
use elrond_wasm::BoxedBytes;
use elrond_wasm_debug::TxContext;

fn create_list() -> LinkedListMapper<TxContext, u64> {
	LinkedListMapper::new(TxContext::dummy(), BoxedBytes::from_concat(&[b"my_list"]))
}

#[test]
fn test_list_simple() {
	let mut list = create_list();
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

fn assert_list_eq(list: &LinkedListMapper<TxContext, u64>, expected: Vec<u64>) {
	assert_eq!(list.len(), expected.len());
	let vec: Vec<u64> = list.iter().collect();
	assert_eq!(vec, expected);
}

#[test]
fn test_list_pop() {
	let mut list = create_list();
	(42..47).for_each(|value| list.push_back(value));
	assert_list_eq(&list, vec![42, 43, 44, 45, 46]);

	assert_eq!(list.pop_back(), Some(46));
	assert_list_eq(&list, vec![42, 43, 44, 45]);

	assert_eq!(list.pop_back(), Some(45));
	assert_list_eq(&list, vec![42, 43, 44]);

	assert_eq!(list.pop_front(), Some(42));
	assert_list_eq(&list, vec![43, 44]);

	assert_eq!(list.pop_front(), Some(43));
	assert_list_eq(&list, vec![44]);

	assert_eq!(list.pop_front(), Some(44));
	assert_list_eq(&list, vec![]);

	assert_eq!(list.pop_front(), None);
	assert_eq!(list.pop_back(), None);
}

#[test]
fn test_list_iter_processing() {
	let mut list = create_list();
	let range = 40..45;
	range.for_each(|value| list.push_back(value));
	let processed: Vec<u64> = list.iter().map(|val| val + 10).collect();
	let expected: Vec<u64> = (50..55).collect();
	assert_eq!(processed, expected);
}
