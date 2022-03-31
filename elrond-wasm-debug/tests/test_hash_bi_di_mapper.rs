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

    assert_eq!(actual_ids, ids);
    assert_eq!(actual_values, values);
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
