use multiversx_sc::storage::{
    mappers::{LinkedListMapper, StorageClearable, StorageMapper},
    StorageKey,
};
use multiversx_sc_scenario::api::SingleTxApi;

fn create_list() -> LinkedListMapper<SingleTxApi, u64> {
    let base_key = StorageKey::new(&b"my_list"[..]);
    LinkedListMapper::new(base_key)
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
    assert_eq!(list.front().unwrap().into_value(), 42);
    let mut it = list.iter();
    assert_eq!(it.next().unwrap().into_value(), 42);
    assert_eq!(it.next().unwrap().into_value(), 43);
    assert_eq!(it.next().unwrap().into_value(), 44);
    assert!(it.next().is_none());
    assert!(list.check_internal_consistency());
}

fn check_list(list: &LinkedListMapper<SingleTxApi, u64>, expected: Vec<u64>) {
    assert_eq!(list.len(), expected.len());
    let vec: Vec<u64> = list.iter().map(|x| x.into_value()).collect();
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

    assert_eq!(list.pop_back().unwrap().into_value(), 46);
    check_list(&list, vec![42, 43, 44, 45]);

    assert_eq!(list.pop_back().unwrap().into_value(), 45);
    check_list(&list, vec![42, 43, 44]);

    assert_eq!(list.pop_front().unwrap().into_value(), 42);
    check_list(&list, vec![43, 44]);

    assert_eq!(list.pop_front().unwrap().into_value(), 43);
    check_list(&list, vec![44]);

    assert_eq!(list.pop_front().unwrap().into_value(), 44);
    check_list(&list, vec![]);

    assert!(list.pop_front().is_none());
    assert!(list.pop_back().is_none());
    assert!(list.check_internal_consistency());
}

#[test]
fn test_list_iter_processing() {
    let mut list = create_list();
    let range = 40..45;
    range.for_each(|value| {
        let _ = list.push_back(value);
    });
    let processed: Vec<u64> = list.iter().map(|val| val.into_value() + 10).collect();
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
