use elrond_wasm::types::{LockableStaticBuffer, StaticBufferRef};
use elrond_wasm_debug::DebugApi;

fn new_static_buffer(bytes: &[u8]) -> Option<StaticBufferRef<DebugApi>> {
    StaticBufferRef::try_new(DebugApi::dummy(), bytes)
}

#[test]
fn test_try_extend_from_slice() {
    let mut s = new_static_buffer(b"z").unwrap();
    assert!(s.try_extend_from_slice(b"abc"));
    assert!(s.try_extend_from_slice(b"def"));
    assert!(s.contents_eq(b"zabcdef"));
}

#[test]
fn test_lock_unlock() {
    let api = DebugApi::dummy();
    {
        let s = StaticBufferRef::try_new(api.clone(), b"first").unwrap();
        assert!(s.contents_eq(b"first"));
        // should unlock here
    }

    let s = StaticBufferRef::try_new(api.clone(), b"another").unwrap();
    assert!(StaticBufferRef::try_new(api, b"no, locked").is_none());
    assert!(s.contents_eq(b"another"));
}

#[test]
fn test_extend_past_buffer_limits() {
    let mut s = new_static_buffer(&[]).unwrap();
    assert!(s.try_extend_from_slice(&[22; LockableStaticBuffer::capacity() - 1]));
    assert!(s.try_extend_from_slice(&[33; 1]));
    assert!(!s.try_extend_from_slice(&[44; 1]));
}

fn new_should_fail() {
    let buffer_option = new_static_buffer(b"test");
    assert!(buffer_option.is_none());
}

fn new_should_succeed() {
    let buffer_option = new_static_buffer(b"test");
    assert!(buffer_option.is_some());
}

#[test]
fn test_lock_2() {
    let api = DebugApi::dummy();
    let buffer_option = StaticBufferRef::try_new(api, b"locking_test");
    new_should_fail();
    assert!(buffer_option.is_some());
    let s1_buffer = buffer_option.unwrap();
    new_should_fail();
    assert!(s1_buffer.contents_eq(b"locking_test"));
    new_should_fail();
    drop(s1_buffer);
    new_should_succeed();
    new_should_succeed();
}
