use multiversx_sc::api::ManagedTypeApi;
use multiversx_sc::codec::derive::{NestedDecode, NestedEncode, TopDecode, TopEncode};
use multiversx_sc::codec::{self, DecodeDefault, EncodeDefault};
use multiversx_sc::proxy_imports::{TopDecodeOrDefault, TopEncodeOrDefault};
use multiversx_sc::types::{BigUint, ManagedBuffer, ManagedMapEncoded};
use multiversx_sc_scenario::api::StaticApi;

type ManagedMapEncodedBig =
    ManagedMapEncoded<StaticApi, BigUint<StaticApi>, ManagedBuffer<StaticApi>>;

fn assert_missing_key_int(mme: &ManagedMapEncoded<StaticApi, i32, i64>, key: i32) {
    assert!(!mme.contains(&key));
    assert_eq!(mme.get(&key), 0);
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

fn assert_missing_key_big(mme: &ManagedMapEncodedBig, key: &BigUint<StaticApi>) {
    assert!(!mme.contains(key));
    assert_eq!(mme.get(key), ManagedBuffer::new());
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

#[derive(TopEncode, TopDecode)]
pub struct StructKey {
    a: i32,
    b: i32,
}

#[derive(NestedEncode, NestedDecode, PartialEq, Debug)]
pub struct StructValue<M: ManagedTypeApi> {
    x: i32,
    y: ManagedBuffer<M>,
}

fn assert_missing_opt_struct(
    mme: &ManagedMapEncoded<StaticApi, StructKey, Option<StructValue<StaticApi>>>,
    key: &StructKey,
) {
    assert!(!mme.contains(key));
    assert_eq!(mme.get(key), None);
}

#[test]
fn managed_map_encoded_opt_struct_test() {
    let mut mme = ManagedMapEncoded::<StaticApi, StructKey, Option<StructValue<StaticApi>>>::new();

    let key = StructKey { a: 1, b: 2 };
    assert_missing_opt_struct(&mme, &key);

    let value = Some(StructValue {
        x: 3,
        y: ManagedBuffer::from("abc"),
    });

    mme.put(&key, &value);
    assert!(mme.contains(&key));
    assert_eq!(&mme.get(&key), &value);

    assert_eq!(&mme.remove(&key), &value);
    assert_missing_opt_struct(&mme, &key);

    assert_eq!(&mme.remove(&key), &None);
    assert_missing_opt_struct(&mme, &key);

    mme.put(&key, &None);
    assert_missing_opt_struct(&mme, &key);

    assert_eq!(&mme.remove(&key), &None);
    assert_missing_opt_struct(&mme, &key);
}

#[derive(TopEncode, TopDecode)]
pub struct ManagedStructKey<M: ManagedTypeApi> {
    a: BigUint<M>,
    b: BigUint<M>,
}

#[derive(TopEncodeOrDefault, TopDecodeOrDefault, PartialEq, Debug)]
pub struct StructValueOrDefault<M: ManagedTypeApi> {
    x: i32,
    y: ManagedBuffer<M>,
}

impl<M: ManagedTypeApi> EncodeDefault for StructValueOrDefault<M> {
    fn is_default(&self) -> bool {
        self.x == 0
    }
}

impl<M: ManagedTypeApi> DecodeDefault for StructValueOrDefault<M> {
    fn default() -> Self {
        StructValueOrDefault {
            x: 0,
            y: ManagedBuffer::new(),
        }
    }
}

fn assert_missing_struct(
    mme: &ManagedMapEncoded<
        StaticApi,
        ManagedStructKey<StaticApi>,
        StructValueOrDefault<StaticApi>,
    >,
    key: &ManagedStructKey<StaticApi>,
) {
    assert!(!mme.contains(key));
    assert_eq!(mme.get(key), StructValueOrDefault::default());
}

#[test]
fn managed_map_encoded_struct_or_default_test() {
    let mut mme = ManagedMapEncoded::<
        StaticApi,
        ManagedStructKey<StaticApi>,
        StructValueOrDefault<StaticApi>,
    >::new();

    let key = ManagedStructKey {
        a: 1u32.into(),
        b: 2u32.into(),
    };
    assert_missing_struct(&mme, &key);

    let value = StructValueOrDefault {
        x: 3,
        y: ManagedBuffer::from("abc"),
    };
    let default = StructValueOrDefault::default();

    mme.put(&key, &value);
    assert!(mme.contains(&key));
    assert_eq!(&mme.get(&key), &value);

    assert_eq!(&mme.remove(&key), &value);
    assert_missing_struct(&mme, &key);

    assert_eq!(&mme.remove(&key), &default);
    assert_missing_struct(&mme, &key);

    mme.put(&key, &default);
    assert_missing_struct(&mme, &key);

    assert_eq!(&mme.remove(&key), &default);
    assert_missing_struct(&mme, &key);
}
