use core::num::NonZero;
use multiversx_sc::types::{BigUint, NonZeroBigUint};
use multiversx_sc_scenario::api::StaticApi;

#[test]
fn test_non_zero_big_uint_creation_and_conversion() {
    use multiversx_sc::types::ManagedBuffer;
    type NZ = NonZeroBigUint<StaticApi>;
    type BU = BigUint<StaticApi>;

    // Creation from non-zero BigUint
    assert_eq!(
        NZ::new(BU::from(42u32)).unwrap().as_big_uint().to_u64(),
        Some(42)
    );

    // Creation from zero BigUint
    assert!(NZ::new(BU::from(0u32)).is_none());

    // Creation from u8 (non-zero)
    assert_eq!(
        TryInto::<NZ>::try_into(42u8)
            .unwrap()
            .as_big_uint()
            .to_u64(),
        Some(42)
    );

    // One
    assert_eq!(NZ::one().as_big_uint().to_u64(), Some(1));

    // Creation from u8 (zero)
    assert!(TryInto::<NZ>::try_into(0u8).is_err());

    // Creation from u16 (non-zero)
    assert_eq!(
        TryInto::<NZ>::try_into(12345u16)
            .unwrap()
            .as_big_uint()
            .to_u64(),
        Some(12345)
    );

    // Creation from u16 (zero)
    assert!(TryInto::<NZ>::try_into(0u16).is_err());

    // Creation from u32 (non-zero)
    assert_eq!(
        TryInto::<NZ>::try_into(987654u32)
            .unwrap()
            .as_big_uint()
            .to_u64(),
        Some(987654)
    );

    // Creation from u32 (zero)
    assert!(TryInto::<NZ>::try_into(0u32).is_err());

    // Creation from u64 (non-zero)
    assert_eq!(
        TryInto::<NZ>::try_into(123456789u64)
            .unwrap()
            .as_big_uint()
            .to_u64(),
        Some(123456789)
    );

    // Creation from u64 (zero)
    assert!(TryInto::<NZ>::try_into(0u64).is_err());

    // Creation from u128 (non-zero)
    assert_eq!(
        TryInto::<NZ>::try_into(123u128)
            .unwrap()
            .as_big_uint()
            .to_u64(),
        Some(123)
    );

    // Creation from u128 (zero)
    assert!(TryInto::<NZ>::try_into(0u128).is_err());

    // Creation from usize (non-zero)
    assert_eq!(
        TryInto::<NZ>::try_into(12345usize)
            .unwrap()
            .as_big_uint()
            .to_u64(),
        Some(12345)
    );

    // Creation from usize (zero)
    assert!(TryInto::<NZ>::try_into(0usize).is_err());

    // Creation from ManagedBuffer (non-zero)
    assert!(TryInto::<NZ>::try_into(ManagedBuffer::<StaticApi>::from(&[1u8, 0, 0, 0][..])).is_ok());

    // Creation from ManagedBuffer (zero)
    assert!(TryInto::<NZ>::try_into(ManagedBuffer::<StaticApi>::from(&[0u8][..])).is_err());

    // into_big_uint and as_big_uint
    assert_eq!(
        NZ::new_or_panic(BU::from(7u32))
            .clone()
            .into_big_uint()
            .to_u64(),
        Some(7)
    );
    assert_eq!(
        NZ::new_or_panic(BU::from(7u32)).as_big_uint().to_u64(),
        Some(7)
    );
}

#[test]
#[should_panic]
fn test_non_zero_big_uint_new_or_panic_panics_on_zero() {
    let _ = NonZeroBigUint::<StaticApi>::new_or_panic(BigUint::<StaticApi>::from(0u32));
}

#[test]
fn test_non_zero_big_uint_from_nonzero_types() {
    type NZ = NonZeroBigUint<StaticApi>;

    // Test From<NonZero<u8>>
    let nz_u8 = NonZero::<u8>::new(255).unwrap();
    let nz_big_uint = NZ::from(nz_u8);
    assert_eq!(nz_big_uint.as_big_uint().to_u64(), Some(255));

    // Test From<NonZero<u16>>
    let nz_u16 = NonZero::<u16>::new(65535).unwrap();
    let nz_big_uint = NZ::from(nz_u16);
    assert_eq!(nz_big_uint.as_big_uint().to_u64(), Some(65535));

    // Test From<NonZero<u32>>
    let nz_u32 = NonZero::<u32>::new(4294967295).unwrap();
    let nz_big_uint = NZ::from(nz_u32);
    assert_eq!(nz_big_uint.as_big_uint().to_u64(), Some(4294967295));

    // Test From<NonZero<u64>>
    let nz_u64 = NonZero::<u64>::new(i64::MAX as u64).unwrap();
    let nz_big_uint = NZ::from(nz_u64);
    assert_eq!(nz_big_uint.as_big_uint().to_u64(), Some(i64::MAX as u64));

    // Test From<NonZero<u128>>
    let nz_u128 = NonZero::<u128>::new(123456789012345678901234567890).unwrap();
    let nz_big_uint = NZ::from(nz_u128);
    // For u128, we can't use to_u64(), so we'll convert back and compare
    let converted_back = nz_big_uint.as_big_uint().to_bytes_be();
    let expected = BigUint::<StaticApi>::from(123456789012345678901234567890u128).to_bytes_be();
    assert_eq!(converted_back.as_slice(), expected.as_slice());

    // Test From<NonZero<usize>>
    let nz_usize = NonZero::<usize>::new(12345).unwrap();
    let nz_big_uint = NZ::from(nz_usize);
    assert_eq!(nz_big_uint.as_big_uint().to_u64(), Some(12345));

    // Test with small values to ensure they're handled correctly
    let nz_u8_small = NonZero::<u8>::new(1).unwrap();
    let nz_big_uint_small = NZ::from(nz_u8_small);
    assert_eq!(nz_big_uint_small.as_big_uint().to_u64(), Some(1));
}
