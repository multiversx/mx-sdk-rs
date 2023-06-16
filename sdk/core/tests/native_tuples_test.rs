use multiversx_sc::types::ManagedBuffer;
use multiversx_sc_scenario::DebugApi;
use multiversx_sdk::data::types::native::NativeConvertible;

#[test]
fn test_tuple_2_to_native() {
    let _ = DebugApi::dummy();
    let buffer1: ManagedBuffer<DebugApi> = ManagedBuffer::from("first");
    let buffer2: ManagedBuffer<DebugApi> = ManagedBuffer::from("second");

    let tuple = (buffer1, buffer2);
    let native = tuple.to_native();

    let expected_result = (String::from("first"), String::from("second"));

    assert_eq!(
        native,
        expected_result
    );
}

#[test]
fn test_tuple_7_to_native() {
    let _ = DebugApi::dummy();
    let buffer1: ManagedBuffer<DebugApi> = ManagedBuffer::from("first");
    let buffer2: ManagedBuffer<DebugApi> = ManagedBuffer::from("second");
    let buffer3: ManagedBuffer<DebugApi> = ManagedBuffer::from("third");
    let buffer4: ManagedBuffer<DebugApi> = ManagedBuffer::from("fourth");
    let buffer5: ManagedBuffer<DebugApi> = ManagedBuffer::from("fifth");
    let buffer6: ManagedBuffer<DebugApi> = ManagedBuffer::from("sixth");
    let buffer7: ManagedBuffer<DebugApi> = ManagedBuffer::from("seventh");

    let tuple = (
        buffer1,
        buffer2,
        buffer3,
        buffer4,
        buffer5,
        buffer6,
        buffer7,
    );
    let native = tuple.to_native();

    let expected_result = (
        String::from("first"),
        String::from("second"),
        String::from("third"),
        String::from("fourth"),
        String::from("fifth"),
        String::from("sixth"),
        String::from("seventh"),
    );

    assert_eq!(
        native,
        expected_result
    );
}