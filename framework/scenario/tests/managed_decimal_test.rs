use multiversx_sc::{
    codec::test_util::{check_dep_encode_decode, check_top_encode_decode},
    derive::{debug_const_managed_decimal, debug_managed_decimal},
    typenum::{self, U0, U1, U10, U2, U3, U4, U5, U6, U7, U8},
    types::{
        BigFloat, BigInt, BigUint, ConstDecimals, Decimals, ManagedDecimal, ManagedDecimalSigned,
        NumDecimals,
    },
};
use multiversx_sc_scenario::api::StaticApi;

#[test]
pub fn test_managed_decimal() {
    let fixed = ManagedDecimal::<StaticApi, ConstDecimals<U2>>::from(BigUint::from(1u64));
    let fixed_2 = ManagedDecimal::<StaticApi, ConstDecimals<U2>>::from(BigUint::from(5u64));
    let fixed_3 = ManagedDecimal::<StaticApi, ConstDecimals<U4>>::from(BigUint::from(8u64));

    let addition = fixed.clone() + fixed_2.clone();

    assert_eq!(
        addition,
        ManagedDecimal::<StaticApi, ConstDecimals<U2>>::from(BigUint::from(6u64))
    );
    assert_eq!(addition.into_raw_units(), &BigUint::from(600u64));
    assert_eq!(addition.trunc(), BigUint::from(6u64));

    let subtraction = addition - fixed;
    assert_eq!(
        subtraction,
        ManagedDecimal::<StaticApi, ConstDecimals<U2>>::from(BigUint::from(5u64))
    );

    let multiplication = fixed_3.clone() * fixed_2;
    assert_eq!(
        multiplication,
        ManagedDecimal::<StaticApi, ConstDecimals<U6>>::from(BigUint::from(40u64))
    );

    let division = multiplication / fixed_3;
    assert_eq!(
        division,
        ManagedDecimal::<StaticApi, ConstDecimals<U2>>::from(BigUint::from(5u64))
    );
}

#[test]
pub fn test_managed_decimal_mixed() {
    let fixed =
        ManagedDecimal::<StaticApi, NumDecimals>::from_raw_units(BigUint::from(100u64), 2usize);
    let fixed_2 = ManagedDecimal::<StaticApi, ConstDecimals<U2>>::from(BigUint::from(5u64));
    let fixed_3 =
        ManagedDecimal::<StaticApi, NumDecimals>::from_raw_units(BigUint::from(80000u64), 4usize);

    let addition = fixed.clone() + fixed_2.clone();

    assert_eq!(
        addition,
        ManagedDecimal::<StaticApi, ConstDecimals<U2>>::from(BigUint::from(6u64))
    );

    assert_eq!(addition.into_raw_units(), &BigUint::from(600u64));
    assert_eq!(addition.trunc(), BigUint::from(6u64));

    let subtraction = addition - fixed;
    assert_eq!(
        subtraction,
        ManagedDecimal::<StaticApi, ConstDecimals<U2>>::from(BigUint::from(5u64))
    );

    let multiplication = fixed_3.clone() * fixed_2;
    assert_eq!(
        multiplication,
        ManagedDecimal::<StaticApi, ConstDecimals<U6>>::from(BigUint::from(40u64))
    );

    let division = multiplication / fixed_3;
    assert_eq!(
        division,
        ManagedDecimal::<StaticApi, ConstDecimals<U2>>::from(BigUint::from(5u64))
    );
}

fn assert_exact<D: Decimals>(dec: &ManagedDecimal<StaticApi, D>, raw_u64: u64, scale: usize) {
    let raw_units = BigUint::from(raw_u64);
    assert_eq!(dec.scale(), scale);
    assert_eq!(dec.into_raw_units(), &raw_units);
    assert_eq!(dec, &ManagedDecimal::from_raw_units(raw_units, scale));
}

#[test]
pub fn test_managed_decimal_rescale_unchanged() {
    let dec: ManagedDecimal<StaticApi, NumDecimals> =
        ManagedDecimal::from_raw_units(BigUint::from(100u64), 2usize);
    assert_exact(&dec, 100u64, 2);
    let rescaled = dec.rescale(2usize);
    assert_exact(&rescaled, 100u64, 2);
    assert_eq!(
        rescaled,
        ManagedDecimal::from_raw_units(BigUint::from(100000000u64), 8usize)
    );
}

