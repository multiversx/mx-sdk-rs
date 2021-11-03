use elrond_wasm::storage::{
    mappers::{StorageClearable, StorageMapper, UnorderedSetMapper},
    StorageKey,
};
use elrond_wasm_debug::DebugApi;

fn create_set() -> UnorderedSetMapper<DebugApi, u64> {
    let api = DebugApi::dummy();
    let base_key = StorageKey::new(api.clone(), &b"my_unordered_set"[..]);
    UnorderedSetMapper::new(api, base_key)
}

fn check_set(set: &UnorderedSetMapper<DebugApi, u64>, expected: Vec<u64>) {
    assert_eq!(set.len(), expected.len());
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
    assert_eq!(set.swap_remove(&50), false);
    check_set(&set, vec![42, 43]);
    assert_eq!(set.swap_remove(&42), true);
    check_set(&set, vec![43]);
    assert_eq!(set.contains(&42), false);
    assert_eq!(set.swap_remove(&42), false);
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
    assert_eq!(set.swap_remove(&43), true);
    check_set(&set, vec![42, 45, 44]);
    assert_eq!(set.swap_remove(&45), true);
    check_set(&set, vec![42, 44]);
    assert_eq!(set.swap_remove(&44), true);
    check_set(&set, vec![42]);
    assert_eq!(set.swap_remove(&42), true);
    check_set(&set, vec![]);
}

#[test]
fn test_set_clear() {
    let mut set = create_set();
    set.insert(42);
    set.insert(43);
    set.insert(44);
    set.insert(45);
    set.clear();
    assert_eq!(set.len(), 0);
    assert!(set.is_empty());
}
