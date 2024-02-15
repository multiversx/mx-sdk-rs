#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use multiversx_sc::{
    codec::test_util::{check_dep_encode_decode, check_top_encode_decode},
    types::{BigFloat, BigUint, ConstDecimals, ManagedDecimal, NumDecimals},
};
use multiversx_sc_scenario::api::StaticApi;

#[test]
pub fn test_managed_decimal() {
    let fixed = ManagedDecimal::<StaticApi, ConstDecimals<2>>::from(BigUint::from(1u64));
    let fixed_2 = ManagedDecimal::<StaticApi, ConstDecimals<2>>::from(BigUint::from(5u64));
    let fixed_3 = ManagedDecimal::<StaticApi, ConstDecimals<4>>::from(BigUint::from(8u64));

    let addition = fixed.clone() + fixed_2.clone();
    assert_eq!(
        addition,
        ManagedDecimal::<StaticApi, ConstDecimals<2>>::from(BigUint::from(6u64))
    );
    assert_eq!(addition.into_raw_units(), &BigUint::from(600u64));
    assert_eq!(addition.trunc(), BigUint::from(6u64));

    let subtraction = addition - fixed;
    assert_eq!(
        subtraction,
        ManagedDecimal::<StaticApi, ConstDecimals<2>>::from(BigUint::from(5u64))
    );

    let multiplication = fixed_3.clone() * fixed_2;
    assert_eq!(
        multiplication,
        ManagedDecimal::<StaticApi, ConstDecimals<6>>::from(BigUint::from(40u64))
    );

    let division = multiplication / fixed_3;
    assert_eq!(
        division,
        ManagedDecimal::<StaticApi, ConstDecimals<2>>::from(BigUint::from(5u64))
    );

    let fixed_4: ManagedDecimal<StaticApi, NumDecimals> =
        ManagedDecimal::from_raw_units(BigUint::from(100u64), 2usize);
    let fixed_5 = fixed_4.rescale(2usize);
    assert_eq!(
        fixed_5,
        ManagedDecimal::from_raw_units(BigUint::from(100000000u64), 8usize)
    );

    let fixed_6: ManagedDecimal<StaticApi, ConstDecimals<2>> =
        ManagedDecimal::from(BigUint::from(1500u64));
    let fixed_7 = fixed_6.rescale(ConstDecimals::<8>);
    assert_eq!(
        fixed_7,
        ManagedDecimal::<StaticApi, ConstDecimals<8>>::from(BigUint::from(1500u64))
    );

    let fixed_8: ManagedDecimal<StaticApi, NumDecimals> =
        ManagedDecimal::from_raw_units(BigUint::from(5u64), 5usize);
    let fixed_9 = fixed_8.rescale(ConstDecimals::<3>);
    assert_eq!(
        fixed_9,
        ManagedDecimal::<StaticApi, ConstDecimals<3>>::const_decimals_from_raw(BigUint::from(
            500u64
        ))
    );

    let float_1 = BigFloat::<StaticApi>::from_frac(3i64, 2i64);
    let fixed_float_1 = ManagedDecimal::<StaticApi, ConstDecimals<1>>::from_big_float(
        float_1.clone(),
        ConstDecimals::<1>,
    );
    let fixed_float_2 = ManagedDecimal::<StaticApi, NumDecimals>::from_big_float(float_1, 1usize);

    assert_eq!(
        fixed_float_1,
        ManagedDecimal::<StaticApi, ConstDecimals<1>>::const_decimals_from_raw(BigUint::from(
            15u64
        ))
    );
    assert_eq!(
        fixed_float_2,
        ManagedDecimal::<StaticApi, NumDecimals>::from_raw_units(BigUint::from(15u64), 1usize)
    );
}

#[test]
fn test_managed_decimal_conversion() {
    let fixed: ManagedDecimal<StaticApi, NumDecimals> =
        ManagedDecimal::from_raw_units(BigUint::from(123456789123456789u64), 15usize); //123,45....

    let float_coresp = fixed.to_big_float();

    // hook not available yet, uncomment when available
    // assert_eq!(
    //     float_coresp.to_buffer(),
    //     ManagedBuffer::from("123.456789123456789")
    // );

    assert_eq!(
        float_coresp,
        BigFloat::from_frac(123456789123456789i64, 1_000_000_000_000_000i64)
    );
}

#[test]
fn test_encode_decode() {
    let fixed_struct: ManagedDecimal<StaticApi, NumDecimals> =
        ManagedDecimal::from_raw_units(BigUint::from(1u64), 1usize);

    #[rustfmt::skip]
	let nested_bytes = &[
		/* BigUint */ 0, 0, 0, 0x01, 0x01,  
		/* usize */ 0, 0, 0, 0x01, 
	];

    check_dep_encode_decode(fixed_struct.clone(), nested_bytes);
    check_top_encode_decode(fixed_struct, nested_bytes);

    let fixed_const: ManagedDecimal<StaticApi, ConstDecimals<1>> =
        ManagedDecimal::const_decimals_from_raw(BigUint::from(1u64));

    #[rustfmt::skip]
    let bytes = &[
        /* BigUint */ 0x01,
    ];

    check_top_encode_decode(fixed_const, bytes);
}
