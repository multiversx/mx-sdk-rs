use elrond_wasm::{
    api::ContractBase,
    elrond_codec::{test_util::check_top_encode, DecodeError, TopDecode, TopEncode},
    types::{BoxedBytes, ManagedBuffer},
};

use crate::TxContext;

use core::fmt::Debug;

/// Uses the managed types api to test encoding.
/// Can be used on any type, but managed types are especially relevant.
pub fn check_managed_top_encode<T: TopEncode>(api: TxContext, obj: &T) -> BoxedBytes {
    let as_mb = api.serializer().top_encode_to_managed_buffer(obj);
    let as_bb = api.serializer().top_encode_to_boxed_bytes(obj);
    assert_eq!(as_mb.to_boxed_bytes(), as_bb);

    // also check classic encoding
    let unmanaged_encoded = check_top_encode(obj);
    assert_eq!(
        as_mb.to_boxed_bytes().as_slice(),
        unmanaged_encoded.as_slice()
    );

    as_bb
}

/// Uses the managed types api to test encoding.
/// Can be used on any type, but managed types are especially relevant.
/// Also works on types that have no un-managed decoding,
/// by allowing DecodeError::UNSUPPORTED_OPERATION result.
pub fn check_managed_top_decode<T: TopDecode + PartialEq + Debug>(
    api: TxContext,
    bytes: &[u8],
) -> T {
    let mb = ManagedBuffer::new_from_bytes(api.clone(), bytes);
    let from_mb: T = api.serializer().top_decode_from_managed_buffer(&mb);
    let from_slice: T = api.serializer().top_decode_from_byte_slice(bytes);
    assert_eq!(from_mb, from_slice);

    match T::top_decode(bytes) {
        Ok(from_unmanaged) => {
            assert_eq!(from_unmanaged, from_mb);
        },
        Err(DecodeError::UNSUPPORTED_OPERATION) => {
            // Ok
        },
        Err(_) => {
            panic!("Unexpected encoding error");
        },
    }

    from_mb
}
