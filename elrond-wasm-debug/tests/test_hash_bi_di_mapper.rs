use elrond_wasm::storage::{
    mappers::{BiDiMapper, StorageMapper},
    StorageKey,
};
use elrond_wasm_debug::DebugApi;

fn create_set_1() -> BiDiMapper<DebugApi, u64, u32> {
    let _ = DebugApi::dummy();
    let base_key = StorageKey::new(&b"my_bidi_set"[..]);
    BiDiMapper::new(base_key)
}

fn check_set_1(set: &BiDiMapper<DebugApi, u64, u32>, ids: Vec<u64>, values: Vec<u32>) {
    assert_eq!(values.len(), ids.len());
    assert_eq!(set.len(), ids.len());

    let actual_ids: Vec<u64> = set.get_all_ids().into_vec();
    let actual_values: Vec<u32> = set.get_all_values().into_vec();

    for id in ids.clone() {
        assert_eq!(actual_ids.contains(&id), true);
    }

    for actual_id in actual_ids {
        assert_eq!(ids.contains(&actual_id), true);
    }

    for val in values.clone() {
        assert_eq!(actual_values.contains(&val), true);
    }

    for actual_val in actual_values {
        assert_eq!(values.contains(&actual_val), true);
    }
}

#[test]
fn test_hash_set_simple_1() {
    let mut set = create_set_1();
    check_set_1(&set, vec![], vec![]);

    assert_eq!(set.insert(42, 43), true);
    check_set_1(&set, vec![42], vec![43]);
    assert_eq!(set.insert(42, 44), false);
    assert_eq!(set.insert(44, 43), false);

    assert_eq!(set.insert(1, 101), true);
    assert_eq!(set.insert(2, 102), true);
    assert_eq!(set.insert(3, 103), true);
    assert_eq!(set.insert(4, 104), true);
    check_set_1(&set, vec![42, 1, 2, 3, 4], vec![43, 101, 102, 103, 104]);

    assert_eq!(set.get_id(&101), 1);
    assert_eq!(set.get_value(&4), 104);
}

#[test]
fn test_set_removal_by_id() {
    let mut set = create_set_1();
    check_set_1(&set, vec![], vec![]);

    assert_eq!(set.insert(42, 43), true);
    check_set_1(&set, vec![42], vec![43]);
    assert_eq!(set.insert(42, 44), false);
    assert_eq!(set.insert(44, 43), false);

    assert_eq!(set.insert(1, 101), true);
    assert_eq!(set.insert(2, 102), true);
    assert_eq!(set.insert(3, 103), true);
    assert_eq!(set.insert(4, 104), true);
    check_set_1(&set, vec![42, 1, 2, 3, 4], vec![43, 101, 102, 103, 104]);

    assert_eq!(set.remove_by_id(&42), true);
    check_set_1(&set, vec![1, 2, 3, 4], vec![101, 102, 103, 104]);
    assert_eq!(set.get_id(&101), 1);
    assert_eq!(set.get_value(&4), 104);
    assert_eq!(set.remove_by_id(&42), false);

    set.remove_all_by_ids([2, 4]);
    check_set_1(&set, vec![1, 3], vec![101, 103]);
}

#[test]
fn test_set_removal_by_value() {
    let mut set = create_set_1();
    check_set_1(&set, vec![], vec![]);

    assert_eq!(set.insert(42, 43), true);
    check_set_1(&set, vec![42], vec![43]);
    assert_eq!(set.insert(42, 44), false);
    assert_eq!(set.insert(44, 43), false);

    assert_eq!(set.insert(1, 101), true);
    assert_eq!(set.insert(2, 102), true);
    assert_eq!(set.insert(3, 103), true);
    assert_eq!(set.insert(4, 104), true);
    check_set_1(&set, vec![42, 1, 2, 3, 4], vec![43, 101, 102, 103, 104]);

    assert_eq!(set.remove_by_value(&43), true);
    check_set_1(&set, vec![1, 2, 3, 4], vec![101, 102, 103, 104]);
    assert_eq!(set.get_id(&101), 1);
    assert_eq!(set.get_value(&4), 104);
    assert_eq!(set.remove_by_value(&43), false);

    set.remove_all_by_values([102, 104]);
    check_set_1(&set, vec![1, 3], vec![101, 103]);
}
