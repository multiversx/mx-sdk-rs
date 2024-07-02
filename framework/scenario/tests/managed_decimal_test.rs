use multiversx_sc::{
    codec::test_util::{check_dep_encode_decode, check_top_encode_decode},
    derive::{debug_const_managed_decimal, debug_managed_decimal},
    types::{
        BigFloat, BigInt, BigUint, ConstDecimals, ManagedDecimal, ManagedDecimalSigned, NumDecimals,
    },
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
    let fixed_float_1 = ManagedDecimalSigned::<StaticApi, ConstDecimals<1>>::from_big_float(
        &float_1,
        ConstDecimals::<1>,
    );
    let fixed_float_2 =
        ManagedDecimalSigned::<StaticApi, NumDecimals>::from_big_float(&float_1, 1usize);

    assert_eq!(
        fixed_float_1,
        ManagedDecimalSigned::<StaticApi, ConstDecimals<1>>::const_decimals_from_raw(BigInt::from(
            15
        ))
    );
    assert_eq!(
        fixed_float_2,
        ManagedDecimalSigned::<StaticApi, NumDecimals>::from_raw_units(BigInt::from(15), 1usize)
    );
}

#[test]
fn test_managed_decimal_macros() {
    let small = debug_managed_decimal!("3.1");
    assert_eq!(small.scale(), 1usize);
    assert_eq!(small.into_raw_units(), &BigUint::from(31u64));
    assert_eq!(&small.trunc(), &BigUint::from(3u64));

    let three = debug_const_managed_decimal!("1.654");
    assert_eq!(three.scale(), 3usize);

    let four = debug_managed_decimal!("89632.2223");
    assert_eq!(four.scale(), 4usize);

    let huge = debug_const_managed_decimal!("8723.283764652365232");
    assert_eq!(huge.scale(), 15usize);
    assert_eq!(
        huge.into_raw_units(),
        &BigUint::from(8723283764652365232u64)
    );
    assert_eq!(&huge.trunc(), &BigUint::from(8723u64));
}

#[test]
fn test_managed_decimal_conversion() {
    let fixed: ManagedDecimalSigned<StaticApi, NumDecimals> =
        ManagedDecimalSigned::from_raw_units(BigInt::from(123456789123456789i64), 15usize);
    //123,45....
    let float_coresp = fixed.to_big_float();

    // hook not available yet, uncomment when available
    // assert_eq!(
    //     float_coresp.to_buffer(),
    //     ManagedBuffer::from("123.456789123456789")
    // );

    assert_eq!(
        float_coresp,
        BigFloat::from_frac(123456789123456789i64, 1_000_000_000_000_000i64),
    );
}

#[test]
pub fn test_addition_managed_decimal_signed() {
    let fixed_1 = ManagedDecimalSigned::<StaticApi, ConstDecimals<2>>::from(BigInt::from(1i64));
    let fixed_2 = ManagedDecimalSigned::<StaticApi, ConstDecimals<2>>::from(BigInt::from(3i64));
    let fixed_3 = ManagedDecimalSigned::<StaticApi, ConstDecimals<2>>::from(BigInt::from(-5i64));
    let fixed_4 = ManagedDecimalSigned::<StaticApi, ConstDecimals<2>>::from(BigInt::from(-2i64));

    let addition_1 = fixed_1.clone() + fixed_2.clone();
    assert_eq!(
        addition_1,
        ManagedDecimalSigned::<StaticApi, ConstDecimals<2>>::from(BigInt::from(4i64))
    );
    assert_eq!(addition_1.into_raw_units(), &BigInt::from(400i64));
    assert_eq!(addition_1.trunc(), BigInt::from(4i64));

    let addition_2 = fixed_1.clone() + fixed_3.clone();
    assert_eq!(
        addition_2,
        ManagedDecimalSigned::<StaticApi, ConstDecimals<2>>::from(BigInt::from(-4i64))
    );
    assert_eq!(addition_2.into_raw_units(), &BigInt::from(-400i64));
    assert_eq!(addition_2.trunc(), BigInt::from(-4i64));

    let addition_3 = fixed_3.clone() + fixed_4.clone();
    assert_eq!(
        addition_3,
        ManagedDecimalSigned::<StaticApi, ConstDecimals<2>>::from(BigInt::from(-7i64))
    );
    assert_eq!(addition_3.into_raw_units(), &BigInt::from(-700i64));
    assert_eq!(addition_3.trunc(), BigInt::from(-7i64));

    let addition_4 = fixed_4.clone() + fixed_2.clone();
    assert_eq!(
        addition_4,
        ManagedDecimalSigned::<StaticApi, ConstDecimals<2>>::from(BigInt::from(1i64))
    );
    assert_eq!(addition_4.into_raw_units(), &BigInt::from(100i64));
    assert_eq!(addition_4.trunc(), BigInt::from(1i64));
}

