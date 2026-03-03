use multiversx_sc::types::ManagedBuffer;
use multiversx_sc_scenario::api::StaticApi;

#[test]
fn test_managed_buffer_parse_as_i64_positive() {
    let mb = ManagedBuffer::<StaticApi>::from(&[0x7B][..]); // 123
    assert_eq!(mb.parse_as_i64(), Some(123));

    let mb = ManagedBuffer::<StaticApi>::from(&[0x01, 0xC8][..]); // 456
    assert_eq!(mb.parse_as_i64(), Some(456));
}

#[test]
fn test_managed_buffer_parse_as_i64_negative() {
    // -123 in two's complement (single byte)
    let mb = ManagedBuffer::<StaticApi>::from(&[0x85][..]); // -123
    assert_eq!(mb.parse_as_i64(), Some(-123));

    // -456 in two's complement (two bytes)
    let mb = ManagedBuffer::<StaticApi>::from(&[0xFE, 0x38][..]); // -456
    assert_eq!(mb.parse_as_i64(), Some(-456));
}

#[test]
fn test_managed_buffer_parse_as_i64_zero() {
    let mb = ManagedBuffer::<StaticApi>::new();
    assert_eq!(mb.parse_as_i64(), Some(0));

    let mb = ManagedBuffer::<StaticApi>::from(&[0x00][..]);
    assert_eq!(mb.parse_as_i64(), Some(0));
}

#[test]
fn test_managed_buffer_parse_as_i64_max_min() {
    // i64::MAX = 9223372036854775807 = 0x7FFFFFFFFFFFFFFF
    let mb =
        ManagedBuffer::<StaticApi>::from(&[0x7F, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF][..]);
    assert_eq!(mb.parse_as_i64(), Some(i64::MAX));

    // i64::MIN = -9223372036854775808 = 0x8000000000000000
    let mb =
        ManagedBuffer::<StaticApi>::from(&[0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00][..]);
    assert_eq!(mb.parse_as_i64(), Some(i64::MIN));
}

#[test]
fn test_managed_buffer_parse_as_i64_too_large() {
    // 9 bytes - should return None
    let mb = ManagedBuffer::<StaticApi>::from(
        &[0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00][..],
    );
    assert_eq!(mb.parse_as_i64(), None);
}

#[test]
fn test_managed_buffer_parse_as_i64_various_sizes() {
    // 1 byte
    let mb = ManagedBuffer::<StaticApi>::from(&[0x01][..]);
    assert_eq!(mb.parse_as_i64(), Some(1));

    // 2 bytes
    let mb = ManagedBuffer::<StaticApi>::from(&[0x01, 0x00][..]);
    assert_eq!(mb.parse_as_i64(), Some(256));

    // 4 bytes
    let mb = ManagedBuffer::<StaticApi>::from(&[0x00, 0x00, 0x10, 0x00][..]);
    assert_eq!(mb.parse_as_i64(), Some(4096));

    // 8 bytes
    let mb =
        ManagedBuffer::<StaticApi>::from(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00][..]);
    assert_eq!(mb.parse_as_i64(), Some(65536));
}

#[test]
fn test_managed_buffer_parse_as_u64_positive() {
    let mb = ManagedBuffer::<StaticApi>::from(&[0x7B][..]); // 123
    assert_eq!(mb.parse_as_u64(), Some(123));

    let mb = ManagedBuffer::<StaticApi>::from(&[0x01, 0xC8][..]); // 456
    assert_eq!(mb.parse_as_u64(), Some(456));

    let mb = ManagedBuffer::<StaticApi>::from(&[0x04, 0xD2][..]); // 1234
    assert_eq!(mb.parse_as_u64(), Some(1234));
}

#[test]
fn test_managed_buffer_parse_as_u64_zero() {
    let mb = ManagedBuffer::<StaticApi>::new();
    assert_eq!(mb.parse_as_u64(), Some(0));

    let mb = ManagedBuffer::<StaticApi>::from(&[0x00][..]);
    assert_eq!(mb.parse_as_u64(), Some(0));
}

#[test]
fn test_managed_buffer_parse_as_u64_max() {
    // u64::MAX = 18446744073709551615 = 0xFFFFFFFFFFFFFFFF
    let mb =
        ManagedBuffer::<StaticApi>::from(&[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF][..]);
    assert_eq!(mb.parse_as_u64(), Some(u64::MAX));
}

#[test]
fn test_managed_buffer_parse_as_u64_too_large() {
    // 9 bytes - should return None
    let mb = ManagedBuffer::<StaticApi>::from(
        &[0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00][..],
    );
    assert_eq!(mb.parse_as_u64(), None);
}

#[test]
fn test_managed_buffer_parse_as_u64_various_sizes() {
    // 1 byte
    let mb = ManagedBuffer::<StaticApi>::from(&[0x01][..]);
    assert_eq!(mb.parse_as_u64(), Some(1));

    let mb = ManagedBuffer::<StaticApi>::from(&[0xFF][..]); // 255
    assert_eq!(mb.parse_as_u64(), Some(255));

    // 2 bytes
    let mb = ManagedBuffer::<StaticApi>::from(&[0x01, 0x00][..]);
    assert_eq!(mb.parse_as_u64(), Some(256));

    let mb = ManagedBuffer::<StaticApi>::from(&[0xFF, 0xFF][..]); // 65535
    assert_eq!(mb.parse_as_u64(), Some(65535));

    // 4 bytes
    let mb = ManagedBuffer::<StaticApi>::from(&[0x00, 0x00, 0x10, 0x00][..]);
    assert_eq!(mb.parse_as_u64(), Some(4096));

    let mb = ManagedBuffer::<StaticApi>::from(&[0xFF, 0xFF, 0xFF, 0xFF][..]); // 4294967295
    assert_eq!(mb.parse_as_u64(), Some(4294967295));

    // 8 bytes
    let mb =
        ManagedBuffer::<StaticApi>::from(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00][..]);
    assert_eq!(mb.parse_as_u64(), Some(65536));

    let mb =
        ManagedBuffer::<StaticApi>::from(&[0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00][..]);
    assert_eq!(mb.parse_as_u64(), Some(4294967296)); // 2^32
}
