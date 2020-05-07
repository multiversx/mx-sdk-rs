use core::fmt::Debug;
use elrond_wasm::serializer::{to_bytes, from_bytes};
use elrond_wasm::serde as serde;

pub fn the_same<V>(element: V)
where
    V: serde::Serialize + serde::de::DeserializeOwned + PartialEq + Debug + 'static,
{
    let serialized_bytes = to_bytes(&element).unwrap();
    let deserialized: V = from_bytes(serialized_bytes.as_slice()).unwrap();
    assert_eq!(deserialized, element);
}

pub fn ser_deser_ok<V>(element: V, expected_bytes: &[u8])
where
    V: serde::Serialize + serde::de::DeserializeOwned + PartialEq + Debug + 'static,
{
    // serialize
    let serialized_bytes = to_bytes(&element).unwrap();
    assert_eq!(serialized_bytes.as_slice(), expected_bytes);

    // deserialize
    let deserialized: V = from_bytes(serialized_bytes.as_slice()).unwrap();
    assert_eq!(deserialized, element);
}
