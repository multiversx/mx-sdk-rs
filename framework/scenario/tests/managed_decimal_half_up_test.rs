use multiversx_sc::types::{BigInt, BigUint, ManagedDecimal, ManagedDecimalSigned, NumDecimals};
use multiversx_sc_scenario::api::StaticApi;

// ── half-up rounding helpers ──────────────────────────────────────────────────

fn md2(v: u64) -> ManagedDecimal<StaticApi, NumDecimals> {
    ManagedDecimal::from_raw_units(BigUint::from(v), 2usize)
}

fn mds2(v: i64) -> ManagedDecimalSigned<StaticApi, NumDecimals> {
    ManagedDecimalSigned::from_raw_units(BigInt::from(v), 2usize)
}

fn md1(v: u64) -> ManagedDecimal<StaticApi, NumDecimals> {
    ManagedDecimal::from_raw_units(BigUint::from(v), 1usize)
}

fn mds1(v: i64) -> ManagedDecimalSigned<StaticApi, NumDecimals> {
    ManagedDecimalSigned::from_raw_units(BigInt::from(v), 1usize)
}

// ── mul_half_up ───────────────────────────────────────────────────────────────
// Multiplication rescales both inputs to target precision, multiplies the raw
// values (giving 2×precision digits), then rounds: (product + scale/2) / scale.

#[test]
fn test_mul_half_up_exact() {
    // 2.0 * 3.0 = 6.0 exactly at precision 1
    // scaled_a=20, scaled_b=30, product=600, half=5, (600+5)/10=60 → 6.0
    let result = md1(20).mul_half_up(&md1(30), 1usize);
    assert_eq!(result.scale(), 1);
    assert_eq!(result.into_raw_units(), &BigUint::from(60u64));
}

#[test]
fn test_mul_half_up_rounds_up_at_half() {
    // 0.5 * 0.1 = 0.05 at precision 1 → rounds up to 0.1 (at exact half)
    // product=5, scale=10, half=5, (5+5)/10=1 → 0.1
    let result = md1(5).mul_half_up(&md1(1), 1usize);
    assert_eq!(result.into_raw_units(), &BigUint::from(1u64)); // 0.1
}

#[test]
fn test_mul_half_up_rounds_up_above_half() {
    // 1.5 * 1.5 = 2.25 at precision 1 → rounds up to 2.3
    // product=225, half=5, (225+5)/10=23 → 2.3
    let result = md1(15).mul_half_up(&md1(15), 1usize);
    assert_eq!(result.into_raw_units(), &BigUint::from(23u64)); // 2.3
}

#[test]
fn test_mul_half_up_rounds_down_below_half() {
    // 1.1 * 1.1 = 1.21 at precision 1 → rounds down to 1.2
    // product=121, half=5, (121+5)/10=12 → 1.2
    let result = md1(11).mul_half_up(&md1(11), 1usize);
    assert_eq!(result.into_raw_units(), &BigUint::from(12u64)); // 1.2
}

#[test]
fn test_mul_half_up_precision_increase() {
    // 1.00 * 2.00 with output at precision 4
    let result = md2(100).mul_half_up(&md2(200), 4usize);
    assert_eq!(result.scale(), 4);
    assert_eq!(result.into_raw_units(), &BigUint::from(20000u64)); // 2.0000
}

#[test]
fn test_mul_half_up_zero() {
    let result = md2(0).mul_half_up(&md2(500), 2usize);
    assert_eq!(result.into_raw_units(), &BigUint::from(0u64));
}

// ── div_half_up ───────────────────────────────────────────────────────────────
// Division: numerator = scaled_a * scale, denominator = scaled_b.
// Rounded: (numerator + denominator/2) / denominator.

#[test]
fn test_div_half_up_exact() {
    // 6.00 / 3.00 = 2.00 exactly
    let result = md2(600).div_half_up(&md2(300), 2usize);
    assert_eq!(result.into_raw_units(), &BigUint::from(200u64));
}

