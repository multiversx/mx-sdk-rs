use elrond_wasm::storage::{
    mappers::{BiDiMapper, StorageMapper},
    StorageKey,
};
use elrond_wasm_debug::DebugApi;

fn create_set() -> BiDiMapper<DebugApi, u64, u32> {
    let _ = DebugApi::dummy();
    let base_key = StorageKey::new(&b"my_bidi_set"[..]);
    BiDiMapper::new(base_key)
}

fn check_set(set: &BiDiMapper<DebugApi, u64, u32>, ids: Vec<u64>, values: Vec<u32>) {
    assert_eq!(values.len(), ids.len());
    assert_eq!(set.len(), ids.len());

    let actual_ids: Vec<u64> = set.get_all_ids().into_vec();
    let actual_values: Vec<u32> = set.get_all_values().into_vec();

    assert_eq!(actual_ids, ids);
    assert_eq!(actual_values, values);
}

#[test]
fn test_hash_set_simple() {
    let mut set = create_set();
    check_set(&set, vec![], vec![]);

    assert_eq!(set.insert(42, 43), true);
    check_set(&set, vec![42], vec![43]);
    assert_eq!(set.insert(42, 44), false);
    assert_eq!(set.insert(44, 43), false);

    assert_eq!(set.insert(1, 101), true);
    assert_eq!(set.insert(2, 102), true);
    assert_eq!(set.insert(3, 103), true);
    assert_eq!(set.insert(4, 104), true);
    check_set(&set, vec![42, 1, 2, 3, 4], vec![43, 101, 102, 103, 104]);

    assert_eq!(set.get_id(&101), 1);
    assert_eq!(set.get_value(&4), 104);
}

// #[test]
// fn test_set_removal() {
//     let mut set = create_set();
//     check_set(&set, vec![]);
//     set.insert(42);
//     check_set(&set, vec![42]);
//     set.insert(43);
//     check_set(&set, vec![42, 43]);
//     assert_eq!(set.swap_remove(&50), false);
//     check_set(&set, vec![42, 43]);
//     assert_eq!(set.swap_remove(&42), true);
//     check_set(&set, vec![43]);
//     assert_eq!(set.contains(&42), false);
//     assert_eq!(set.swap_remove(&42), false);
//     check_set(&set, vec![43]);
// }

// #[test]
// fn test_set_removal_from_middle() {
//     let mut set = create_set();
//     set.insert(42);
//     set.insert(43);
//     set.insert(44);
//     set.insert(45);
//     check_set(&set, vec![42, 43, 44, 45]);
//     assert_eq!(set.swap_remove(&43), true);
//     check_set(&set, vec![42, 45, 44]);
//     assert_eq!(set.swap_remove(&45), true);
//     check_set(&set, vec![42, 44]);
//     assert_eq!(set.swap_remove(&44), true);
//     check_set(&set, vec![42]);
//     assert_eq!(set.swap_remove(&42), true);
//     check_set(&set, vec![]);
// }

// #[test]
// fn test_set_clear() {
//     let mut set = create_set();
//     set.insert(42);
//     set.insert(43);
//     set.insert(44);
//     set.insert(45);
//     set.clear();
//     assert_eq!(set.len(), 0);
//     assert!(set.is_empty());
// }