#[test]
pub fn test_managed_decimal_rescale_up() {
    let uint_value = 1500u64;
    let dec: ManagedDecimal<StaticApi, ConstDecimals<U2>> =
        ManagedDecimal::from(BigUint::from(uint_value));
    assert_exact(&dec, uint_value * 100, 2);
    let rescaled = dec.rescale(ConstDecimals::<U8>::new());
    assert_exact(&rescaled, uint_value * 100000000, 8);
    assert_eq!(
        rescaled,
        ManagedDecimal::<StaticApi, ConstDecimals<U8>>::from(BigUint::from(uint_value))
    );
}

#[test]
pub fn test_managed_decimal_rescale_down_1() {
    // 1234.0000000 -> 1234.
    let uint_value = 1234u64;
    let dec: ManagedDecimal<StaticApi, ConstDecimals<U7>> =
        ManagedDecimal::from(BigUint::from(uint_value));
    assert_exact(&dec, uint_value * 10000000, 7);
    let rescaled = dec.rescale(ConstDecimals::<U3>::new());
    assert_exact(&rescaled, uint_value * 1000, 3);
}

#[test]
pub fn test_managed_decimal_rescale_down_2() {
    // 0.00009 -> 0.0000
    let dec: ManagedDecimal<StaticApi, NumDecimals> =
        ManagedDecimal::from_raw_units(BigUint::from(9u64), 5usize);
    assert_exact(&dec, 9, 5);
    let rescaled = dec.rescale(ConstDecimals::<U4>::new());
    assert_exact(&rescaled, 0, 4);
    assert_eq!(
        rescaled,
        ManagedDecimal::<StaticApi, ConstDecimals<U4>>::from(BigUint::zero())
    );
}

#[test]
pub fn test_managed_decimal_rescale_down_3() {
    // 1.00009 -> 1.0000
    let dec: ManagedDecimal<StaticApi, NumDecimals> =
        ManagedDecimal::from_raw_units(BigUint::from(100009u64), 5usize);
    assert_exact(&dec, 100009, 5);
    let rescaled = dec.rescale(ConstDecimals::<U4>::new());
    assert_exact(&rescaled, 10000, 4);
    assert_eq!(
        rescaled,
        ManagedDecimal::<StaticApi, ConstDecimals<U4>>::from(BigUint::from(1u64))
    );
}

