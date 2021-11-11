use elrond_wasm::types::StaticBufferRef;
use elrond_wasm_debug::DebugApi;

#[test]
fn test_try_extend_from_slice() {
    let mut s = StaticBufferRef::<DebugApi>::try_new(b"z").unwrap();
    assert!(s.try_extend_from_slice(b"abc"));
    assert!(s.try_extend_from_slice(b"def"));
    assert!(s.contents_eq(b"zabcdef"));
}

#[test]
fn test_lock_unlock() {
    {
        let s = StaticBufferRef::<DebugApi>::try_new(b"first").unwrap();
        assert!(s.contents_eq(b"first"));
        // should unlock here
    }

    let s = StaticBufferRef::<DebugApi>::try_new(b"another").unwrap();
    assert!(StaticBufferRef::<DebugApi>::try_new(b"no, locked").is_none());
    assert!(s.contents_eq(b"another"));
}
