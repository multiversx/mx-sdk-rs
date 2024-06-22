const DENOMINATOR: i64 = 1_000_000_000;

const LN_OF_2_SCALE_9: I64Decimal9 = 693147180; // 0.69314718

/// Indicates that a number is interpreted as a decimal number with 9 decimals.
pub type I64Decimal9 = i64;

fn ln_polynomial(x: I64Decimal9) -> I64Decimal9 {
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

pub fn ln_from_bits_and_normalized(bit_log2: u32, x: I64Decimal9) -> I64Decimal9 {
    let mut result = ln_polynomial(x);

    result += bit_log2 as i64 * LN_OF_2_SCALE_9;

    result
}
