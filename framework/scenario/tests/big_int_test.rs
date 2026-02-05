use multiversx_sc::types::BigInt;
use multiversx_sc_scenario::api::StaticApi;

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
