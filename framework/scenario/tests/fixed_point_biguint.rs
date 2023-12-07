#![allow(clippy::generic_const_exprs)]
#![feature(generic_const_exprs)]

use multiversx_sc::types::{BigUint, Convert, FixedPoint};
use multiversx_sc_scenario::api::StaticApi;

#[test]
fn test_addition() {
    let fixed = FixedPoint::<StaticApi, 2usize>::from(BigUint::from(1u64));
    let fixed_2 = FixedPoint::<StaticApi, 2usize>::from(BigUint::from(5u64));
    let fixed_3 = FixedPoint::<StaticApi, 4usize>::from(BigUint::from(8u64));

    let addition = fixed.clone() + fixed_2.clone();
    assert_eq!(
        addition,
        FixedPoint::<StaticApi, 2usize>::from(BigUint::from(6u64))
    );
    assert_eq!(addition.into_raw_units(), &BigUint::from(600u64));
    assert_eq!(addition.trunc(), BigUint::from(6u64));

    let subtraction = addition - fixed;
    assert_eq!(
        subtraction,
        FixedPoint::<StaticApi, 2usize>::from(BigUint::from(5u64))
    );

    let multiplication = fixed_3.clone() * fixed_2;
    assert_eq!(
        multiplication,
        FixedPoint::<StaticApi, 6usize>::from(BigUint::from(40u64))
    );

    let division = multiplication / fixed_3;
    assert_eq!(
        division,
        FixedPoint::<StaticApi, 2usize>::from(BigUint::from(5u64))
    );

    let fixed_4 = FixedPoint::<StaticApi, 8usize>::from(BigUint::from(5u64));
    let convert = FixedPoint::<StaticApi, 2usize>::convert_from(fixed_4);

    assert_eq!(
        convert,
        FixedPoint::<StaticApi, 2usize>::from(BigUint::from(5u64))
    );
}