#[test]
pub fn test_substraction_managed_decimal_signed() {
    let fixed_1 = ManagedDecimalSigned::<StaticApi, ConstDecimals<2>>::from(BigInt::from(1i64));
    let fixed_2 = ManagedDecimalSigned::<StaticApi, ConstDecimals<2>>::from(BigInt::from(3i64));
    let fixed_3 = ManagedDecimalSigned::<StaticApi, ConstDecimals<2>>::from(BigInt::from(-5i64));
    let fixed_4 = ManagedDecimalSigned::<StaticApi, ConstDecimals<2>>::from(BigInt::from(-2i64));

    let substraction_1 = fixed_2.clone() - fixed_1.clone();
    assert_eq!(
        substraction_1,
        ManagedDecimalSigned::<StaticApi, ConstDecimals<2>>::from(BigInt::from(2i64))
    );
    assert_eq!(substraction_1.into_raw_units(), &BigInt::from(200i64));
    assert_eq!(substraction_1.trunc(), BigInt::from(2i64));

    let substraction_2 = fixed_1.clone() - fixed_2.clone();
    assert_eq!(
        substraction_2,
        ManagedDecimalSigned::<StaticApi, ConstDecimals<2>>::from(BigInt::from(-2i64))
    );
    assert_eq!(substraction_2.into_raw_units(), &BigInt::from(-200i64));
    assert_eq!(substraction_2.trunc(), BigInt::from(-2i64));

    let substraction_3 = substraction_2 - fixed_3.clone();
    assert_eq!(
        substraction_3,
        ManagedDecimalSigned::<StaticApi, ConstDecimals<2>>::from(BigInt::from(3i64))
    );
    assert_eq!(substraction_3.into_raw_units(), &BigInt::from(300i64));
    assert_eq!(substraction_3.trunc(), BigInt::from(3i64));

    let substraction_4 = fixed_3.clone() - fixed_4.clone();
    assert_eq!(
        substraction_4,
        ManagedDecimalSigned::<StaticApi, ConstDecimals<2>>::from(BigInt::from(-3i64))
    );
    assert_eq!(substraction_4.into_raw_units(), &BigInt::from(-300i64));
    assert_eq!(substraction_4.trunc(), BigInt::from(-3i64));
}

#[test]
pub fn test_multiplication_managed_decimal_signed() {
    let fixed_1 = ManagedDecimalSigned::<StaticApi, ConstDecimals<2>>::from(BigInt::from(1i64));
    let fixed_2 = ManagedDecimalSigned::<StaticApi, ConstDecimals<2>>::from(BigInt::from(3i64));
    let fixed_3 = ManagedDecimalSigned::<StaticApi, ConstDecimals<2>>::from(BigInt::from(-5i64));
    let fixed_4 = ManagedDecimalSigned::<StaticApi, ConstDecimals<2>>::from(BigInt::from(-2i64));

    let multiplication_1 = fixed_1.clone() * fixed_2.clone();
    assert_eq!(
        multiplication_1,
        ManagedDecimalSigned::<StaticApi, ConstDecimals<6>>::from(BigInt::from(3i64))
    );

    let multiplication_2 = fixed_3.clone() * fixed_2.clone();
    assert_eq!(
        multiplication_2,
        ManagedDecimalSigned::<StaticApi, ConstDecimals<6>>::from(BigInt::from(-15i64))
    );

    let multiplication_2 = fixed_3.clone() * fixed_4.clone();
    assert_eq!(
        multiplication_2,
        ManagedDecimalSigned::<StaticApi, ConstDecimals<6>>::from(BigInt::from(10i64))
    );
}