#[test]
pub fn test_managed_decimal_from_big_float() {
    let float_1 = BigFloat::<StaticApi>::from_frac(3i64, 2i64);
    let fixed_float_1 = ManagedDecimalSigned::<StaticApi, ConstDecimals<U1>>::from_big_float(
        &float_1,
        ConstDecimals::<U1>::new(),
    );
    let fixed_float_2 =
        ManagedDecimalSigned::<StaticApi, NumDecimals>::from_big_float(&float_1, 1usize);

    assert_eq!(
        fixed_float_1,
        ManagedDecimalSigned::<StaticApi, ConstDecimals<U1>>::const_decimals_from_raw(
            BigInt::from(15)
        )
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
    let fixed_1 = ManagedDecimalSigned::<StaticApi, ConstDecimals<U2>>::from(BigInt::from(1i64));
    let fixed_2 = ManagedDecimalSigned::<StaticApi, ConstDecimals<U2>>::from(BigInt::from(3i64));
    let fixed_3 = ManagedDecimalSigned::<StaticApi, ConstDecimals<U2>>::from(BigInt::from(-5i64));
    let fixed_4 = ManagedDecimalSigned::<StaticApi, ConstDecimals<U2>>::from(BigInt::from(-2i64));

    let addition_1 = fixed_1.clone() + fixed_2.clone();
    assert_eq!(
        addition_1,
        ManagedDecimalSigned::<StaticApi, ConstDecimals<U2>>::from(BigInt::from(4i64))
    );
    assert_eq!(addition_1.into_raw_units(), &BigInt::from(400i64));
    assert_eq!(addition_1.trunc(), BigInt::from(4i64));

    let addition_2 = fixed_1.clone() + fixed_3.clone();
    assert_eq!(
        addition_2,
        ManagedDecimalSigned::<StaticApi, ConstDecimals<U2>>::from(BigInt::from(-4i64))
    );
    assert_eq!(addition_2.into_raw_units(), &BigInt::from(-400i64));
    assert_eq!(addition_2.trunc(), BigInt::from(-4i64));

    let addition_3 = fixed_3.clone() + fixed_4.clone();
    assert_eq!(
        addition_3,
        ManagedDecimalSigned::<StaticApi, ConstDecimals<U2>>::from(BigInt::from(-7i64))
    );
    assert_eq!(addition_3.into_raw_units(), &BigInt::from(-700i64));
    assert_eq!(addition_3.trunc(), BigInt::from(-7i64));

    let addition_4 = fixed_4.clone() + fixed_2.clone();
    assert_eq!(
        addition_4,
        ManagedDecimalSigned::<StaticApi, ConstDecimals<U2>>::from(BigInt::from(1i64))
    );
    assert_eq!(addition_4.into_raw_units(), &BigInt::from(100i64));
    assert_eq!(addition_4.trunc(), BigInt::from(1i64));
}

fn assert_exact_signed<D: Decimals>(
    dec: &ManagedDecimalSigned<StaticApi, D>,
    raw_u64: i64,
    scale: usize,
) {
    let raw_units = BigInt::from(raw_u64);
    assert_eq!(dec.scale(), scale);
    assert_eq!(dec.into_raw_units(), &raw_units);
    assert_eq!(dec, &ManagedDecimalSigned::from_raw_units(raw_units, scale));
}

#[test]
pub fn test_managed_decimal_signed_rescale_unchanged() {
    let dec: ManagedDecimalSigned<StaticApi, NumDecimals> =
        ManagedDecimalSigned::from_raw_units(BigInt::from(100), 2usize);
    assert_exact_signed(&dec, 100, 2);
    let rescaled = dec.rescale(2usize);
    assert_exact_signed(&rescaled, 100, 2);
    assert_eq!(
        rescaled,
        ManagedDecimalSigned::from_raw_units(BigInt::from(100000000), 8usize)
    );
}

#[test]
pub fn test_managed_decimal_signed_rescale_up() {
    let uint_value = -1500i64;
    let dec: ManagedDecimalSigned<StaticApi, ConstDecimals<U2>> =
        ManagedDecimalSigned::from(BigInt::from(uint_value));
    assert_exact_signed(&dec, uint_value * 100, 2);
    let rescaled = dec.rescale(ConstDecimals::<U8>::new());
    assert_exact_signed(&rescaled, uint_value * 100000000, 8);
    assert_eq!(
        rescaled,
        ManagedDecimalSigned::<StaticApi, ConstDecimals<U8>>::from(BigInt::from(uint_value))
    );
}

#[test]
pub fn test_managed_decimal_signed_rescale_down_1() {
    // -1234.0000000 -> -1234.
    let uint_value = -1234;
    let dec: ManagedDecimalSigned<StaticApi, ConstDecimals<U7>> =
        ManagedDecimalSigned::from(BigInt::from(uint_value));
    assert_exact_signed(&dec, uint_value * 10000000, 7);
    let rescaled = dec.rescale(ConstDecimals::<U3>::new());
    assert_exact_signed(&rescaled, uint_value * 1000, 3);
}

#[test]
pub fn test_managed_decimal_signed_rescale_down_2() {
    // 0.00009 -> 0.0000
    let dec: ManagedDecimalSigned<StaticApi, NumDecimals> =
        ManagedDecimalSigned::from_raw_units(BigInt::from(-9), 5usize);
    assert_exact_signed(&dec, -9, 5);
    let rescaled = dec.rescale(ConstDecimals::<U4>::new());
    assert_exact_signed(&rescaled, 0, 4);
    assert_eq!(
        rescaled,
        ManagedDecimalSigned::<StaticApi, ConstDecimals<U4>>::from(BigInt::zero())
    );
}

#[test]
pub fn test_managed_decimal_signed_rescale_down_3() {
    // -1.00009 -> -1.0000
    let dec: ManagedDecimalSigned<StaticApi, NumDecimals> =
        ManagedDecimalSigned::from_raw_units(BigInt::from(-100009), 5usize);
    assert_exact_signed(&dec, -100009, 5);
    let rescaled = dec.rescale(ConstDecimals::<U4>::new());
    assert_exact_signed(&rescaled, -10000, 4);
    assert_eq!(
        rescaled,
        ManagedDecimalSigned::<StaticApi, ConstDecimals<U4>>::from(BigInt::from(-1))
    );
}

#[test]
pub fn test_substraction_managed_decimal_signed() {
    let fixed_1 = ManagedDecimalSigned::<StaticApi, ConstDecimals<U2>>::from(BigInt::from(1i64));
    let fixed_2 = ManagedDecimalSigned::<StaticApi, ConstDecimals<U2>>::from(BigInt::from(3i64));
    let fixed_3 = ManagedDecimalSigned::<StaticApi, ConstDecimals<U2>>::from(BigInt::from(-5i64));
    let fixed_4 = ManagedDecimalSigned::<StaticApi, ConstDecimals<U2>>::from(BigInt::from(-2i64));

    let substraction_1 = fixed_2.clone() - fixed_1.clone();
    assert_eq!(
        substraction_1,
        ManagedDecimalSigned::<StaticApi, ConstDecimals<U2>>::from(BigInt::from(2i64))
    );
    assert_eq!(substraction_1.into_raw_units(), &BigInt::from(200i64));
    assert_eq!(substraction_1.trunc(), BigInt::from(2i64));

    let substraction_2 = fixed_1.clone() - fixed_2.clone();
    assert_eq!(
        substraction_2,
        ManagedDecimalSigned::<StaticApi, ConstDecimals<U2>>::from(BigInt::from(-2i64))
    );
    assert_eq!(substraction_2.into_raw_units(), &BigInt::from(-200i64));
    assert_eq!(substraction_2.trunc(), BigInt::from(-2i64));

    let substraction_3 = substraction_2 - fixed_3.clone();
    assert_eq!(
        substraction_3,
        ManagedDecimalSigned::<StaticApi, ConstDecimals<U2>>::from(BigInt::from(3i64))
    );
    assert_eq!(substraction_3.into_raw_units(), &BigInt::from(300i64));
    assert_eq!(substraction_3.trunc(), BigInt::from(3i64));

    let substraction_4 = fixed_3.clone() - fixed_4.clone();
    assert_eq!(
        substraction_4,
        ManagedDecimalSigned::<StaticApi, ConstDecimals<U2>>::from(BigInt::from(-3i64))
    );
    assert_eq!(substraction_4.into_raw_units(), &BigInt::from(-300i64));
    assert_eq!(substraction_4.trunc(), BigInt::from(-3i64));
}

#[test]
pub fn test_multiplication_managed_decimal_signed() {
    let fixed_1 = ManagedDecimalSigned::<StaticApi, ConstDecimals<U2>>::from(BigInt::from(1i64));
    let fixed_2 = ManagedDecimalSigned::<StaticApi, ConstDecimals<U2>>::from(BigInt::from(3i64));
    let fixed_3 = ManagedDecimalSigned::<StaticApi, ConstDecimals<U2>>::from(BigInt::from(-5i64));
    let fixed_4 = ManagedDecimalSigned::<StaticApi, ConstDecimals<U2>>::from(BigInt::from(-2i64));

    let multiplication_1 = fixed_1.clone() * fixed_2.clone();
    assert_eq!(
        multiplication_1,
        ManagedDecimalSigned::<StaticApi, ConstDecimals<U6>>::from(BigInt::from(3i64))
    );

    let multiplication_2 = fixed_3.clone() * fixed_2.clone();
    assert_eq!(
        multiplication_2,
        ManagedDecimalSigned::<StaticApi, ConstDecimals<U6>>::from(BigInt::from(-15i64))
    );

    let multiplication_2 = fixed_3.clone() * fixed_4.clone();
    assert_eq!(
        multiplication_2,
        ManagedDecimalSigned::<StaticApi, ConstDecimals<U6>>::from(BigInt::from(10i64))
    );
}

#[test]
pub fn test_devision_managed_decimal_signed() {
    let fixed_1 = ManagedDecimalSigned::<StaticApi, ConstDecimals<U2>>::from(BigInt::from(6i64));
    let fixed_2 = ManagedDecimalSigned::<StaticApi, ConstDecimals<U2>>::from(BigInt::from(2i64));
    let fixed_3 = ManagedDecimalSigned::<StaticApi, ConstDecimals<U2>>::from(BigInt::from(-8i64));
    let fixed_4 = ManagedDecimalSigned::<StaticApi, ConstDecimals<U2>>::from(BigInt::from(-2i64));

    let division_1 = fixed_1.clone() / fixed_2;
    assert_eq!(
        division_1,
        ManagedDecimalSigned::<StaticApi, ConstDecimals<U2>>::from(BigInt::from(3i64))
    );

    let division_2 = fixed_1 / fixed_4.clone();
    assert_eq!(
        division_2,
        ManagedDecimalSigned::<StaticApi, ConstDecimals<U2>>::from(BigInt::from(-3i64))
    );

    let division_3 = fixed_3 / fixed_4;
    assert_eq!(
        division_3,
        ManagedDecimalSigned::<StaticApi, ConstDecimals<U2>>::from(BigInt::from(4i64))
    );
}

#[test]
pub fn test_rescale_managed_decimal_signed() {
    let float_1 = BigFloat::<StaticApi>::from_frac(-3i64, 2i64);
    let fixed_float_1 = ManagedDecimalSigned::<StaticApi, ConstDecimals<U1>>::from_big_float(
        &float_1,
        ConstDecimals::<U1>::new(),
    );
    let fixed_float_2 =
        ManagedDecimalSigned::<StaticApi, NumDecimals>::from_big_float(&float_1, 1usize);

    assert_eq!(
        fixed_float_1,
        ManagedDecimalSigned::<StaticApi, ConstDecimals<U1>>::const_decimals_from_raw(
            BigInt::from(-15)
        )
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

    let fixed_const: ManagedDecimal<StaticApi, ConstDecimals<U1>> =
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

    let const_dec = ManagedDecimal::<StaticApi, ConstDecimals<U0>>::const_decimals_from_raw(
        BigUint::from(29299837u64),
    );
    let ln_const = const_dec.ln().unwrap();

    assert_eq!(ln_const.to_string(), "17.193072541");

    let small = ManagedDecimal::<StaticApi, ConstDecimals<U1>>::const_decimals_from_raw(
        BigUint::from(1u64),
    ); // 0.1
    let ln_small = small.ln().unwrap();

    assert_eq!(ln_small.to_string(), "-2.302524494");

    let v_small = ManagedDecimal::<StaticApi, ConstDecimals<U2>>::const_decimals_from_raw(
        BigUint::from(1u64),
    );
    // 0.01
    let ln_v_small = v_small.ln().unwrap();

    assert_eq!(ln_v_small.to_string(), "-4.605109587");

    let smallest = ManagedDecimal::<StaticApi, ConstDecimals<U6>>::const_decimals_from_raw(
        BigUint::from(1u64),
    );
    // 0.000001
    let ln_smallest = smallest.ln().unwrap();

    assert_eq!(ln_smallest.to_string(), "-13.815449959");

    let frac = ManagedDecimal::<StaticApi, ConstDecimals<U2>>::const_decimals_from_raw(
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

#[test]
fn test_managed_decimal_log2() {
    let fixed =
        ManagedDecimal::<StaticApi, NumDecimals>::from_raw_units(BigUint::from(5u64), 0usize);
    let log2_fixed = fixed.log2().unwrap();

    assert_eq!(log2_fixed.to_string(), "2.321990749");

    let const_dec = ManagedDecimal::<StaticApi, ConstDecimals<U0>>::const_decimals_from_raw(
        BigUint::from(29299837u64),
    );
    let log2_const = const_dec.log2().unwrap();

    assert_eq!(log2_const.to_string(), "24.804360510");

    let small = ManagedDecimal::<StaticApi, ConstDecimals<U1>>::const_decimals_from_raw(
        BigUint::from(1u64),
    ); // 0.1
    let log2_small = small.log2().unwrap();

    assert_eq!(log2_small.to_string(), "-3.321840669");

    let v_small = ManagedDecimal::<StaticApi, ConstDecimals<U2>>::const_decimals_from_raw(
        BigUint::from(1u64),
    );
    // 0.01
    let log2_v_small = v_small.log2().unwrap();

    assert_eq!(log2_v_small.to_string(), "-6.643768764");

    let smallest = ManagedDecimal::<StaticApi, ConstDecimals<U6>>::const_decimals_from_raw(
        BigUint::from(1u64),
    );
    // 0.000001
    let log2_smallest = smallest.log2().unwrap();

    assert_eq!(log2_smallest.to_string(), "-19.931481144");

    let frac =
        ManagedDecimal::<StaticApi, NumDecimals>::from_raw_units(BigUint::from(1872u64), 2usize); //18.72
    let log2_frac = frac.log2().unwrap();

    assert_eq!(log2_frac.to_string(), "4.226560385");

    let b_frac = ManagedDecimal::<StaticApi, NumDecimals>::from_raw_units(
        BigUint::from(39874291763u64),
        7usize,
    ); //3987.4291763
    let log2_b_frac = b_frac.log2().unwrap();

    assert_eq!(log2_b_frac.to_string(), "11.961212882");

    let normal_prec_frac =
        ManagedDecimal::<StaticApi, NumDecimals>::from_raw_units(BigUint::from(453211u64), 3usize);
    // 453.211
    let log2_np_frac = normal_prec_frac.log2().unwrap();

    assert_eq!(log2_np_frac.to_string(), "8.823994915");

    let high_prec_frac = ManagedDecimal::<StaticApi, NumDecimals>::from_raw_units(
        BigUint::from(453211000000u64),
        9usize,
    );
    // 453.211000000
    let log2_hp_frac = high_prec_frac.log2().unwrap();

    assert_eq!(log2_hp_frac.to_string(), "8.823953218");
}

#[test]
pub fn test_managed_decimal_mul_mix_decimals_type() {
    let dynamic: ManagedDecimal<StaticApi, NumDecimals> =
        ManagedDecimal::from_raw_units(BigUint::from(10000000u64), 5usize);
    let constant: ManagedDecimal<StaticApi, ConstDecimals<U5>> =
        ManagedDecimal::from(BigUint::from(100u64));

    let result = dynamic * constant;

    let expected: ManagedDecimal<StaticApi, ConstDecimals<U10>> =
        ManagedDecimal::from(BigUint::from(10000u64));

    assert_eq!(result, expected)
}

#[test]
pub fn test_managed_decimal_mul_mix_decimals_type_reverse() {
    let dynamic: ManagedDecimal<StaticApi, NumDecimals> =
        ManagedDecimal::from_raw_units(BigUint::from(100u64), 5usize);
    let constant: ManagedDecimal<StaticApi, ConstDecimals<U5>> =
        ManagedDecimal::from(BigUint::from(100u64));

    let result = constant * dynamic;

    let expected = ManagedDecimal::from_raw_units(BigUint::from(1000000000u64), 10usize);

    assert_eq!(result, expected)
}

#[test]
pub fn test_managed_decimal_div_mix_decimals_type() {
    let dynamic: ManagedDecimal<StaticApi, NumDecimals> =
        ManagedDecimal::from_raw_units(BigUint::from(100u64), 5usize);
    let constant: ManagedDecimal<StaticApi, ConstDecimals<U10>> =
        ManagedDecimal::from(BigUint::from(100u64));

    let result = constant / dynamic;

    let expected = ManagedDecimal::from_raw_units(BigUint::from(10000000000u64), 5usize);

    assert_eq!(result, expected)
}

#[test]
pub fn test_managed_decimal_div_mix_decimals_type_reverse() {
    let dynamic: ManagedDecimal<StaticApi, NumDecimals> =
        ManagedDecimal::from_raw_units(BigUint::from(100u64), 10usize);
    let constant: ManagedDecimal<StaticApi, ConstDecimals<U10>> =
        ManagedDecimal::from(BigUint::from(100u64));

    let result = dynamic / constant;

    let expected = ManagedDecimal::from_raw_units(BigUint::from(0u64), 0usize);

    assert_eq!(result, expected)
}
