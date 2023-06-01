use multiversx_sc::{
    codec::{test_util::check_top_encode, TopDecode, TopEncode},
    contract_base::ManagedSerializer,
    types::{heap::BoxedBytes, ManagedBuffer},
};

use core::fmt::Debug;

use crate::api::StaticApi;

/// Uses the managed types api to test encoding.
/// Can be used on any type, but managed types are especially relevant.
pub fn check_managed_top_encode<T: TopEncode>(obj: &T) -> BoxedBytes {
    let serializer = ManagedSerializer::<StaticApi>::new();
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
pub fn check_managed_top_decode<T: TopDecode + PartialEq + Debug>(bytes: &[u8]) -> T {
    let serializer = ManagedSerializer::<StaticApi>::new();
    let mb = ManagedBuffer::new_from_bytes(bytes);
    let from_mb: T = serializer.top_decode_from_managed_buffer(&mb);
    let from_slice: T = serializer.top_decode_from_byte_slice(bytes);
    assert_eq!(from_mb, from_slice);

    match T::top_decode(bytes) {
        Ok(from_unmanaged) => {
            assert_eq!(from_unmanaged, from_mb);
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
pub fn check_managed_top_encode_decode<V>(element: V, expected_bytes: &[u8])
where
    V: TopEncode + TopDecode + PartialEq + Debug + 'static,
{
    // serialize
    let serialized_bytes = check_managed_top_encode(&element);
    assert_eq!(serialized_bytes.as_slice(), expected_bytes);

    // deserialize
    let deserialized: V = check_managed_top_decode::<V>(serialized_bytes.as_slice());
    assert_eq!(deserialized, element);
}
