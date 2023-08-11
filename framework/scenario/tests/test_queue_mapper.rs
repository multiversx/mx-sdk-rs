use multiversx_sc::storage::{
    mappers::{QueueMapper, StorageClearable, StorageMapper},
    StorageKey,
};
use multiversx_sc_scenario::api::SingleTxApi;

fn create_queue() -> QueueMapper<SingleTxApi, u64> {
    let base_key = StorageKey::new(&b"my_queue"[..]);
    QueueMapper::new(base_key)
}

#[test]
fn test_queue_simple() {
    let mut queue = create_queue();
    assert!(queue.check_internal_consistency());
    assert_eq!(queue.len(), 0);
    queue.push_back(42);
    assert_eq!(queue.len(), 1);
    assert!(queue.check_internal_consistency());
    queue.push_back(43);
    assert_eq!(queue.len(), 2);
    assert!(queue.check_internal_consistency());
    queue.push_back(44);
    assert_eq!(queue.len(), 3);
    assert!(queue.check_internal_consistency());
    assert_eq!(queue.front(), Some(42));
    let mut it = queue.iter();
    assert_eq!(it.next(), Some(42));
    assert_eq!(it.next(), Some(43));
    assert_eq!(it.next(), Some(44));
    assert_eq!(it.next(), None);
    assert!(queue.check_internal_consistency());
}

fn check_queue(queue: &QueueMapper<SingleTxApi, u64>, expected: Vec<u64>) {
    assert_eq!(queue.len(), expected.len());
    let vec: Vec<u64> = queue.iter().collect();
    assert_eq!(vec, expected);
    assert!(queue.check_internal_consistency());
}

#[test]
fn test_queue_pop() {
    let mut queue = create_queue();

    queue.push_back(44);
    assert!(queue.check_internal_consistency());
    queue.push_back(45);
    assert!(queue.check_internal_consistency());
    queue.push_back(46);
    assert!(queue.check_internal_consistency());
    queue.push_front(43);
    assert!(queue.check_internal_consistency());
    queue.push_front(42);
    assert!(queue.check_internal_consistency());

    check_queue(&queue, vec![42, 43, 44, 45, 46]);

    assert_eq!(queue.pop_back(), Some(46));
    check_queue(&queue, vec![42, 43, 44, 45]);

    assert_eq!(queue.pop_back(), Some(45));
    check_queue(&queue, vec![42, 43, 44]);

    assert_eq!(queue.pop_front(), Some(42));
    check_queue(&queue, vec![43, 44]);

    assert_eq!(queue.pop_front(), Some(43));
    check_queue(&queue, vec![44]);

    assert_eq!(queue.pop_front(), Some(44));
    check_queue(&queue, vec![]);

    assert_eq!(queue.pop_front(), None);
    assert_eq!(queue.pop_back(), None);
    assert!(queue.check_internal_consistency());
}

#[test]
fn test_queue_iter_processing() {
    let mut queue = create_queue();
    let range = 40..45;
    range.for_each(|value| queue.push_back(value));
    let processed: Vec<u64> = queue.iter().map(|val| val + 10).collect();
    let expected: Vec<u64> = (50..55).collect();
    assert_eq!(processed, expected);
    assert!(queue.check_internal_consistency());
}

#[test]
fn test_queue_clear() {
    let mut queue = create_queue();

    queue.push_back(44);
    queue.push_back(45);
    queue.push_back(46);
    assert!(queue.check_internal_consistency());
    queue.clear();
    assert!(queue.check_internal_consistency());
    assert_eq!(queue.len(), 0);
    assert!(queue.is_empty());
}
