use core::num::NonZero;
use multiversx_sc::types::{BigUint, NonZeroBigUint};
use multiversx_sc_scenario::api::StaticApi;

#[test]
fn test_non_zero_big_uint_creation_and_conversion() {
    use multiversx_sc::types::ManagedBuffer;
    type NZ = NonZeroBigUint<StaticApi>;
    type BU = BigUint<StaticApi>;

    // Creation from non-zero BigUint
    let n1 = NZ::new(BU::from(42u32));
    assert!(n1.is_some());
    assert_eq!(n1.as_ref().unwrap().as_big_uint().to_u64(), Some(42));

    // Creation from zero BigUint
    let n0 = NZ::new(BU::from(0u32));
    assert!(n0.is_none());

    // Creation from u128 (non-zero)
    let n2: Result<NZ, _> = 123u128.try_into();
    assert!(n2.is_ok());
    assert_eq!(n2.unwrap().as_big_uint().to_u64(), Some(123));

    // Creation from u128 (zero)
    let n3: Result<NZ, _> = 0u128.try_into();
    assert!(n3.is_err());

    // Creation from ManagedBuffer (non-zero)
    let buf = ManagedBuffer::<StaticApi>::from(&[1u8, 0, 0, 0][..]);
    let n4: Result<NZ, _> = buf.clone().try_into();
    assert!(n4.is_ok());

    // Creation from ManagedBuffer (zero)
    let buf_zero = ManagedBuffer::<StaticApi>::from(&[0u8][..]);
    let n5: Result<NZ, _> = buf_zero.try_into();
    assert!(n5.is_err());

    // into_big_uint and as_big_uint
    let nz = NZ::new_or_panic(BU::from(7u32));
    let bu = nz.clone().into_big_uint();
    assert_eq!(bu.to_u64(), Some(7));
    assert_eq!(nz.as_big_uint().to_u64(), Some(7));
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
