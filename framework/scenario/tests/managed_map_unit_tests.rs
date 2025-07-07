use multiversx_sc::types::{ManagedBuffer, ManagedMap};
use multiversx_sc_scenario::api::StaticApi;

#[test]
fn key_mutability_test() {
    let mut map = ManagedMap::<StaticApi>::new();
    let mut key = ManagedBuffer::from("hello");
    let value = ManagedBuffer::from("world");

    map.put(&key, &value);

    let result = map.get(&key);
    assert_eq!(result, value);

    key.overwrite(b"more_text");

    let result = map.get(&key);
    assert!(result.is_empty());
}

#[test]
fn key_mutation_breaks_contains() {
    let mut map = ManagedMap::<StaticApi>::new();
    let mut key = ManagedBuffer::from("test_key");
    let value = ManagedBuffer::from("test_value");

    map.put(&key, &value);
    assert!(map.contains(&key));

    key.overwrite(b"different_key");

    assert!(!map.contains(&key));
}

#[test]
fn key_mutation_breaks_remove() {
    let mut map = ManagedMap::<StaticApi>::new();
    let mut key = ManagedBuffer::from("removable");
    let value = ManagedBuffer::from("value");

    map.put(&key, &value);

    key.overwrite(b"not_removable");

    let removed = map.remove(&key);
    assert!(removed.is_empty());

    let still_there = map.get(&ManagedBuffer::from("removable"));
    assert_eq!(still_there, value);
}

#[test]
fn multiple_keys_same_buffer_mutation() {
    let mut map = ManagedMap::<StaticApi>::new();
    let mut key_buffer = ManagedBuffer::from("key1");

    let value1 = ManagedBuffer::from("value1");
    map.put(&key_buffer, &value1);

    key_buffer.overwrite(b"key2");
    let value2 = ManagedBuffer::from("value2");
    map.put(&key_buffer, &value2);

    key_buffer.overwrite(b"key1");
    let result1 = map.get(&key_buffer);
    assert_eq!(result1, value1);

    key_buffer.overwrite(b"key2");
    let result2 = map.get(&key_buffer);
    assert_eq!(result2, value2);
}

#[test]
fn key_mutation_then_restore() {
    let mut map = ManagedMap::<StaticApi>::new();
    let mut key = ManagedBuffer::from("original");
    let value = ManagedBuffer::from("value");

    map.put(&key, &value);

    key.overwrite(b"changed");
    assert!(map.get(&key).is_empty());

    key.overwrite(b"original");
    let result = map.get(&key);

    assert_eq!(result, value);
}

#[test]
fn empty_key_mutation() {
    let mut map = ManagedMap::<StaticApi>::new();
    let mut key = ManagedBuffer::from("");
    let value = ManagedBuffer::from("empty_key_value");

    map.put(&key, &value);

    key.overwrite(b"not_empty");
    let result = map.get(&key);
    assert!(result.is_empty());

    key.overwrite(b"");
    let result = map.get(&key);
    assert_eq!(result, value);
}
