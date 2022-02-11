use core::fmt::Debug;
use elrond_wasm::{
    api::ManagedTypeApi,
    types::{
        BigInt, BigUint, ManagedAddress, ManagedBuffer, ManagedByteArray, ManagedRef, ManagedType,
        TokenIdentifier,
    },
};
use elrond_wasm_debug::DebugApi;

fn test_managed_ref_for_type<M, T>(obj: T)
where
    M: ManagedTypeApi,
    T: ManagedType<M> + Clone + Debug + Eq,
{
    let obj_ref = obj.as_ref();
    assert_eq!(
        obj_ref.get_raw_handle(),
        ManagedRef::get_raw_handle_of_ref(obj_ref)
    );
    let obj_clone: T = Clone::clone(&obj_ref);
    assert_eq!(obj, obj_clone);
    assert_ne!(obj_ref.get_raw_handle(), obj_clone.get_raw_handle());
}

#[test]
fn test_managed_ref() {
    let _ = DebugApi::dummy();

    test_managed_ref_for_type(BigUint::<DebugApi>::from(1u32));
    test_managed_ref_for_type(BigInt::<DebugApi>::from(2i32));
    test_managed_ref_for_type(ManagedBuffer::<DebugApi>::from(&b"3abc"[..]));
    test_managed_ref_for_type(ManagedByteArray::<DebugApi, 4>::from(&[4u8; 4]));
    test_managed_ref_for_type(ManagedAddress::<DebugApi>::from(&[5u8; 32]));
    test_managed_ref_for_type(TokenIdentifier::<DebugApi>::from(&b"TOKEN-000006"[..]));
}

#[test]
fn test_managed_ref_clone() {
    let _ = DebugApi::dummy();

    let obj = BigUint::<DebugApi>::from(7u32);
    let obj_ref = obj.as_ref();
    assert_eq!(obj.get_raw_handle(), obj_ref.get_raw_handle());

    let obj_clone = Clone::clone(&*obj_ref);
    assert_eq!(obj, obj_clone);
    assert_ne!(obj.get_raw_handle(), obj_clone.get_raw_handle());
}

#[test]
fn test_managed_ref_eq() {
    let _ = DebugApi::dummy();

    assert_eq!(
        BigUint::<DebugApi>::from(1u32).as_ref(),
        BigUint::<DebugApi>::from(1u32).as_ref()
    );

    assert_ne!(
        BigUint::<DebugApi>::from(1u32).as_ref(),
        BigUint::<DebugApi>::from(2u32).as_ref()
    );
}
