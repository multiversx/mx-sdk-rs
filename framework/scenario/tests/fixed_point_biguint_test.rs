#![allow(clippy::generic_const_exprs)]
#![feature(generic_const_exprs)]

use multiversx_sc::types::{BigUint, ConstDecimals, FixedPoint, NumDecimals};
use multiversx_sc_scenario::api::StaticApi;

#[test]
fn test_fixed_point_biguint() {
    let fixed = FixedPoint::<StaticApi, ConstDecimals<2usize>>::from(BigUint::from(1u64));
    let fixed_2 = FixedPoint::<StaticApi, ConstDecimals<2usize>>::from(BigUint::from(5u64));
    let fixed_3 = FixedPoint::<StaticApi, ConstDecimals<4usize>>::from(BigUint::from(8u64));

    let addition = fixed.clone() + fixed_2.clone();
    assert_eq!(
        addition,
        FixedPoint::<StaticApi, ConstDecimals<2usize>>::from(BigUint::from(6u64))
    );
    assert_eq!(addition.into_raw_units(), &BigUint::from(600u64));
    assert_eq!(addition.trunc(), BigUint::from(6u64));

    let subtraction = addition - fixed;
    assert_eq!(
        subtraction,
        FixedPoint::<StaticApi, ConstDecimals<2usize>>::from(BigUint::from(5u64))
    );

    let multiplication = fixed_3.clone() * fixed_2;
    assert_eq!(
        multiplication,
        FixedPoint::<StaticApi, ConstDecimals<6usize>>::from(BigUint::from(40u64))
    );

    let division = multiplication / fixed_3;
    assert_eq!(
        division,
        FixedPoint::<StaticApi, ConstDecimals<2usize>>::from(BigUint::from(5u64))
    );

    let fixed_4: FixedPoint<StaticApi, NumDecimals> = FixedPoint {
        data: BigUint::from(100u64), // 1 * 10^2
        decimals: 2usize,            // 10^2
    };
    let fixed_5 = fixed_4.rescale(8usize);
    assert_eq!(
        fixed_5,
        FixedPoint::from_raw_units(BigUint::from(100000000u64), 8usize)
    ); //works

    // let fixed_6: FixedPoint<StaticApi, ConstDecimals<2usize>> =
    //     FixedPoint::from(BigUint::from(1500u64)); //15 * 10^2
    // let fixed_7 = fixed_6.rescale(ConstDecimals<2usize>);
    // assert_eq!(fixed_7, FixedPoint::from_raw_units(BigUint::from(150u64), 1usize));

}
