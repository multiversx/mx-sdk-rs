use elrond_wasm::storage::mappers::{QueueMapper, StorageClearable, StorageMapper};
use elrond_wasm::storage::StorageKey;
use elrond_wasm_debug::TxContext;

fn create_list() -> QueueMapper<TxContext, u64> {
    let api = TxContext::dummy();
    let base_key = StorageKey::new(api.clone(), &b"my_list"[..]);
    QueueMapper::new(api, base_key)
}

#[test]
fn test_list_simple() {
    let mut list = create_list();
    assert!(list.check_internal_consistency());
    assert_eq!(list.len(), 0);
    list.push_back(42);
    assert_eq!(list.len(), 1);
    assert!(list.check_internal_consistency());
    list.push_back(43);
    assert_eq!(list.len(), 2);
    assert!(list.check_internal_consistency());
    list.push_back(44);
    assert_eq!(list.len(), 3);
    assert!(list.check_internal_consistency());
    assert_eq!(list.front(), Some(42));
    let mut it = list.iter();
    assert_eq!(it.next(), Some(42));
    assert_eq!(it.next(), Some(43));
    assert_eq!(it.next(), Some(44));
    assert_eq!(it.next(), None);
    assert!(list.check_internal_consistency());
}

fn check_list(list: &QueueMapper<TxContext, u64>, expected: Vec<u64>) {
    assert_eq!(list.len(), expected.len());
    let vec: Vec<u64> = list.iter().collect();
    assert_eq!(vec, expected);
    assert!(list.check_internal_consistency());
}

#[test]
fn test_list_pop() {
    let mut list = create_list();

    list.push_back(44);
    assert!(list.check_internal_consistency());
    list.push_back(45);
    assert!(list.check_internal_consistency());
    list.push_back(46);
    assert!(list.check_internal_consistency());
    list.push_front(43);
    assert!(list.check_internal_consistency());
    list.push_front(42);
    assert!(list.check_internal_consistency());

    check_list(&list, vec![42, 43, 44, 45, 46]);

    assert_eq!(list.pop_back(), Some(46));
    check_list(&list, vec![42, 43, 44, 45]);

    assert_eq!(list.pop_back(), Some(45));
    check_list(&list, vec![42, 43, 44]);

    assert_eq!(list.pop_front(), Some(42));
    check_list(&list, vec![43, 44]);

    assert_eq!(list.pop_front(), Some(43));
    check_list(&list, vec![44]);

    assert_eq!(list.pop_front(), Some(44));
    check_list(&list, vec![]);

    assert_eq!(list.pop_front(), None);
    assert_eq!(list.pop_back(), None);
    assert!(list.check_internal_consistency());
}

#[test]
fn test_list_iter_processing() {
    let mut list = create_list();
    let range = 40..45;
    range.for_each(|value| list.push_back(value));
    let processed: Vec<u64> = list.iter().map(|val| val + 10).collect();
    let expected: Vec<u64> = (50..55).collect();
    assert_eq!(processed, expected);
    assert!(list.check_internal_consistency());
}

#[test]
fn test_list_clear() {
    let mut list = create_list();

    list.push_back(44);
    list.push_back(45);
    list.push_back(46);
    assert!(list.check_internal_consistency());
    list.clear();
    assert!(list.check_internal_consistency());
    assert_eq!(list.len(), 0);
    assert!(list.is_empty());
}
