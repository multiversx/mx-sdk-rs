#![feature(generic_const_exprs)]

use multiversx_sc::types::{BigUint, FixedPoint};
use multiversx_sc_scenario::api::StaticApi;

#[test]
fn test_addition() {
    let fixed = FixedPoint::<StaticApi, 2usize>::from(BigUint::from(1u64));
    let foo = FixedPoint::<StaticApi, 2usize>::from(BigUint::from(5u64));
    let trace = FixedPoint::<StaticApi, 4usize>::from(BigUint::from(8u64));

    let addition = fixed.clone() + foo.clone();
    assert_eq!(
        addition,
        FixedPoint::<StaticApi, 2usize>::from(BigUint::from(6u64))
    );
    assert_eq!(addition.data(), &BigUint::from(600u64));
    assert_eq!(addition.trunc(), BigUint::from(6u64));

    let subtraction = addition - fixed;
    assert_eq!(
        subtraction,
        FixedPoint::<StaticApi, 2usize>::from(BigUint::from(5u64))
    );

    let multiplication = trace.clone() * foo;
    assert_eq!(
        multiplication,
        FixedPoint::<StaticApi, 6usize>::from(BigUint::from(40u64))
    );

    let division = multiplication / trace;
    assert_eq!(
        division,
        FixedPoint::<StaticApi, 2usize>::from(BigUint::from(5u64))
    );
}
