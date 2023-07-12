use multiversx_sc::storage::{
    mappers::{StorageMapper, VecMapper},
    StorageKey,
};
use multiversx_sc_scenario::api::SingleTxApi;

fn create_vec() -> VecMapper<SingleTxApi, u64> {
    let base_key = StorageKey::new(&b"my_queue"[..]);
    VecMapper::new(base_key)
}

#[test]
fn test_vec_simple() {
    let mut vect = create_vec();
    assert_eq!(vect.len(), 0);
    vect.push(&42);
    assert_eq!(vect.len(), 1);
    assert_eq!(vect.get(1), 42);
    vect.push(&43);
    assert_eq!(vect.len(), 2);
    assert_eq!(vect.get(2), 43);
    vect.push(&44);
    assert_eq!(vect.len(), 3);
    assert_eq!(vect.get(3), 44);
    vect.extend_from_slice(&[45, 46, 47, 48, 49]);
    assert_eq!(vect.get(5), 46);
    assert_eq!(vect.get_unchecked(9), 0);
    let mut it = vect.iter();
    assert_eq!(it.next(), Some(42));
    assert_eq!(it.next(), Some(43));
    assert_eq!(it.next(), Some(44));
    assert_eq!(it.next(), Some(45));
    assert_eq!(it.next(), Some(46));
    assert_eq!(it.next(), Some(47));
    assert_eq!(it.next(), Some(48));
    assert_eq!(it.next(), Some(49));
    assert_eq!(it.next(), None);
}

fn check_vec(vect: &VecMapper<SingleTxApi, u64>, expected: Vec<u64>) {
    assert_eq!(vect.len(), expected.len());
    let vec: Vec<u64> = vect.load_as_vec();
    assert_eq!(vec, expected);
}

#[test]
fn test_vec_clear_entries() {
    let mut vect = create_vec();

    vect.extend_from_slice(&[42, 43, 44, 45, 46]);

    assert_eq!(vect.len(), 5);
    assert_eq!(vect.get(5), 46);
    vect.clear_entry(5);
    assert_eq!(vect.len(), 5);
    assert!(vect.item_is_empty(5));
    assert_eq!(vect.get(5), 0);
    vect.clear_entry(2);
    vect.clear_entry(3);
    vect.clear_entry(5);
    check_vec(&vect, vec![42, 0, 0, 45, 0]);
}

#[test]
fn test_vec_swap_remove() {
    let mut vect = create_vec();

    vect.extend_from_slice(&[42, 43, 44, 45, 46]);

    assert_eq!(vect.len(), 5);
    assert_eq!(vect.get(5), 46);
    vect.swap_remove(5);
    assert_eq!(vect.len(), 4);
    assert_eq!(vect.get_unchecked(5), 0);
    check_vec(&vect, vec![42, 43, 44, 45]);
    vect.swap_remove(2);
    check_vec(&vect, vec![42, 45, 44]);
    vect.swap_remove(3);
    check_vec(&vect, vec![42, 45]);
}

#[test]
fn test_vec_iter_processing() {
    let mut vect = create_vec();
    let range = 40..45;
    range.for_each(|value| {
        let _ = vect.push(&value);
    });
    let processed: Vec<u64> = vect.iter().map(|val| val + 10).collect();
    let expected: Vec<u64> = (50..55).collect();
    assert_eq!(processed, expected);
}

#[test]
fn test_vec_clear() {
    let mut vect = create_vec();

    vect.push(&44);
    vect.push(&45);
    vect.push(&46);
    vect.clear();
    assert_eq!(vect.len(), 0);
    assert!(vect.is_empty());
}
