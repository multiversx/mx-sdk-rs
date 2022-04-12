use elrond_wasm::{
    elrond_codec::multi_types::MultiValue5,
    types::{BigUint, MultiValueEncoded},
};
use elrond_wasm_debug::DebugApi;

#[test]
fn test_multi_value_encoded_1() {
    let _ = DebugApi::dummy();

    let mut multi_value_1 = MultiValueEncoded::<DebugApi, BigUint<DebugApi>>::new();
    for i in 20u64..=30u64 {
        multi_value_1.push(BigUint::from(i));
    }
    assert_eq!(multi_value_1.len(), 11);
}

#[test]
fn test_multi_value_encoded_5() {
    let _ = DebugApi::dummy();

    let mut multi_value_1 =
        MultiValueEncoded::<DebugApi, MultiValue5<u64, u64, u64, u64, u64>>::new();
    for i in 20u64..=30u64 {
        multi_value_1.push(MultiValue5::from((i, i, i, i, i)));
    }
    assert_eq!(multi_value_1.len(), 11);
}
