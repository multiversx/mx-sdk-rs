use crate::*;
use core::fmt::Debug;

pub fn ser_deser_ok<V>(element: V, expected_bytes: &[u8])
where
    V: TopEncode + TopDecode + PartialEq + Debug,
{
    // serialize
    let serialized_bytes = top_encode_to_vec(&element).unwrap();
    assert_eq!(serialized_bytes.as_slice(), expected_bytes);

    // deserialize
    let deserialized: V = V::top_decode(&serialized_bytes[..]).unwrap();
    assert_eq!(deserialized, element);
}
