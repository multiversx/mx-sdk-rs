use multiversx_sc::storage::{
    mappers::{SetMapper, StorageClearable, StorageMapper},
    StorageKey,
};
use multiversx_sc_scenario::api::SingleTxApi;

fn create_set() -> SetMapper<SingleTxApi, u64> {
    let base_key = StorageKey::new(&b"my_set"[..]);
    SetMapper::new(base_key)
}

fn check_set(set: &SetMapper<SingleTxApi, u64>, expected: Vec<u64>) {
    assert_eq!(set.len(), expected.len());
    assert!(set.check_internal_consistency());
    let actual: Vec<u64> = set.iter().collect();
    assert_eq!(actual, expected);
}

#[test]
fn test_hash_set_simple() {
    let mut set = create_set();
    check_set(&set, vec![]);
    assert!(set.insert(42));
    check_set(&set, vec![42]);
    assert!(!set.insert(42));
    check_set(&set, vec![42]);
    set.insert(43);
    check_set(&set, vec![42, 43]);
    set.insert(44);
    check_set(&set, vec![42, 43, 44]);
    assert!(set.contains(&42));
    assert!(!set.contains(&50));
}

#[test]
fn test_set_removal() {
    let mut set = create_set();
    check_set(&set, vec![]);
    set.insert(42);
    check_set(&set, vec![42]);
    set.insert(43);
    check_set(&set, vec![42, 43]);
    assert!(!set.remove(&50));
    check_set(&set, vec![42, 43]);
    assert!(set.remove(&42));
    check_set(&set, vec![43]);
    assert!(!set.contains(&42));
    assert!(!set.remove(&42));
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
    assert!(set.remove(&43));
    check_set(&set, vec![42, 44, 45]);
    assert!(set.remove(&44));
    check_set(&set, vec![42, 45]);
    assert!(set.remove(&45));
    check_set(&set, vec![42]);
    assert!(set.remove(&42));
    check_set(&set, vec![]);
}

#[test]
fn test_set_clear() {
    let mut set = create_set();
    set.insert(42);
    set.insert(43);
    set.insert(44);
    set.insert(45);
    assert!(set.check_internal_consistency());
    set.clear();
    assert!(set.check_internal_consistency());
    assert_eq!(set.len(), 0);
    assert!(set.is_empty());
}
