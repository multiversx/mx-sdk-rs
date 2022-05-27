use core::fmt::Debug;
use elrond_wasm::{
    api::ManagedTypeApi,
    types::{
        BigInt, BigUint, ManagedAddress, ManagedBuffer, ManagedByteArray, ManagedOption,
        ManagedType, TokenIdentifier,
    },
};
use elrond_wasm_debug::DebugApi;

fn test_some_for_value<M, T, F>(f: F)
where
    M: ManagedTypeApi,
    T: ManagedType<M> + Clone + Debug + Eq,
    F: Fn() -> T,
{
    assert!(ManagedOption::some(f()).is_some());
    assert_eq!(ManagedOption::some(f()), ManagedOption::some(f()));
    assert_eq!(ManagedOption::some(f()).clone(), ManagedOption::some(f()));
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
    assert_eq!(
        ManagedOption::<M, T>::none().clone(),
        ManagedOption::<M, T>::none()
    );
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
            .unwrap_or_else(|| BigUint::<DebugApi>::zero()),
        BigUint::<DebugApi>::from(1u32)
    );
    assert_eq!(
        ManagedOption::none().unwrap_or_else(|| BigUint::<DebugApi>::zero()),
        BigUint::<DebugApi>::zero()
    );
}
