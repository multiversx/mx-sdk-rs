use core::fmt::Debug;
use multiversx_chain_vm::DebugApi;
use multiversx_sc::{
    api::ManagedTypeApi,
    types::{
        BigInt, BigUint, ManagedAddress, ManagedBuffer, ManagedByteArray, ManagedOption,
        ManagedType, TokenIdentifier,
    },
};

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
    let _ = DebugApi::dummy();

    test_some_for_value(|| BigUint::<DebugApi>::from(1u32));
    test_some_for_value(|| BigInt::<DebugApi>::from(2i32));
    test_some_for_value(|| ManagedBuffer::<DebugApi>::from(&b"3abc"[..]));
    test_some_for_value(|| ManagedByteArray::<DebugApi, 4>::from(&[4u8; 4]));
    test_some_for_value(|| ManagedAddress::<DebugApi>::from(&[5u8; 32]));
    test_some_for_value(|| TokenIdentifier::<DebugApi>::from(&b"TOKEN-000006"[..]));
}

#[test]
fn test_none() {
    let _ = DebugApi::dummy();

    test_none_for_type::<DebugApi, BigUint<DebugApi>>();
    test_none_for_type::<DebugApi, BigInt<DebugApi>>();
    test_none_for_type::<DebugApi, ManagedBuffer<DebugApi>>();
    test_none_for_type::<DebugApi, ManagedByteArray<DebugApi, 4>>();
    test_none_for_type::<DebugApi, ManagedAddress<DebugApi>>();
    test_none_for_type::<DebugApi, TokenIdentifier<DebugApi>>();
}

#[test]
fn test_unwrap() {
    let _ = DebugApi::dummy();

    assert_eq!(
        ManagedOption::some(BigUint::<DebugApi>::from(1u32))
            .unwrap_or_else(BigUint::<DebugApi>::zero),
        BigUint::<DebugApi>::from(1u32)
    );
    assert_eq!(
        ManagedOption::none().unwrap_or_else(BigUint::<DebugApi>::zero),
        BigUint::<DebugApi>::zero()
    );
}

#[test]
fn test_map() {
    let _ = DebugApi::dummy();

    // example BigInt -> BigUint
    assert_eq!(
        ManagedOption::some(BigUint::<DebugApi>::from(1u32)).map(BigInt::<DebugApi>::from),
        ManagedOption::some(BigInt::<DebugApi>::from(1i32))
    );
    assert_eq!(
        ManagedOption::<DebugApi, BigUint::<DebugApi>>::none().map(BigInt::<DebugApi>::from),
        ManagedOption::none()
    );

    // example BigUint -> BigInt (magnitude)
    assert_eq!(
        ManagedOption::some(BigInt::<DebugApi>::from(-1i32)).map(|x| x.magnitude()),
        ManagedOption::some(BigUint::<DebugApi>::from(1u32))
    );
    assert_eq!(
        ManagedOption::none().map(|x: BigInt<DebugApi>| x.magnitude()),
        ManagedOption::none()
    );

    // BigInt::into_big_uint is actually related
    assert_eq!(
        BigInt::<DebugApi>::from(1i32).into_big_uint(),
        ManagedOption::some(BigUint::<DebugApi>::from(1u32))
    );
    assert_eq!(
        BigInt::<DebugApi>::from(-1i32).into_big_uint(),
        ManagedOption::none()
    );
}
