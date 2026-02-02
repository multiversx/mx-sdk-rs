use multiversx_sc::types::{BigUint, NonZeroBigUint};
use multiversx_sc_scenario::api::StaticApi;

fn assert_non_zero_big_uint_proportion(total: u64, part: u64, denom: u64, expected: u64) {
    let total = NonZeroBigUint::<StaticApi>::new_or_panic(BigUint::<StaticApi>::from(total));
    let expected = NonZeroBigUint::<StaticApi>::new_or_panic(BigUint::<StaticApi>::from(expected));
    assert_eq!(total.proportion(part, denom), expected);
    assert_eq!(total.clone().into_proportion(part, denom), expected);
}

#[test]
fn test_non_zero_big_uint_proportion_all() {
    assert_non_zero_big_uint_proportion(1000, 25, 100, 250);
    assert_non_zero_big_uint_proportion(1000, 50, 100, 500);
    assert_non_zero_big_uint_proportion(1000, 75, 100, 750);
    assert_non_zero_big_uint_proportion(1000, 100, 100, 1000);
    assert_non_zero_big_uint_proportion(3333, 1, 3, 1111);
    assert_non_zero_big_uint_proportion(3333, 2, 3, 2222);
    assert_non_zero_big_uint_proportion(3, 1, 2, 1); // 3 * 1/2 = 1.5 -> 1
    assert_non_zero_big_uint_proportion(3, 3, 2, 4); // 3 * 3/2 = 4.5 -> 4
}

#[test]
#[should_panic]
fn test_non_zero_big_uint_proportion_panics_on_zero_result() {
    let total = NonZeroBigUint::<StaticApi>::new_or_panic(BigUint::<StaticApi>::from(1000u32));

    // This should panic because 0/100 * 1000 = 0, and NonZeroBigUint cannot be zero
    let _result = total.proportion(0u64, 100u64);
}

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
