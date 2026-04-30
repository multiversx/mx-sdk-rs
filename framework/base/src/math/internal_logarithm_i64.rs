const DENOMINATOR: i64 = 1_000_000_000;

const LN_OF_2_SCALE_9: I64Decimal9 = 693147180; // 0.69314718
const LN_OF_10_SCALE_9: I64Decimal9 = 2302585093; // 2.3025850929940456840...
const LOG2_OF_10_SCALE_9: I64Decimal9 = 3321928095; // 3.3219280948873623478...

/// Indicates that a number is interpreted as a decimal number with 9 decimals.
pub type I64Decimal9 = i64;

/// Approximates the logarithm between 1 and 2 with a polynomial.
///
/// The polynomial is: `-1.7417939 + x * (2.8212026 + (-1.4699568 + (0.44717955 - 0.056570851 * x) * x) * x)`
pub fn ln_polynomial(x: I64Decimal9) -> I64Decimal9 {
    // x normalized to [1.0, 2.0]
    debug_assert!(x >= DENOMINATOR);
    debug_assert!(x <= 2 * DENOMINATOR);

    let mut result: i64 = -56570851; // -0.056570851
    result *= x;
    result /= DENOMINATOR;
    result += 447179550; // 0.44717955
    result *= x;
    result /= DENOMINATOR;
    result += -1469956800; // -1.4699568
    result *= x;
    result /= DENOMINATOR;
    result += 2821202600; // 2.8212026
    result *= x;
    result /= DENOMINATOR;
    result += -1741793900; // -1.7417939

    result
}

/// Just took the coefficients from ln and dividem them all by ln(2).
pub fn log2_polynomial(x: I64Decimal9) -> I64Decimal9 {
    // x normalized to [1.0, 2.0]
    debug_assert!(x >= DENOMINATOR);
    debug_assert!(x <= 2 * DENOMINATOR);

    let mut result: i64 = -81614486; // -0.08161448626
    result *= x;
    result /= DENOMINATOR;
    result += 645143719; // 0.645143719
    result *= x;
    result /= DENOMINATOR;
    result += -2120699387; // -2.120699387
    result *= x;
    result /= DENOMINATOR;
    result += 4070135003; //  4.070135003
    result *= x;
    result /= DENOMINATOR;
    result += -2512877423; // -2.512877423

    result
}

pub fn ln_add_bit_log2(result: &mut I64Decimal9, bit_log2: u32) {
    *result += bit_log2 as i64 * LN_OF_2_SCALE_9;
}

pub fn log2_add_bit_log2(result: &mut I64Decimal9, bit_log2: u32) {
    *result += bit_log2 as i64 * DENOMINATOR;
}

pub fn ln_sub_decimals(result: &mut I64Decimal9, num_decimals: usize) {
    *result -= num_decimals as i64 * LN_OF_10_SCALE_9;
}

pub fn log2_sub_decimals(result: &mut I64Decimal9, num_decimals: usize) {
    *result -= num_decimals as i64 * LOG2_OF_10_SCALE_9;
}
