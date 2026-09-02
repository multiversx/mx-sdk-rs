use multiversx_sc::types::BigInt;
use multiversx_sc_scenario::api::StaticApi;

// BigInt intentionally does not implement Send or Sync,
// since it holds a managed handle that is only valid on the thread of the original context.
static_assertions::assert_not_impl_any!(BigInt::<StaticApi>: Send, Sync);

#[test]
fn test_big_int_add() {
    let x = BigInt::<StaticApi>::from(2);
    let y = BigInt::<StaticApi>::from(3);
    assert_eq!(x + y, BigInt::<StaticApi>::from(5))
}

fn assert_big_int_proportion(total: i64, part: i64, denom: i64, expected: i64) {
    let total = BigInt::<StaticApi>::from(total);
    let expected = BigInt::<StaticApi>::from(expected);
    assert_eq!(total.proportion(part, denom), expected);
    assert_eq!(total.clone().into_proportion(part, denom), expected);
}

#[test]
fn test_big_int_display() {
    assert_eq!(BigInt::<StaticApi>::from(0).to_string(), "0");
    assert_eq!(BigInt::<StaticApi>::from(1).to_string(), "1");
    assert_eq!(BigInt::<StaticApi>::from(-1).to_string(), "-1");
    assert_eq!(BigInt::<StaticApi>::from(42).to_string(), "42");
    assert_eq!(BigInt::<StaticApi>::from(-42).to_string(), "-42");
    assert_eq!(BigInt::<StaticApi>::from(1000000).to_string(), "1000000");
    assert_eq!(BigInt::<StaticApi>::from(-1000000).to_string(), "-1000000");
    assert_eq!(
        BigInt::<StaticApi>::from(i64::MAX).to_string(),
        i64::MAX.to_string()
    );
    assert_eq!(
        BigInt::<StaticApi>::from(i64::MIN).to_string(),
        i64::MIN.to_string()
    );
    // format! also uses Display
    assert_eq!(format!("{}", BigInt::<StaticApi>::from(123)), "123");
    assert_eq!(format!("{}", BigInt::<StaticApi>::from(-123)), "-123");
}

#[test]
fn test_big_int_proportion_all() {
    assert_big_int_proportion(1000, 0, 100, 0);
    assert_big_int_proportion(1000, 25, 100, 250);
    assert_big_int_proportion(1000, 50, 100, 500);
    assert_big_int_proportion(1000, 75, 100, 750);
    assert_big_int_proportion(1000, 100, 100, 1000);
    assert_big_int_proportion(-1000, 25, 100, -250);
    assert_big_int_proportion(-1000, 50, 100, -500);
    assert_big_int_proportion(2000, 1, 4, 500);
    assert_big_int_proportion(2000, 3, 4, 1500);
    assert_big_int_proportion(100, 1, 3, 33); // 33.333... -> 33
}
