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
