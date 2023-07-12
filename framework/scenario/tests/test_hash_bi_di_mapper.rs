use multiversx_sc::storage::{
    mappers::{BiDiMapper, StorageMapper},
    StorageKey,
};
use multiversx_sc_scenario::api::SingleTxApi;

fn create_set_1() -> BiDiMapper<SingleTxApi, u64, u32> {
    let base_key = StorageKey::new(&b"my_bidi_set"[..]);
    BiDiMapper::new(base_key)
}

fn check_set_1(set: &BiDiMapper<SingleTxApi, u64, u32>, ids: Vec<u64>, values: Vec<u32>) {
    assert_eq!(values.len(), ids.len());
    assert_eq!(set.len(), ids.len());

    let actual_ids: Vec<u64> = set.get_all_ids().collect();
    let actual_values: Vec<u32> = set.get_all_values().collect();

    for id in ids.clone() {
        assert!(actual_ids.contains(&id));
    }

    for actual_id in actual_ids {
        assert!(ids.contains(&actual_id));
    }

    for val in values.clone() {
        assert!(actual_values.contains(&val));
    }

    for actual_val in actual_values {
        assert!(values.contains(&actual_val));
    }
}

#[test]
fn test_hash_set_simple_1() {
    let mut set = create_set_1();
    check_set_1(&set, vec![], vec![]);

    assert!(set.insert(42, 43));
    check_set_1(&set, vec![42], vec![43]);
    assert!(!set.insert(42, 44));
    assert!(!set.insert(44, 43));

    assert!(set.insert(1, 101));
    assert!(set.insert(2, 102));
    assert!(set.insert(3, 103));
    assert!(set.insert(4, 104));
    check_set_1(&set, vec![42, 1, 2, 3, 4], vec![43, 101, 102, 103, 104]);

    assert_eq!(set.get_id(&101), 1);
    assert_eq!(set.get_value(&4), 104);
}

#[test]
fn test_set_removal_by_id() {
    let mut set = create_set_1();
    check_set_1(&set, vec![], vec![]);

    assert!(set.insert(42, 43));
    check_set_1(&set, vec![42], vec![43]);
    assert!(!set.insert(42, 44));
    assert!(!set.insert(44, 43));

    assert!(set.insert(1, 101));
    assert!(set.insert(2, 102));
    assert!(set.insert(3, 103));
    assert!(set.insert(4, 104));
    check_set_1(&set, vec![42, 1, 2, 3, 4], vec![43, 101, 102, 103, 104]);

    assert!(set.remove_by_id(&42));
    check_set_1(&set, vec![1, 2, 3, 4], vec![101, 102, 103, 104]);
    assert_eq!(set.get_id(&101), 1);
    assert_eq!(set.get_value(&4), 104);
    assert!(!set.remove_by_id(&42));

    set.remove_all_by_ids([2, 4]);
    check_set_1(&set, vec![1, 3], vec![101, 103]);
}

#[test]
fn test_set_removal_by_value() {
    let mut set = create_set_1();
    check_set_1(&set, vec![], vec![]);

    assert!(set.insert(42, 43));
    check_set_1(&set, vec![42], vec![43]);
    assert!(!set.insert(42, 44));
    assert!(!set.insert(44, 43));

    assert!(set.insert(1, 101));
    assert!(set.insert(2, 102));
    assert!(set.insert(3, 103));
    assert!(set.insert(4, 104));
    check_set_1(&set, vec![42, 1, 2, 3, 4], vec![43, 101, 102, 103, 104]);

    assert!(set.remove_by_value(&43));
    check_set_1(&set, vec![1, 2, 3, 4], vec![101, 102, 103, 104]);
    assert_eq!(set.get_id(&101), 1);
    assert_eq!(set.get_value(&4), 104);
    assert!(!set.remove_by_value(&43));

    set.remove_all_by_values([102, 104]);
    check_set_1(&set, vec![1, 3], vec![101, 103]);
}
