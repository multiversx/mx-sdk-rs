use hmac::digest::generic_array::typenum::assert_type_eq;
use multiversx_sc::types::ManagedBuffer;
use multiversx_sc_codec::multi_types::OptionalValue;
use multiversx_sc_scenario::DebugApi;
use multiversx_sdk::data::types::native::NativeConvertible;

#[test]
fn test_none_option_to_native() {
    trait IsString {
        fn is_string(&self) -> bool;
    }

    impl IsString for Option<ManagedBuffer<DebugApi>> {
        fn is_string(&self) -> bool {
            false
        }
    }

    impl IsString for Option<String> {
        fn is_string(&self) -> bool {
            true
        }
    }

    let _ = DebugApi::dummy();
    let result: Option<ManagedBuffer<DebugApi>> = None;
    let native = result.to_native();

    assert!(!result.is_string());
    assert!(native.is_string());
    assert!(native.is_none());
}

#[test]
fn test_some_option_to_native() {
    let _ = DebugApi::dummy();
    let buffer: ManagedBuffer<DebugApi> = ManagedBuffer::from("some");
    let result: Option<ManagedBuffer<DebugApi>> = Some(buffer);
    let native = result.to_native();

    let expected_result = String::from("some");

    assert_eq!(
        native.unwrap(),
        expected_result
    );
}

#[test]
fn test_none_optional_value_to_native() {
    trait IsString {
        fn is_string(&self) -> bool;
    }

    impl IsString for OptionalValue<ManagedBuffer<DebugApi>> {
        fn is_string(&self) -> bool {
            false
        }
    }

    impl IsString for Option<String> {
        fn is_string(&self) -> bool {
            true
        }
    }

    let _ = DebugApi::dummy();
    let result: OptionalValue<ManagedBuffer<DebugApi>> = OptionalValue::None;
    let native = result.to_native();

    assert!(!result.is_string());
    assert!(native.is_string());
    assert!(native.is_none());
}

#[test]
fn test_some_optional_value_to_native() {
    let _ = DebugApi::dummy();
    let buffer: ManagedBuffer<DebugApi> = ManagedBuffer::from("some");
    let result: OptionalValue<ManagedBuffer<DebugApi>> = OptionalValue::Some(buffer);
    let native = result.to_native();

    let expected_result = String::from("some");

    assert_eq!(
        native.unwrap(),
        expected_result
    );
}