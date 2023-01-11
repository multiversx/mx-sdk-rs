use multiversx_chain_vm::{check_managed_top_encode_decode, DebugApi};
use multiversx_sc::types::{
    BigInt, BigUint, BoxedBytes, ManagedAddress, ManagedBuffer, ManagedVec,
};

#[test]
fn test_big_uint_serialization() {
    let api = DebugApi::dummy();

    check_managed_top_encode_decode(api, BigUint::<DebugApi>::from(5u32), &[5u8]);
}

#[test]
fn test_vec_of_big_uint_serialization() {
    let api = DebugApi::dummy();
    let v = vec![
        BigUint::<DebugApi>::from(5u32),
        BigUint::<DebugApi>::from(6u32),
    ];

    check_managed_top_encode_decode(api, v, &[0, 0, 0, 1, 5, 0, 0, 0, 1, 6]);
}

#[test]
fn test_big_int_serialization() {
    let api = DebugApi::dummy();

    check_managed_top_encode_decode(api.clone(), BigInt::<DebugApi>::from(5), &[5u8]);
    check_managed_top_encode_decode(api, BigInt::<DebugApi>::from(-5), &[251u8]);
}

#[test]
fn test_vec_of_big_int_serialization() {
    let api = DebugApi::dummy();
    let v = vec![BigInt::<DebugApi>::from(5), BigInt::<DebugApi>::from(6)];

    check_managed_top_encode_decode(api, v, &[0, 0, 0, 1, 5, 0, 0, 0, 1, 6]);
}

#[test]
fn test_man_buf_serialization() {
    let api = DebugApi::dummy();

    check_managed_top_encode_decode(
        api,
        ManagedBuffer::<DebugApi>::new_from_bytes(&b"abc"[..]),
        &b"abc"[..],
    );
}

#[test]
fn test_vec_of_man_buf_serialization() {
    let api = DebugApi::dummy();
    let v = vec![
        ManagedBuffer::<DebugApi>::new_from_bytes(&b"abc"[..]),
        ManagedBuffer::<DebugApi>::new_from_bytes(&b"de"[..]),
    ];

    check_managed_top_encode_decode(
        api,
        v,
        &[0, 0, 0, 3, b'a', b'b', b'c', 0, 0, 0, 2, b'd', b'e'],
    );
}

#[test]
fn test_man_address_serialization() {
    let api = DebugApi::dummy();
    let v = ManagedAddress::<DebugApi>::new_from_bytes(&[7u8; 32]);

    check_managed_top_encode_decode(api, v, &[7u8; 32]);
}

#[test]
fn test_managed_vec_of_man_address_serialization() {
    let api = DebugApi::dummy();
    let mut v = ManagedVec::<DebugApi, ManagedAddress<DebugApi>>::new();
    v.push(ManagedAddress::new_from_bytes(&[7u8; 32]));
    v.push(ManagedAddress::new_from_bytes(&[8u8; 32]));
    v.push(ManagedAddress::new_from_bytes(&[9u8; 32]));

    let expected_v = BoxedBytes::from_concat(&[&[7u8; 32], &[8u8; 32], &[9u8; 32]]);

    check_managed_top_encode_decode(api.clone(), v.clone(), expected_v.as_slice());

    let option = Some(v);
    let expected_opt_v = BoxedBytes::from_concat(&[&[1], &[0, 0, 0, 3], expected_v.as_slice()]);

    check_managed_top_encode_decode(api, option, expected_opt_v.as_slice());
}
