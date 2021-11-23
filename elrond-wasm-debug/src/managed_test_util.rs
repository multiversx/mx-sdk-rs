use elrond_wasm::{
    contract_base::ManagedSerializer,
    elrond_codec::{test_util::check_top_encode, DecodeError, TopDecode, TopEncode},
    types::{BoxedBytes, ManagedBuffer},
};

use core::fmt::Debug;

use crate::DebugApi;

/// Uses the managed types api to test encoding.
/// Can be used on any type, but managed types are especially relevant.
pub fn check_managed_top_encode<T: TopEncode>(api: DebugApi, obj: &T) -> BoxedBytes {
    let serializer = ManagedSerializer::new(api);
    let as_mb = serializer.top_encode_to_managed_buffer(obj);
    let as_bb = serializer.top_encode_to_boxed_bytes(obj);
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
    api: DebugApi,
    bytes: &[u8],
) -> T {
    let serializer = ManagedSerializer::new(api);
    let mb = ManagedBuffer::new_from_bytes(bytes);
    let from_mb: T = serializer.top_decode_from_managed_buffer(&mb);
    let from_slice: T = serializer.top_decode_from_byte_slice(bytes);
    assert_eq!(from_mb, from_slice);

    match T::top_decode(bytes) {
        Ok(from_unmanaged) => {
            assert_eq!(from_unmanaged, from_mb);
        },
        Err(DecodeError::UNSUPPORTED_OPERATION) => {
            // Ok
        },
        Err(err) => {
            panic!(
                "Unexpected encoding error:: {}",
                core::str::from_utf8(err.message_bytes()).unwrap()
            )
        },
    }

    from_mb
}

/// Uses the managed types api to test encoding both ways.
pub fn check_managed_top_encode_decode<V>(api: DebugApi, element: V, expected_bytes: &[u8])
where
    V: TopEncode + TopDecode + PartialEq + Debug + 'static,
{
    // serialize
    let serialized_bytes = check_managed_top_encode(api.clone(), &element);
    assert_eq!(serialized_bytes.as_slice(), expected_bytes);

    // deserialize
    let deserialized: V = check_managed_top_decode::<V>(api, serialized_bytes.as_slice());
    assert_eq!(deserialized, element);
}