#[test]
fn test_div_half_up_rounds_up_at_half() {
    // 1.00 / 2.00 = 0.5 at precision 0 → rounds up to 1
    let result = md2(100).div_half_up(&md2(200), 0usize);
    assert_eq!(result.scale(), 0);
    assert_eq!(result.into_raw_units(), &BigUint::from(1u64));
}

#[test]
fn test_div_half_up_rounds_down_below_half() {
    // 1.00 / 3.00 ≈ 0.333 at precision 1 → rounds down to 0.3
    // scaled_a=10, scaled_b=30, num=100, half=15, (100+15)/30=3
    let result = md2(100).div_half_up(&md2(300), 1usize);
    assert_eq!(result.scale(), 1);
    assert_eq!(result.into_raw_units(), &BigUint::from(3u64));
}

#[test]
fn test_div_half_up_rounds_up_above_half() {
    // 2.00 / 3.00 ≈ 0.667 at precision 1 → rounds up to 0.7
    // scaled_a=20, scaled_b=30, num=200, half=15, (200+15)/30=7
    let result = md2(200).div_half_up(&md2(300), 1usize);
    assert_eq!(result.scale(), 1);
    assert_eq!(result.into_raw_units(), &BigUint::from(7u64));
}

#[test]
fn test_div_half_up_higher_precision() {
    // 1.00 / 3.00 at precision 4 ≈ 0.3333
    // scaled_a=10000, scaled_b=30000, num=100_000_000, half=15000
    // (100_000_000+15000)/30000 = 3333
    let result = md2(100).div_half_up(&md2(300), 4usize);
    assert_eq!(result.scale(), 4);
    assert_eq!(result.into_raw_units(), &BigUint::from(3333u64));
}

// ── mul_half_up_signed ────────────────────────────────────────────────────────
// Same product rounding as unsigned; sign-aware: negative product subtracts
// half before dividing, so truncation rounds away from zero.

#[test]
fn test_mul_half_up_signed_both_positive() {
    // 1.5 * 1.5 = 2.25 → 2.3 (rounds up)
    let result = mds1(15).mul_half_up_signed(&mds1(15), 1usize);
    assert_eq!(result.into_raw_units(), &BigInt::from(23i64));
}

#[test]
fn test_mul_half_up_signed_positive_x_negative_rounds_away() {
    // 1.5 * (-1.5) = -2.25 → -2.3 (away from zero)
    // product=-225, sign Minus → subtract: (-225-5)/10=-23
    let result = mds1(15).mul_half_up_signed(&mds1(-15), 1usize);
    assert_eq!(result.into_raw_units(), &BigInt::from(-23i64));
}

#[test]
fn test_mul_half_up_signed_both_negative() {
    // (-1.5) * (-1.5) = +2.25 → +2.3
    let result = mds1(-15).mul_half_up_signed(&mds1(-15), 1usize);
    assert_eq!(result.into_raw_units(), &BigInt::from(23i64));
}

#[test]
fn test_mul_half_up_signed_negative_x_positive_rounds_away() {
    // (-1.5) * 1.5 = -2.25 → -2.3
    let result = mds1(-15).mul_half_up_signed(&mds1(15), 1usize);
    assert_eq!(result.into_raw_units(), &BigInt::from(-23i64));
}

#[test]
fn test_mul_half_up_signed_rounds_down_below_half() {
    // (-1.1) * 1.1 = -1.21 → -1.2 (|remainder|=0.01 < 0.05, toward zero)
    // product=-121, sign Minus → (-121-5)/10=-126/10=-12 → -1.2
    let result = mds1(-11).mul_half_up_signed(&mds1(11), 1usize);
    assert_eq!(result.into_raw_units(), &BigInt::from(-12i64));
}

#[test]
fn test_mul_half_up_signed_exact_no_rounding() {
    // -2.0 * 3.0 = -6.0 exactly
    let result = mds2(-200).mul_half_up_signed(&mds2(300), 2usize);
    assert_eq!(result.into_raw_units(), &BigInt::from(-600i64));
}

#[test]
fn test_mul_half_up_signed_zero() {
    let result = mds2(0).mul_half_up_signed(&mds2(-300), 2usize);
    assert_eq!(result.into_raw_units(), &BigInt::from(0i64));
}

