use multiversx_sc::types::{BigUint, ManagedBuffer, ManagedMapEncoded};
use multiversx_sc_scenario::api::StaticApi;

type ManagedMapEncodedBig =
    ManagedMapEncoded<StaticApi, BigUint<StaticApi>, ManagedBuffer<StaticApi>>;

fn assert_missing_key_int(mme: &ManagedMapEncoded<StaticApi, i32, i64>, key: i32) {
    assert!(!mme.contains(&key));
    assert_eq!(mme.get(&key), 0);
}

fn assert_missing_key_big(mme: &ManagedMapEncodedBig, key: &BigUint<StaticApi>) {
    assert!(!mme.contains(key));
    assert_eq!(mme.get(key), ManagedBuffer::new());
}

#[test]
fn managed_map_encoded_int_test() {
    let key = 1;
    let mut mme = ManagedMapEncoded::<StaticApi, i32, i64>::new();

    assert_missing_key_int(&mme, key);

    let value = 10;

    mme.put(&key, &value);
    assert!(mme.contains(&key));
    assert_eq!(mme.get(&key), value);

    assert_eq!(mme.remove(&key), value);
    assert_missing_key_int(&mme, key);

    assert_eq!(mme.remove(&key), 0);
    assert_missing_key_int(&mme, key);

    let value = 0;

    mme.put(&key, &value);
    assert_missing_key_int(&mme, key);

    assert_eq!(mme.remove(&key), 0);
    assert_missing_key_int(&mme, key);
}

#[test]
fn managed_map_encoded_big_test() {
    let mut mme =
        ManagedMapEncoded::<StaticApi, BigUint<StaticApi>, ManagedBuffer<StaticApi>>::new();

    let key = BigUint::from(1u32);
    assert_missing_key_big(&mme, &key);

    let value = ManagedBuffer::from("abc");
    let empty = ManagedBuffer::new();

    mme.put(&key, &value);
    assert!(mme.contains(&key));
    assert_eq!(&mme.get(&key), &value);

    assert_eq!(&mme.remove(&key), &value);
    assert_missing_key_big(&mme, &key);

    assert_eq!(&mme.remove(&key), &empty);
    assert_missing_key_big(&mme, &key);

    mme.put(&key, &empty);
    assert_missing_key_big(&mme, &key);

    assert_eq!(&mme.remove(&key), &empty);
    assert_missing_key_big(&mme, &key);
}
