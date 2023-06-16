use multiversx_sc::types::{ManagedBuffer, ManagedVec};
use multiversx_sc_scenario::DebugApi;
use multiversx_sdk::data::types::native::NativeConvertible;

#[test]
fn test_managed_vec_to_native() {
    let _ = DebugApi::dummy();
    let mut managed_vec: ManagedVec<DebugApi, ManagedBuffer<DebugApi>> = ManagedVec::new();

    managed_vec.push(ManagedBuffer::from("first"));
    managed_vec.push(ManagedBuffer::from("second"));
    managed_vec.push(ManagedBuffer::from("third"));

    let native = managed_vec.to_native();
    let expected = vec![
        String::from("first"),
        String::from("second"),
        String::from("third")
    ];

    assert_eq!(
        native,
        expected
    )
}