// ── div_half_up_signed ────────────────────────────────────────────────────────
// Numerator sign determines pre-bias direction so bi_t_div (truncates toward
// zero) always rounds away from zero:
//   num >= 0 → add half   (both signs: +/+ rounds up, +/− rounds toward −∞)
//   num <  0 → subtract half  (−/+ rounds toward −∞, −/− rounds toward +∞)

#[test]
fn test_div_half_up_signed_exact_positive() {
    // 6.00 / 3.00 = 2.00 exactly
    let result = mds2(600).div_half_up_signed(&mds2(300), 2usize);
    assert_eq!(result.into_raw_units(), &BigInt::from(200i64));
}

#[test]
fn test_div_half_up_signed_exact_negative_result() {
    // -6.00 / 3.00 = -2.00 exactly
    let result = mds2(-600).div_half_up_signed(&mds2(300), 2usize);
    assert_eq!(result.into_raw_units(), &BigInt::from(-200i64));
}

#[test]
fn test_div_half_up_signed_exact_both_negative() {
    // -6.00 / -2.00 = 3.00 exactly
    // num=-60000, num<0 → subtract: (-60000-100)/(-200)=-60100/−200=300
    let result = mds2(-600).div_half_up_signed(&mds2(-200), 2usize);
    assert_eq!(result.into_raw_units(), &BigInt::from(300i64));
}

#[test]
fn test_div_half_up_signed_positive_rounds_up_at_half() {
    // 1.00 / 2.00 = 0.5 at precision 0 → +1 (away from zero)
    let result = mds2(100).div_half_up_signed(&mds2(200), 0usize);
    assert_eq!(result.scale(), 0);
    assert_eq!(result.into_raw_units(), &BigInt::from(1i64));
}

#[test]
fn test_div_half_up_signed_negative_rounds_away_at_half() {
    // -1.00 / 2.00 = -0.5 at precision 0 → -1 (away from zero)
    let result = mds2(-100).div_half_up_signed(&mds2(200), 0usize);
    assert_eq!(result.scale(), 0);
    assert_eq!(result.into_raw_units(), &BigInt::from(-1i64));
}

#[test]
fn test_div_half_up_signed_positive_rounds_up_above_half() {
    // 2.00 / 3.00 ≈ 0.667 at precision 1 → 0.7
    let result = mds2(200).div_half_up_signed(&mds2(300), 1usize);
    assert_eq!(result.scale(), 1);
    assert_eq!(result.into_raw_units(), &BigInt::from(7i64));
}

#[test]
fn test_div_half_up_signed_positive_rounds_down_below_half() {
    // 1.00 / 3.00 ≈ 0.333 at precision 1 → 0.3
    let result = mds2(100).div_half_up_signed(&mds2(300), 1usize);
    assert_eq!(result.scale(), 1);
    assert_eq!(result.into_raw_units(), &BigInt::from(3i64));
}

#[test]
fn test_div_half_up_signed_negative_dividend_positive_divisor() {
    // -2.00 / 3.00 ≈ -0.667 at precision 1 → -0.7 (away from zero)
    // num=-200*10=-2000... wait: scaled_a=-20(at p1), scaled_b=30(at p1)
    // num=-20*10=-200, denom=30, half=15, num<0 → (-200-15)/30=-215/30=-7
    let result = mds2(-200).div_half_up_signed(&mds2(300), 1usize);
    assert_eq!(result.scale(), 1);
    assert_eq!(result.into_raw_units(), &BigInt::from(-7i64));
}

#[test]
fn test_div_half_up_signed_positive_dividend_negative_divisor() {
    // 2.00 / -3.00 ≈ -0.667 at precision 1 → -0.7 (away from zero)
    // num=200, denom=-30, half=15, num>=0 → (200+15)/(-30)=215/(-30)=-7
    let result = mds2(200).div_half_up_signed(&mds2(-300), 1usize);
    assert_eq!(result.scale(), 1);
    assert_eq!(result.into_raw_units(), &BigInt::from(-7i64));
}
