#![allow(clippy::generic_const_exprs)]
#![feature(generic_const_exprs)]

use multiversx_sc::{
    codec::test_util::{check_dep_encode_decode, check_top_encode_decode},
    types::{BigFloat, BigUint, ConstDecimals, FixedPoint, NumDecimals},
};
use multiversx_sc_scenario::api::StaticApi;

#[test]
pub fn test_fixed_point_biguint() {
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

    let fixed_4: FixedPoint<StaticApi, NumDecimals> =
        FixedPoint::from_raw_units(BigUint::from(100u64), 2usize);
    let fixed_5 = fixed_4.rescale(2usize);
    assert_eq!(
        fixed_5,
        FixedPoint::from_raw_units(BigUint::from(100000000u64), 8usize)
    );

    let fixed_6: FixedPoint<StaticApi, ConstDecimals<2usize>> =
        FixedPoint::from(BigUint::from(1500u64));
    let fixed_7 = fixed_6.rescale(ConstDecimals::<8>);
    assert_eq!(
        fixed_7,
        FixedPoint::<StaticApi, ConstDecimals<8>>::from(BigUint::from(1500u64))
    );

    let fixed_8: FixedPoint<StaticApi, NumDecimals> =
        FixedPoint::from_raw_units(BigUint::from(5u64), 5usize);
    let fixed_9 = fixed_8.rescale(ConstDecimals::<3>);
    assert_eq!(
        fixed_9,
        FixedPoint::<StaticApi, ConstDecimals<3>>::const_decimals_from_raw(BigUint::from(500u64))
    );

    let float_1 = BigFloat::<StaticApi>::from_frac(3i64, 2i64);
    let fixed_float_1 = FixedPoint::<StaticApi, ConstDecimals<1>>::from_big_float(
        float_1.clone(),
        ConstDecimals::<1>,
    );
    let fixed_float_2 = FixedPoint::<StaticApi, NumDecimals>::from_big_float(float_1, 1usize);

    assert_eq!(
        fixed_float_1,
        FixedPoint::<StaticApi, ConstDecimals<1>>::const_decimals_from_raw(BigUint::from(15u64))
    );
    assert_eq!(
        fixed_float_2,
        FixedPoint::<StaticApi, NumDecimals>::from_raw_units(BigUint::from(15u64), 1usize)
    );
}

#[test]
fn encode_decode_test() {
    let fixed_struct: FixedPoint<StaticApi, NumDecimals> =
        FixedPoint::from_raw_units(BigUint::from(1u64), 1usize);

    #[rustfmt::skip]
	let nested_bytes = &[
		/* BigUint */ 0, 0, 0, 0x01, 0x01,  
		/* usize */ 0, 0, 0, 0x01, 
	];

    check_dep_encode_decode(fixed_struct.clone(), nested_bytes);
    check_top_encode_decode(fixed_struct, nested_bytes);

    let fixed_const: FixedPoint<StaticApi, ConstDecimals<1>> =
        FixedPoint::const_decimals_from_raw(BigUint::from(1u64));

    #[rustfmt::skip]
    let bytes = &[
        /* BigUint */ 0x01,
    ];

    check_top_encode_decode(fixed_const, bytes);
}
