use multiversx_sc::math::{
    LinearInterpolationInvalidValuesError, linear_interpolation, weighted_average,
    weighted_average_round_up,
};
use multiversx_sc::types::{BigUint, ManagedDecimal, NumDecimals};
use multiversx_sc_scenario::api::StaticApi;

fn md(v: u64) -> ManagedDecimal<StaticApi, NumDecimals> {
    ManagedDecimal::from_raw_units(BigUint::from(v), 4usize)
}

fn bu(v: u64) -> BigUint<StaticApi> {
    BigUint::from(v)
}

// ---- linear_interpolation ----

#[test]
fn linear_interpolation_at_min_input() {
    // current_in == min_in => output == min_out
    let result = linear_interpolation(md(0), md(100), md(0), md(200), md(400)).unwrap();
    assert_eq!(result, md(200));
}

#[test]
fn linear_interpolation_at_max_input() {
    // current_in == max_in => output == max_out
    let result = linear_interpolation(md(0), md(100), md(100), md(200), md(400)).unwrap();
    assert_eq!(result, md(400));
}

#[test]
fn linear_interpolation_at_midpoint() {
    // current_in at the midpoint => output at midpoint of output range
    let result = linear_interpolation(md(0), md(100), md(50), md(0), md(1000)).unwrap();
    assert_eq!(result, md(500));
}

#[test]
fn linear_interpolation_at_one_quarter() {
    // current_in at 25% => output at 25% of output range
    let result = linear_interpolation(md(0), md(100), md(25), md(0), md(1000)).unwrap();
    assert_eq!(result, md(250));
}

#[test]
fn linear_interpolation_non_zero_based_ranges() {
    // Input range [10, 50], output range [100, 200], current_in = 30 (50% through input)
    let result = linear_interpolation(md(10), md(50), md(30), md(100), md(200)).unwrap();
    assert_eq!(result, md(150));
}

#[test]
fn linear_interpolation_below_range_returns_error() {
    let result = linear_interpolation(md(10), md(100), md(5), md(0), md(1000));
    assert!(matches!(result, Err(LinearInterpolationInvalidValuesError)));
}

#[test]
fn linear_interpolation_above_range_returns_error() {
    let result = linear_interpolation(md(0), md(100), md(110), md(0), md(1000));
    assert!(matches!(result, Err(LinearInterpolationInvalidValuesError)));
}

// ---- weighted_average ----

#[test]
fn weighted_average_equal_weights() {
    // (10 * 1 + 20 * 1) / (1 + 1) = 15
    let result = weighted_average(bu(10), bu(1), bu(20), bu(1));
    assert_eq!(result, bu(15));
}

#[test]
fn weighted_average_all_weight_on_first() {
    // second_weight = 0 => result == first_value
    let result = weighted_average(bu(10), bu(5), bu(99), bu(0));
    assert_eq!(result, bu(10));
}

#[test]
fn weighted_average_all_weight_on_second() {
    // first_weight = 0 => result == second_value
    let result = weighted_average(bu(99), bu(0), bu(20), bu(5));
    assert_eq!(result, bu(20));
}

#[test]
fn weighted_average_three_to_one() {
    // (0 * 1 + 60 * 3) / (1 + 3) = 180 / 4 = 45
    let result = weighted_average(bu(0), bu(1), bu(60), bu(3));
    assert_eq!(result, bu(45));
}

// ---- weighted_average_round_up ----

#[test]
fn weighted_average_round_up_exact_division() {
    // (10 * 1 + 20 * 1) / (1 + 1) = 15, no rounding needed
    let result = weighted_average_round_up(bu(10), bu(1), bu(20), bu(1));
    assert_eq!(result, bu(15));
}

#[test]
fn weighted_average_round_up_truncates_vs_rounds() {
    // floor: (0 * 1 + 10 * 3) / (1 + 3) = 30 / 4 = 7
    // ceil:  (30 + 4 - 1) / 4 = 33 / 4 = 8
    let floor_result = weighted_average(bu(0), bu(1), bu(10), bu(3));
    let ceil_result = weighted_average_round_up(bu(0), bu(1), bu(10), bu(3));
    assert_eq!(floor_result, bu(7));
    assert_eq!(ceil_result, bu(8));
}

#[test]
fn weighted_average_round_up_no_change_when_exact() {
    // (0 * 1 + 20 * 3) / (1 + 3) = 60 / 4 = 15 exactly
    let result = weighted_average_round_up(bu(0), bu(1), bu(20), bu(3));
    assert_eq!(result, bu(15));
}
