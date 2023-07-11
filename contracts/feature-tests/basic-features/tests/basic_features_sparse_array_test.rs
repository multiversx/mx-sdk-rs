use multiversx_sc::types::SparseArray;
use multiversx_sc_scenario::api::StaticApi;

#[test]
fn sparse_array_test() {
    let mut array = SparseArray::<StaticApi, 100>::new(5);
    assert_eq!(array.len(), 5);
    assert_eq!(array.as_raw_slice(), vec![0, 0, 0, 0, 0].as_slice());

    for (i, val) in array.iter().enumerate() {
        assert_eq!(i, val);
    }
    assert_eq!(array.get(2), 2);

    array.set(2, 5);
    assert_eq!(array.get(2), 5);
    assert_eq!(array.as_raw_slice(), vec![0, 0, 5, 0, 0].as_slice());

    let val = array.swap_remove(2);
    assert_eq!(val, 5);
    assert_eq!(array.len(), 4);
    assert_eq!(array.as_raw_slice(), vec![0, 0, 4, 0].as_slice());
}

#[should_panic]
#[test]
fn sparse_array_create_over_capacity_test() {
    let _ = SparseArray::<StaticApi, 100>::new(101);
}

#[should_panic]
#[test]
fn sparse_array_get_invalid_index_test() {
    let array = SparseArray::<StaticApi, 100>::new(5);
    let _ = array.get(5);
}
