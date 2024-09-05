pub fn denominate(value: f64) -> u128 {
    let multiplier: f64 = 10f64.powi(18);
    let result = value * multiplier;

    if result < 0.0 {
        panic!("Negative values are not allowed.");
    }
    if result > u128::MAX as f64 {
        panic!("Result is too large to fit in u128.");
    }

    result as u128
}
