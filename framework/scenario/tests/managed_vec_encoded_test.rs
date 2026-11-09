use multiversx_sc::codec::{TopDecode, TopEncode};
use multiversx_sc::types::{BigUint, ManagedBuffer, ManagedVec, ManagedVecEncoded, Payment};
use multiversx_sc_scenario::api::StaticApi;

fn payment(i: u64) -> Payment<StaticApi> {
    Payment::new(
        "TOKEN-123456",
        i,
        BigUint::from(1000 + i).into_non_zero_or_panic(),
    )
}

#[test]
fn managed_vec_encoded_push_iter_test() {
    let mut mve = ManagedVecEncoded::<StaticApi, u32>::new();
    assert!(mve.is_empty());
    assert_eq!(mve.iter().next(), None);

    mve.push(1);
    assert!(!mve.is_empty());
    mve.push(2);
    mve.push(3);

    assert_eq!(mve.iter().collect::<Vec<_>>(), vec![1, 2, 3]);
    // iterating by reference leaves the original intact
    assert_eq!((&mve).into_iter().count(), 3);
    assert_eq!(mve.into_iter().collect::<Vec<_>>(), vec![1, 2, 3]);
}

#[test]
fn managed_vec_encoded_payment_test() {
    let mut mve = ManagedVecEncoded::<StaticApi, Payment<StaticApi>>::new();
    let mut vec = ManagedVec::<StaticApi, Payment<StaticApi>>::new();
    for i in 0..5 {
        mve.push(payment(i));
        vec.push(payment(i));
    }

    // same encoding as ManagedVec
    let mut encoded_vec = ManagedBuffer::<StaticApi>::new();
    vec.top_encode(&mut encoded_vec).unwrap();
    let mut encoded_mve = ManagedBuffer::<StaticApi>::new();
    mve.top_encode(&mut encoded_mve).unwrap();
    assert_eq!(encoded_mve, encoded_vec);
    assert_eq!(mve.as_raw_buffer(), &encoded_vec);

    // top-decoding does not change the buffer
    let decoded =
        ManagedVecEncoded::<StaticApi, Payment<StaticApi>>::top_decode(encoded_vec).unwrap();
    assert_eq!(decoded.as_raw_buffer(), mve.as_raw_buffer());

    // conversions
    let from_vec: ManagedVecEncoded<StaticApi, Payment<StaticApi>> = vec.clone().into();
    assert_eq!(from_vec.as_raw_buffer(), mve.as_raw_buffer());
    let back: ManagedVec<StaticApi, Payment<StaticApi>> = mve.clone().into();
    assert_eq!(back, vec);

    for (i, item) in mve.iter().enumerate() {
        assert_eq!(item, payment(i as u64));
    }
}
