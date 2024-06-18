use multiversx_sc::{
    codec::test_util::{check_dep_encode_decode, check_top_encode_decode},
    derive::{debug_const_managed_decimal, debug_managed_decimal},
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

#[test]
fn test_managed_decimal_ln() {
    let fixed = ManagedDecimal::<StaticApi, ConstDecimals<0>>::const_decimals_from_raw(
        BigUint::from(23u64),
    );

    let ln_fixed = fixed.ln(ConstDecimals::<10>); // precision of 10 decimal points

    println!("{ln_fixed:?}");
    assert_eq!(
        ln_fixed,
        ManagedDecimal::<StaticApi, ConstDecimals<10>>::const_decimals_from_raw(BigUint::from(
            31355146488u64
        ),)
    );
}

// #[test]
// fn test_managed_decimal_ln() {
//     let fixed =
//         ManagedDecimal::<StaticApi, NumDecimals>::from_raw_units(BigUint::from(23u64), 0usize);

//     let ln_fixed = fixed.ln(&10usize); // precision of 10 decimal points

//     println!("{ln_fixed:?}");
//     assert_eq!(
//         ln_fixed,
//         ManagedDecimal::<StaticApi, NumDecimals>::from_raw_units(
//             BigUint::from(31355146488u64),
//             10usize
//         )
//     );
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

// #[test]
// fn test_managed_decimal_log_any_base() {
// let fixed = ManagedDecimal::from_raw_units(BigUint::from(10000u64), 1usize);
// let log2_fixed = fixed.log(
//     &ManagedDecimal::from_raw_units(BigUint::from(2u64), 1usize),
//     4usize,
// );
// assert_eq!(
//     log2_fixed,
//     ManagedDecimal::<StaticApi, NumDecimals>::from_raw_units(
//         BigUint::from(33219u64),
//         4usize
//     )
// );

// let fixed_const = ManagedDecimal::<StaticApi, ConstDecimals<1>>::const_decimals_from_raw(
//     BigUint::from(10u64),
// );
// let log2_const = fixed_const.log(
//     &ManagedDecimal::<StaticApi, ConstDecimals<1>>::const_decimals_from_raw(BigUint::from(
//         2u64,
//     )),
//     &ConstDecimals::<10_000>,
// );
// assert_eq!(
//     log2_const,
//     ManagedDecimal::<StaticApi, ConstDecimals::<10_000>>::const_decimals_from_raw(
//         BigUint::from(33219u64)
//     )
// );
// }
