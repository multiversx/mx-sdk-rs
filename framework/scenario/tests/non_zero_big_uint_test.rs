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
