use crate::types::RawHandle;
use std::collections::HashMap;

#[derive(Debug)]
pub struct HandleMap<V> {
    next_handle: RawHandle,
    pub map: HashMap<RawHandle, V>,
}

impl<V> HandleMap<V> {
    pub fn new() -> Self {
        HandleMap {
            next_handle: 0,
            map: HashMap::new(),
        }
    }
}

impl<V> Default for HandleMap<V> {
    fn default() -> Self {
        HandleMap::new()
    }
}

impl<V> HandleMap<V> {
    pub fn insert_new_handle_raw(&mut self, value: V) -> RawHandle {
        let new_handle = self.next_handle;
        self.map.insert(new_handle, value);
        self.next_handle += 1;
        new_handle
    }

    pub fn get(&self, handle: RawHandle) -> &V {
        // TODO: consider simulating the actual error from the VM
        self.map
            .get(&handle)
            .unwrap_or_else(|| panic!("handle not found: {handle}"))
    }

    pub fn get_mut(&mut self, handle: RawHandle) -> &mut V {
        // TODO: consider simulating the actual error from the VM
        self.map
            .get_mut(&handle)
            .unwrap_or_else(|| panic!("handle not found: {handle}"))
    }

    pub fn insert(&mut self, handle: RawHandle, value: V) {
        let _ = self.map.insert(handle, value);
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn remove_handle(&mut self, handle: RawHandle) {
        debug_assert!(
            self.map.contains_key(&handle),
            "attempting to remove non-existing handle {handle}, this is a memory management issue"
        );
        let _ = self.map.remove(&handle);

        // Shrink only when capacity has grown 4x beyond the live entry count.
        // This triggers at most O(log n) times during a bulk delete, keeping
        // total rehash work at O(n log n) instead of O(n²).
        if self.map.capacity() > 4 * self.map.len() + 16 {
            self.map.shrink_to_fit();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_handle_map_is_empty() {
        let map: HandleMap<String> = HandleMap::new();
        assert_eq!(map.len(), 0);
        assert_eq!(map.next_handle, 0);
    }

    #[test]
    fn test_default_handle_map_is_empty() {
        let map: HandleMap<i32> = HandleMap::default();
        assert_eq!(map.len(), 0);
        assert_eq!(map.next_handle, 0);
    }

    #[test]
    fn test_insert_new_handle_raw_single() {
        let mut map = HandleMap::new();
        let handle = map.insert_new_handle_raw("test value");

        assert_eq!(handle, 0);
        assert_eq!(map.next_handle, 1);
        assert_eq!(map.len(), 1);
        assert_eq!(*map.get(handle), "test value");
    }

    #[test]
    fn test_insert_new_handle_raw_multiple() {
        let mut map = HandleMap::new();

        let h0 = map.insert_new_handle_raw(100);
        let h1 = map.insert_new_handle_raw(200);
        let h2 = map.insert_new_handle_raw(300);

        assert_eq!(h0, 0);
        assert_eq!(h1, 1);
        assert_eq!(h2, 2);
        assert_eq!(map.next_handle, 3);
        assert_eq!(map.len(), 3);

        assert_eq!(*map.get(h0), 100);
        assert_eq!(*map.get(h1), 200);
        assert_eq!(*map.get(h2), 300);
    }

    #[test]
    fn test_get_existing_handle() {
        let mut map = HandleMap::new();
        let handle = map.insert_new_handle_raw(vec![1, 2, 3]);

        let value = map.get(handle);
        assert_eq!(value, &vec![1, 2, 3]);
    }

    #[test]
    #[should_panic(expected = "handle not found: 999")]
    fn test_get_non_existent_handle_panics() {
        let map: HandleMap<i32> = HandleMap::new();
        let _ = map.get(999);
    }

    #[test]
    fn test_get_mut_and_modify() {
        let mut map = HandleMap::new();
        let handle = map.insert_new_handle_raw(42);

        {
            let value = map.get_mut(handle);
            *value = 100;
        }

        assert_eq!(*map.get(handle), 100);
    }

    #[test]
    #[should_panic(expected = "handle not found: 5")]
    fn test_get_mut_non_existent_handle_panics() {
        let mut map: HandleMap<String> = HandleMap::new();
        let _ = map.get_mut(5);
    }

    #[test]
    fn test_insert_at_specific_handle() {
        let mut map = HandleMap::new();

        map.insert(10, "value at 10");
        map.insert(20, "value at 20");

        assert_eq!(map.len(), 2);
        assert_eq!(*map.get(10), "value at 10");
        assert_eq!(*map.get(20), "value at 20");
        // next_handle should still be 0 since we didn't use insert_new_handle_raw
        assert_eq!(map.next_handle, 0);
    }

    #[test]
    fn test_insert_overwrites_existing_handle() {
        let mut map = HandleMap::new();
        let handle = map.insert_new_handle_raw("original");

        map.insert(handle, "updated");

        assert_eq!(map.len(), 1);
        assert_eq!(*map.get(handle), "updated");
    }

    #[test]
    fn test_remove_handle_success() {
        let mut map = HandleMap::new();
        let h0 = map.insert_new_handle_raw(100);
        let h1 = map.insert_new_handle_raw(200);

        assert_eq!(map.len(), 2);

        map.remove_handle(h0);

        assert_eq!(map.len(), 1);
        assert_eq!(*map.get(h1), 200);
    }

    #[test]
    #[should_panic(
        expected = "attempting to remove non-existing handle 42, this is a memory management issue"
    )]
    fn test_remove_non_existent_handle_panics() {
        let mut map: HandleMap<i32> = HandleMap::new();
        map.remove_handle(42);
    }

    #[test]
    #[should_panic(
        expected = "attempting to remove non-existing handle 0, this is a memory management issue"
    )]
    fn test_remove_already_removed_handle_panics() {
        let mut map = HandleMap::new();
        let handle = map.insert_new_handle_raw("value");

        map.remove_handle(handle);
        // Attempting to remove again should panic
        map.remove_handle(handle);
    }

    #[test]
    fn test_handle_lifecycle() {
        let mut map = HandleMap::new();

        // Insert multiple handles
        let h0 = map.insert_new_handle_raw("zero");
        let h1 = map.insert_new_handle_raw("one");
        let h2 = map.insert_new_handle_raw("two");

        assert_eq!(map.len(), 3);

        // Remove middle handle
        map.remove_handle(h1);
        assert_eq!(map.len(), 2);

        // Can still access other handles
        assert_eq!(*map.get(h0), "zero");
        assert_eq!(*map.get(h2), "two");

        // Insert new handle - should continue from next_handle
        let h3 = map.insert_new_handle_raw("three");
        assert_eq!(h3, 3);
        assert_eq!(map.len(), 3);
    }

    #[test]
    fn test_stress_many_handles() {
        let mut map = HandleMap::new();
        let count = 1000;

        // Insert many handles
        for i in 0..count {
            let handle = map.insert_new_handle_raw(i);
            assert_eq!(handle, i);
        }

        assert_eq!(map.len(), count as usize);
        assert_eq!(map.next_handle, count);

        // Verify all values are correct
        for i in 0..count {
            assert_eq!(*map.get(i), i);
        }

        // Remove every other handle
        for i in (0..count).step_by(2) {
            map.remove_handle(i);
        }

        assert_eq!(map.len(), (count / 2) as usize);

        // Verify remaining handles
        for i in (1..count).step_by(2) {
            assert_eq!(*map.get(i), i);
        }
    }

    #[test]
    fn test_handle_map_with_complex_type() {
        #[derive(Debug, PartialEq)]
        struct ComplexData {
            id: u32,
            name: String,
            values: Vec<i32>,
        }

        let mut map = HandleMap::new();

        let data1 = ComplexData {
            id: 1,
            name: "first".to_string(),
            values: vec![1, 2, 3],
        };

        let data2 = ComplexData {
            id: 2,
            name: "second".to_string(),
            values: vec![4, 5, 6],
        };

        let h1 = map.insert_new_handle_raw(data1);
        let h2 = map.insert_new_handle_raw(data2);

        assert_eq!(map.get(h1).id, 1);
        assert_eq!(map.get(h1).name, "first");
        assert_eq!(map.get(h2).values, vec![4, 5, 6]);

        // Modify complex data
        map.get_mut(h1).values.push(99);
        assert_eq!(map.get(h1).values, vec![1, 2, 3, 99]);
    }

    #[test]
    fn test_no_handle_reuse_after_remove() {
        let mut map = HandleMap::new();

        let h0 = map.insert_new_handle_raw("value0");
        let _h1 = map.insert_new_handle_raw("value1");

        map.remove_handle(h0);

        // Next handle should be 2, not reusing 0
        let h2 = map.insert_new_handle_raw("value2");
        assert_eq!(h2, 2);
        assert_eq!(map.next_handle, 3);
    }

    #[test]
    fn test_insert_and_insert_new_handle_raw_interleaved() {
        let mut map = HandleMap::new();

        // Use insert_new_handle_raw
        let h0 = map.insert_new_handle_raw("auto0");
        assert_eq!(h0, 0);

        // Manually insert at specific handle
        map.insert(100, "manual100");

        // Use insert_new_handle_raw again
        let h1 = map.insert_new_handle_raw("auto1");
        assert_eq!(h1, 1);

        // Verify all values
        assert_eq!(*map.get(0), "auto0");
        assert_eq!(*map.get(1), "auto1");
        assert_eq!(*map.get(100), "manual100");
        assert_eq!(map.len(), 3);
    }

    #[test]
    fn test_shrink_triggered_automatically_after_bulk_delete() {
        let mut map = HandleMap::new();
        let count = 1000;

        for i in 0..count {
            map.insert_new_handle_raw(i);
        }

        let capacity_before = map.map.capacity();
        assert!(capacity_before >= count as usize);

        // Removing all entries triggers automatic shrinking via the 4x heuristic.
        for i in 0..count {
            map.remove_handle(i);
        }
        assert_eq!(map.len(), 0);

        // Capacity must be much smaller than after the bulk insert.
        assert!(map.map.capacity() < capacity_before);
    }
}
