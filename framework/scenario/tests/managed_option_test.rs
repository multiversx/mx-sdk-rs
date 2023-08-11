use core::fmt::Debug;
use multiversx_sc::{
    api::ManagedTypeApi,
    types::{
        BigInt, BigUint, ManagedAddress, ManagedBuffer, ManagedByteArray, ManagedOption,
        ManagedType, TokenIdentifier,
    },
};
use multiversx_sc_scenario::api::StaticApi;

fn test_some_for_value<M, T, F>(f: F)
where
    M: ManagedTypeApi,
    T: ManagedType<M> + Clone + Debug + Eq,
    F: Fn() -> T,
{
    assert!(ManagedOption::some(f()).is_some());
    assert_eq!(ManagedOption::some(f()), ManagedOption::some(f()));
    assert_eq!(ManagedOption::some(f()), ManagedOption::some(f()));
    assert_eq!(ManagedOption::from(Some(f())), ManagedOption::some(f()));
    assert_eq!(ManagedOption::some(f()).into_option(), Some(f()));
    assert_ne!(ManagedOption::some(f()), ManagedOption::<M, T>::none());
}

fn test_none_for_type<M, T>()
where
    M: ManagedTypeApi,
    T: ManagedType<M> + Clone + Debug + Eq,
{
    assert!(ManagedOption::<M, T>::none().is_none());
    assert_eq!(ManagedOption::<M, T>::none(), ManagedOption::<M, T>::none());
    assert_eq!(ManagedOption::<M, T>::none(), ManagedOption::<M, T>::none());
    assert_eq!(ManagedOption::from(None), ManagedOption::<M, T>::none());
    assert_eq!(ManagedOption::<M, T>::none().into_option(), None);
}

#[test]
fn test_some() {
    test_some_for_value(|| BigUint::<StaticApi>::from(1u32));
    test_some_for_value(|| BigInt::<StaticApi>::from(2i32));
    test_some_for_value(|| ManagedBuffer::<StaticApi>::from(&b"3abc"[..]));
    test_some_for_value(|| ManagedByteArray::<StaticApi, 4>::from(&[4u8; 4]));
    test_some_for_value(|| ManagedAddress::<StaticApi>::from(&[5u8; 32]));
    test_some_for_value(|| TokenIdentifier::<StaticApi>::from(&b"TOKEN-000006"[..]));
}

#[test]
fn test_none() {
    test_none_for_type::<StaticApi, BigUint<StaticApi>>();
    test_none_for_type::<StaticApi, BigInt<StaticApi>>();
    test_none_for_type::<StaticApi, ManagedBuffer<StaticApi>>();
    test_none_for_type::<StaticApi, ManagedByteArray<StaticApi, 4>>();
    test_none_for_type::<StaticApi, ManagedAddress<StaticApi>>();
    test_none_for_type::<StaticApi, TokenIdentifier<StaticApi>>();
}

#[test]
fn test_unwrap() {
    assert_eq!(
        ManagedOption::some(BigUint::<StaticApi>::from(1u32))
            .unwrap_or_else(BigUint::<StaticApi>::zero),
        BigUint::<StaticApi>::from(1u32)
    );
    assert_eq!(
        ManagedOption::none().unwrap_or_else(BigUint::<StaticApi>::zero),
        BigUint::<StaticApi>::zero()
    );
}

#[test]
fn test_map() {
    // example BigInt -> BigUint
    assert_eq!(
        ManagedOption::some(BigUint::<StaticApi>::from(1u32)).map(BigInt::<StaticApi>::from),
        ManagedOption::some(BigInt::<StaticApi>::from(1i32))
    );
    assert_eq!(
        ManagedOption::<StaticApi, BigUint::<StaticApi>>::none().map(BigInt::<StaticApi>::from),
        ManagedOption::none()
    );

    // example BigUint -> BigInt (magnitude)
    assert_eq!(
        ManagedOption::some(BigInt::<StaticApi>::from(-1i32)).map(|x| x.magnitude()),
        ManagedOption::some(BigUint::<StaticApi>::from(1u32))
    );
    assert_eq!(
        ManagedOption::none().map(|x: BigInt<StaticApi>| x.magnitude()),
        ManagedOption::none()
    );

    // BigInt::into_big_uint is actually related
    assert_eq!(
        BigInt::<StaticApi>::from(1i32).into_big_uint(),
        ManagedOption::some(BigUint::<StaticApi>::from(1u32))
    );
    assert_eq!(
        BigInt::<StaticApi>::from(-1i32).into_big_uint(),
        ManagedOption::none()
    );
}
