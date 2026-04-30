use multiversx_sc::math::{
    LinearInterpolationInvalidValuesError, linear_interpolation, weighted_average,
    weighted_average_round_up,
};

// ---- linear_interpolation ----

#[test]
fn linear_interpolation_at_min_input() {
    // current_in == min_in => output == min_out
    let result = linear_interpolation(0u32, 100u32, 0u32, 200u32, 400u32).unwrap();
    assert_eq!(result, 200u32);
}

#[test]
fn linear_interpolation_at_max_input() {
    // current_in == max_in => output == max_out
    let result = linear_interpolation(0u32, 100u32, 100u32, 200u32, 400u32).unwrap();
    assert_eq!(result, 400u32);
}

#[test]
fn linear_interpolation_at_midpoint() {
    // current_in at the midpoint => output at midpoint of output range
    let result = linear_interpolation(0u32, 100u32, 50u32, 0u32, 1000u32).unwrap();
    assert_eq!(result, 500u32);
}

#[test]
fn linear_interpolation_at_one_quarter() {
    // current_in at 25% => output at 25% of output range
    let result = linear_interpolation(0u32, 100u32, 25u32, 0u32, 1000u32).unwrap();
    assert_eq!(result, 250u32);
}

#[test]
fn linear_interpolation_non_zero_based_ranges() {
    // Input range [10, 50], output range [100, 200], current_in = 30 (50% through input)
    let result = linear_interpolation(10u32, 50u32, 30u32, 100u32, 200u32).unwrap();
    assert_eq!(result, 150u32);
}

#[test]
fn linear_interpolation_reversed_output_range() {
    // min_out > max_out is valid: output decreases as input increases
    // Input range [0, 100], output range [1000, 0], current_in = 25 => output = 750
    let result = linear_interpolation(0u32, 100u32, 25u32, 1000u32, 0u32).unwrap();
    assert_eq!(result, 750u32);
}

#[test]
fn linear_interpolation_below_range_returns_error() {
    let result = linear_interpolation(10u32, 100u32, 5u32, 0u32, 1000u32);
    assert!(matches!(result, Err(LinearInterpolationInvalidValuesError)));
}

#[test]
fn linear_interpolation_above_range_returns_error() {
    let result = linear_interpolation(0u32, 100u32, 110u32, 0u32, 1000u32);
    assert!(matches!(result, Err(LinearInterpolationInvalidValuesError)));
}

// ---- weighted_average ----

#[test]
fn weighted_average_equal_weights() {
    // (10 * 1 + 20 * 1) / (1 + 1) = 15
    let result = weighted_average(10u64, 1u64, 20u64, 1u64);
    assert_eq!(result, 15);
}

#[test]
fn weighted_average_all_weight_on_first() {
    // second_weight = 0 => result == first_value
    let result = weighted_average(10u64, 5u64, 99u64, 0u64);
    assert_eq!(result, 10);
}

#[test]
fn weighted_average_all_weight_on_second() {
    // first_weight = 0 => result == second_value
    let result = weighted_average(99u64, 0u64, 20u64, 5u64);
    assert_eq!(result, 20);
}

#[test]
fn weighted_average_three_to_one() {
    // (0 * 1 + 60 * 3) / (1 + 3) = 180 / 4 = 45
    let result = weighted_average(0u64, 1u64, 60u64, 3u64);
    assert_eq!(result, 45);
}

// ---- weighted_average_round_up ----

#[test]
fn weighted_average_round_up_exact_division() {
    // (10 * 1 + 20 * 1) / (1 + 1) = 15, no rounding needed
    let result = weighted_average_round_up(10u64, 1u64, 20u64, 1u64);
    assert_eq!(result, 15);
}

#[test]
fn weighted_average_round_up_truncates_vs_rounds() {
    // floor: (0 * 1 + 10 * 3) / (1 + 3) = 30 / 4 = 7
    // ceil:  (30 + 4 - 1) / 4 = 33 / 4 = 8
    let floor_result = weighted_average(0u64, 1u64, 10u64, 3u64);
    let ceil_result = weighted_average_round_up(0u64, 1u64, 10u64, 3u64);
    assert_eq!(floor_result, 7);
    assert_eq!(ceil_result, 8);
}

#[test]
fn weighted_average_round_up_no_change_when_exact() {
    // (0 * 1 + 20 * 3) / (1 + 3) = 60 / 4 = 15 exactly
    let result = weighted_average_round_up(0u64, 1u64, 20u64, 3u64);
    assert_eq!(result, 15);
}
