use multiversx_sc::types::{ManagedBuffer, TokenIdentifier};
use multiversx_sc_scenario::DebugApi;
use multiversx_sdk::data::types::native::NativeConvertible;

#[test]
fn test_managed_buffer_to_native() {
    let _ = DebugApi::dummy();
    let buffer: ManagedBuffer<DebugApi> = ManagedBuffer::from("This is a buffer");
    let native = buffer.to_native();

    let expected = String::from("This is a buffer");

    assert_eq!(
        native,
        expected
    );
}

#[test]
fn test_token_identifier_to_native() {
    let _ = DebugApi::dummy();
    let buffer: TokenIdentifier<DebugApi> = TokenIdentifier::from("WEGLD-abcdef");
    let native = buffer.to_native();

    let expected = String::from("WEGLD-abcdef");

    assert_eq!(
        native,
        expected
    );
}