use crate::*;
use core::fmt::Debug;

pub fn ser_deser_ok<V>(element: V, expected_bytes: &[u8])
where
    V: Encode + NestedDecode + PartialEq + Debug + 'static,
{
    // serialize
    let serialized_bytes = element.top_encode().unwrap();
    assert_eq!(serialized_bytes.as_slice(), expected_bytes);

    // deserialize
    let deserialized: V = V::top_decode_old(&mut &serialized_bytes[..]).unwrap();
    assert_eq!(deserialized, element);
}
