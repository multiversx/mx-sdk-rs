use core::fmt::Debug;
use multiversx_sc::{
    api::ManagedTypeApi,
    types::{
        BigInt, BigUint, ManagedAddress, ManagedBuffer, ManagedByteArray, ManagedRef, ManagedType,
        TokenIdentifier,
    },
};
use multiversx_sc_scenario::api::StaticApi;

fn test_managed_ref_for_type<M, T>(obj: T)
where
    M: ManagedTypeApi,
    T: ManagedType<M> + Clone + Debug + Eq,
{
    let obj_ref = obj.as_ref();
    assert_eq!(
        obj_ref.get_handle(),
        ManagedRef::get_raw_handle_of_ref(obj_ref.clone())
    );
    let obj_clone: T = Clone::clone(&obj_ref);
    assert_eq!(obj, obj_clone);
    assert_ne!(obj_ref.get_handle(), obj_clone.get_handle());
}

#[test]
fn test_managed_ref() {
    test_managed_ref_for_type(BigUint::<StaticApi>::from(1u32));
    test_managed_ref_for_type(BigInt::<StaticApi>::from(2i32));
    test_managed_ref_for_type(ManagedBuffer::<StaticApi>::from(&b"3abc"[..]));
    test_managed_ref_for_type(ManagedByteArray::<StaticApi, 4>::from(&[4u8; 4]));
    test_managed_ref_for_type(ManagedAddress::<StaticApi>::from(&[5u8; 32]));
    test_managed_ref_for_type(TokenIdentifier::<StaticApi>::from(&b"TOKEN-000006"[..]));
}

#[test]
fn test_managed_ref_clone() {
    let obj = BigUint::<StaticApi>::from(7u32);
    let obj_ref = obj.as_ref();
    assert_eq!(obj.get_handle(), obj_ref.get_handle());

    let obj_clone = Clone::clone(&*obj_ref);
    assert_eq!(obj, obj_clone);
    assert_ne!(obj.get_handle(), obj_clone.get_handle());
}

#[test]
fn test_managed_ref_eq() {
    assert_eq!(
        BigUint::<StaticApi>::from(1u32).as_ref(),
        BigUint::<StaticApi>::from(1u32).as_ref()
    );

    assert_ne!(
        BigUint::<StaticApi>::from(1u32).as_ref(),
        BigUint::<StaticApi>::from(2u32).as_ref()
    );
}
