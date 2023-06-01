use multiversx_sc::{
    codec::multi_types::MultiValue5,
    types::{BigUint, MultiValueEncoded},
};
use multiversx_sc_scenario::api::StaticApi;

#[test]
fn test_multi_value_encoded_1() {
    let mut multi_value_1 = MultiValueEncoded::<StaticApi, BigUint<StaticApi>>::new();
    for i in 20u64..=30u64 {
        multi_value_1.push(BigUint::from(i));
    }
    assert_eq!(multi_value_1.len(), 11);
}

#[test]
fn test_multi_value_encoded_5() {
    let mut multi_value_1 =
        MultiValueEncoded::<StaticApi, MultiValue5<u64, u64, u64, u64, u64>>::new();
    for i in 20u64..=30u64 {
        multi_value_1.push(MultiValue5::from((i, i, i, i, i)));
    }
    assert_eq!(multi_value_1.len(), 11);
}

#[test]
fn test_multi_value5_eq_test() {
    let multi_value1 = MultiValue5::from((1, 2, 3, 4, 5));
    let multi_value2 = MultiValue5::from((1, 2, 3, 4, 5));
    assert_eq!(multi_value1, multi_value2)
}

#[test]
fn test_multi_value5_ne_test() {
    let multi_value1 = MultiValue5::from((1, 2, 3, 4, 5));
    let multi_value2 = MultiValue5::from((1, 2, 3, 4, 4));
    assert_ne!(multi_value1, multi_value2)
}

#[test]
fn test_multi_value5_from_iterator_trait() {
    let mut multi_value1 =
        MultiValueEncoded::<StaticApi, MultiValue5<u64, u64, u64, u64, u64>>::new();
    for i in 1..=10 {
        multi_value1.push(MultiValue5::from((i, i, i, i, i)));
    }
    let mut multi_value_expected =
        MultiValueEncoded::<StaticApi, MultiValue5<u64, u64, u64, u64, u64>>::new();
    for i in 1..=5 {
        multi_value_expected.push(MultiValue5::from((i, i, i, i, i)));
    }

    let collected_vec = multi_value1
        .into_iter()
        .map(|x| x.into_tuple())
        .filter(|(x, _, _, _, _)| x <= &5)
        .map(|t| t.into())
        .collect::<MultiValueEncoded<StaticApi, MultiValue5<u64, u64, u64, u64, u64>>>();

    assert_eq!(collected_vec, multi_value_expected);
}