#[test]
pub fn test_devision_managed_decimal_signed() {
    let fixed_1 = ManagedDecimalSigned::<StaticApi, ConstDecimals<2>>::from(BigInt::from(6i64));
    let fixed_2 = ManagedDecimalSigned::<StaticApi, ConstDecimals<2>>::from(BigInt::from(2i64));
    let fixed_3 = ManagedDecimalSigned::<StaticApi, ConstDecimals<2>>::from(BigInt::from(-8i64));
    let fixed_4 = ManagedDecimalSigned::<StaticApi, ConstDecimals<2>>::from(BigInt::from(-2i64));

    let division_1 = fixed_1.clone() / fixed_2;
    assert_eq!(
        division_1,
        ManagedDecimalSigned::<StaticApi, ConstDecimals<2>>::from(BigInt::from(3i64))
    );

    let division_2 = fixed_1 / fixed_4.clone();
    assert_eq!(
        division_2,
        ManagedDecimalSigned::<StaticApi, ConstDecimals<2>>::from(BigInt::from(-3i64))
    );

    let division_3 = fixed_3 / fixed_4;
    assert_eq!(
        division_3,
        ManagedDecimalSigned::<StaticApi, ConstDecimals<2>>::from(BigInt::from(4i64))
    );
}

#[test]
pub fn test_rescale_managed_decimal_signed() {
    let fixed_1: ManagedDecimalSigned<StaticApi, NumDecimals> =
        ManagedDecimalSigned::from_raw_units(BigInt::from(-10000i64), 2usize);

    let fixed_2 = fixed_1.rescale(3usize);
    assert_eq!(
        fixed_2,
        ManagedDecimalSigned::from_raw_units(BigInt::from(-100000000i64), 6usize)
    );

    let fixed_3: ManagedDecimalSigned<StaticApi, ConstDecimals<2>> =
        ManagedDecimalSigned::from(BigInt::from(-1500i64));
    let fixed_4 = fixed_3.rescale(ConstDecimals::<8>);
    assert_eq!(fixed_4.into_raw_units(), &BigInt::from(-150000000000i64));

    let fixed_5: ManagedDecimalSigned<StaticApi, NumDecimals> =
        ManagedDecimalSigned::from_raw_units(BigInt::from(-5i64), 5usize);
    let fixed_6 = fixed_5.rescale(ConstDecimals::<3>);
    assert_eq!(
        fixed_6,
        ManagedDecimalSigned::<StaticApi, ConstDecimals<3>>::const_decimals_from_raw(BigInt::from(
            -500i64
        ))
    );

    let float_1 = BigFloat::<StaticApi>::from_frac(-3i64, 2i64);
    let fixed_float_1 = ManagedDecimalSigned::<StaticApi, ConstDecimals<1>>::from_big_float(
        &float_1,
        ConstDecimals::<1>,
    );
    let fixed_float_2 =
        ManagedDecimalSigned::<StaticApi, NumDecimals>::from_big_float(&float_1, 1usize);

    assert_eq!(
        fixed_float_1,
        ManagedDecimalSigned::<StaticApi, ConstDecimals<1>>::const_decimals_from_raw(BigInt::from(
            -15
        ))
    );
    assert_eq!(
        fixed_float_2,
        ManagedDecimalSigned::<StaticApi, NumDecimals>::from_raw_units(BigInt::from(-15), 1usize)
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

#[test]
fn test_managed_decimal_ln() {
    let fixed =
        ManagedDecimal::<StaticApi, NumDecimals>::from_raw_units(BigUint::from(23u64), 0usize);
    let ln_fixed = fixed.ln().unwrap(); // precision of 9 decimal points

    assert_eq!(ln_fixed.to_string(), "3.135514649");

    let const_dec = ManagedDecimal::<StaticApi, ConstDecimals<0>>::const_decimals_from_raw(
        BigUint::from(29299837u64),
    );
    let ln_const = const_dec.ln().unwrap();

    assert_eq!(ln_const.to_string(), "17.193072541");

    let small =
        ManagedDecimal::<StaticApi, ConstDecimals<1>>::const_decimals_from_raw(BigUint::from(1u64)); // 0.1
    let ln_small = small.ln().unwrap();

    assert_eq!(ln_small.to_string(), "-2.302524494");

    let v_small =
        ManagedDecimal::<StaticApi, ConstDecimals<2>>::const_decimals_from_raw(BigUint::from(1u64));
    // 0.01
    let ln_v_small = v_small.ln().unwrap();

    assert_eq!(ln_v_small.to_string(), "-4.605109587");

    let smallest =
        ManagedDecimal::<StaticApi, ConstDecimals<6>>::const_decimals_from_raw(BigUint::from(1u64));
    // 0.000001
    let ln_smallest = smallest.ln().unwrap();

    assert_eq!(ln_smallest.to_string(), "-13.815449959");

    let frac = ManagedDecimal::<StaticApi, ConstDecimals<2>>::const_decimals_from_raw(
        BigUint::from(322u64),
    );
    // 3.22
    let ln_frac = frac.ln().unwrap();

    assert_eq!(ln_frac.to_string(), "1.169428520");

    let frac =
        ManagedDecimal::<StaticApi, NumDecimals>::from_raw_units(BigUint::from(288221u64), 3usize);
    // 288.221
    let ln_frac = frac.ln().unwrap();

    assert_eq!(ln_frac.to_string(), "5.663669039");

    let frac = ManagedDecimal::<StaticApi, NumDecimals>::from_raw_units(
        BigUint::from(288211000000u64),
        9usize,
    );
    // 288.211000000
    let ln_frac = frac.ln().unwrap();

    assert_eq!(ln_frac.to_string(), "5.663649649");
}

// #[test]
// fn test_managed_decimal_log2() {
//     let fixed =
//         ManagedDecimal::<StaticApi, NumDecimals>::from_raw_units(BigUint::from(5u64), 0usize);
//     let ln_fixed = fixed.log2().unwrap();

//     assert_eq!(ln_fixed.to_string(), "3.135514649");

//     let const_dec = ManagedDecimal::<StaticApi, ConstDecimals<0>>::const_decimals_from_raw(
//         BigUint::from(29299837u64),
//     );
//     let ln_const = const_dec.log2().unwrap();

//     assert_eq!(ln_const.to_string(), "17.193072541");

//     let small =
//         ManagedDecimal::<StaticApi, ConstDecimals<1>>::const_decimals_from_raw(BigUint::from(1u64)); // 0.1
//     let ln_small = small.log2().unwrap();

//     assert_eq!(ln_small.to_string(), "-2.302524494");

//     let v_small =
//         ManagedDecimal::<StaticApi, ConstDecimals<2>>::const_decimals_from_raw(BigUint::from(1u64));
//     // 0.01
//     let ln_v_small = v_small.log2().unwrap();

//     assert_eq!(ln_v_small.to_string(), "-4.605109587");

//     let smallest =
//         ManagedDecimal::<StaticApi, ConstDecimals<6>>::const_decimals_from_raw(BigUint::from(1u64));
//     // 0.000001
//     let ln_smallest = smallest.log2().unwrap();

//     assert_eq!(ln_smallest.to_string(), "-13.815510557");
// }

// #[test]
// fn test_managed_decimal_nth_root() {
//     let fixed =
//         ManagedDecimal::<StaticApi, NumDecimals>::from_raw_units(BigUint::from(567u64), 1usize);

//     let fifth_root = fixed.clone().root(5f64, 100usize);
//     let fifth_root_higher_prec = fixed.root(5f64, 100_000usize);

//     assert_eq!(
//         fifth_root,
//         ManagedDecimal::<StaticApi, NumDecimals>::from_raw_units(BigUint::from(355u64), 100usize)
//     );
//     assert_eq!(
//         fifth_root_higher_prec,
//         ManagedDecimal::<StaticApi, NumDecimals>::from_raw_units(
//             BigUint::from(355399u64),
//             100_000usize
//         )
//     );

//     let const_fixed: ManagedDecimal<StaticApi, ConstDecimals<2>> =
//         ManagedDecimal::<StaticApi, ConstDecimals<2>>::const_decimals_from_raw(BigUint::from(
//             876u64,
//         ));

//     let seventh_root_var = const_fixed.clone().root(7f64, 10_000usize);
//     let seventh_root_const = const_fixed.root(7f64, ConstDecimals::<10_000>);

//     assert_eq!(
//         seventh_root_var,
//         ManagedDecimal::<StaticApi, NumDecimals>::from_raw_units(
//             BigUint::from(26324u64),
//             10_000usize
//         )
//     );
//     assert_eq!(
//         seventh_root_const,
//         ManagedDecimal::<StaticApi, ConstDecimals::<10_000>>::const_decimals_from_raw(
//             BigUint::from(26324u64)
//         )
//     );
// }
