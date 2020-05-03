


use serde;
use serde::Deserialize;
use super::bytes_err::{SDError, Result};

const USIZE_SIZE: usize = 4; // wasm32

pub struct ErdDeserializer<'de> {
    // This string starts with the input data and characters are truncated off
    // the beginning as data is parsed.
    input: &'de [u8],
    top_level: bool,
}

impl<'de> ErdDeserializer<'de> {
    // By convention, `Deserializer` constructors are named like `from_xyz`.
    // That way basic use cases are satisfied by something like
    // `serde_json::from_str(...)` while advanced use cases that require a
    // deserializer can make one with `serde_json::Deserializer::from_str(...)`.
    fn from_bytes(input: &'de [u8]) -> Self {
        ErdDeserializer { input, top_level: true }
    }
}

// By convention, the public API of a Serde deserializer is one or more
// `from_xyz` methods such as `from_str`, `from_bytes`, or `from_reader`
// depending on what Rust types the deserializer is able to consume as input.
//
// This basic deserializer supports only `from_str`.
pub fn from_bytes<'a, T>(bytes: &'a [u8]) -> Result<T>
where
    T: Deserialize<'a>,
{
    let mut deserializer = ErdDeserializer::from_bytes(bytes);
    let t = T::deserialize(&mut deserializer)?;
    if deserializer.input.is_empty() {
        Ok(t)
    } else {
        Err(SDError::InputTooLong)
    }
}

fn unsigned_bytes_to_number(bytes: &[u8]) -> u64 {
    let mut result = 0u64;
    for byte in bytes.iter() {
        result <<= 8;
        result |= *byte as u64;
    }
    result
}

impl<'de> ErdDeserializer<'de> {
    fn next_byte(&mut self) -> Result<u8> {
        if self.input.len() > 0 {
            let result = self.input[0];
            self.input = &self.input[1..];
            Ok(result)
        } else {
            Err(SDError::InputTooShort)
        }
    }

    fn next_bytes(&mut self, nr_bytes: usize) -> Result<&'de [u8]> {
        if self.input.len() >= nr_bytes {
            let result = &self.input[..nr_bytes];
            self.input = &self.input[nr_bytes..];
            Ok(result)
        } else {
            Err(SDError::InputTooShort)
        }
    }

    fn flush(&mut self) -> &'de [u8] {
        let bytes = self.input;
        self.input = &[];
        bytes
    }
}

macro_rules! impl_nums {
    ($ty:ty, $deser_method:ident, $visitor_method:ident, $num_bytes:expr) => {
        #[inline]
        fn $deser_method<V>(self, visitor: V) -> Result<V::Value>
            where V: serde::de::Visitor<'de>,
        {
            let bytes = if self.top_level {
                self.flush()
            } else {
                self.next_bytes($num_bytes)?
            };
            
            visitor.$visitor_method(unsigned_bytes_to_number(bytes) as $ty)
        }
    }
}

impl<'de> serde::Deserializer<'de> for &mut ErdDeserializer<'de> {
    type Error = SDError;

    #[inline]
    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(SDError::UnsupportedOperation)
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        let value = self.next_byte()?;
        match value {
            1 => visitor.visit_bool(true),
            0 => visitor.visit_bool(false),
            _ => Err(SDError::InvalidValue)
        }
    }

    impl_nums!(u16, deserialize_u16, visit_u16, 2);
    impl_nums!(u32, deserialize_u32, visit_u32, 4);
    impl_nums!(u64, deserialize_u64, visit_u64, 8);
    impl_nums!(i16, deserialize_i16, visit_i16, 2);
    impl_nums!(i32, deserialize_i32, visit_i32, 4);
    impl_nums!(i64, deserialize_i64, visit_i64, 8);


    #[inline]
    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        let byte = self.next_byte()?;
        visitor.visit_u8(byte)
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        let byte = self.next_byte()?;
        visitor.visit_i8(byte as i8)
    }

    #[inline]
    fn deserialize_f32<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(SDError::UnsupportedOperation)
    }

    #[inline]
    fn deserialize_f64<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(SDError::UnsupportedOperation)
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_char<V>(self, _: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(SDError::UnsupportedOperation)
    }

    fn deserialize_str<V>(self, _: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(SDError::UnsupportedOperation)
    }

    fn deserialize_string<V>(self, _: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(SDError::UnsupportedOperation)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        let bytes = if self.top_level {
            self.flush()
        } else {
            let size_bytes = self.next_bytes(USIZE_SIZE)?;
            let size = unsigned_bytes_to_number(size_bytes) as usize;
            self.next_bytes(size)?
        };
        visitor.visit_borrowed_bytes(bytes)
    }

    fn deserialize_byte_buf<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(SDError::UnsupportedOperation)
    }

    fn deserialize_enum<V>(
        self,
        _enum: &'static str,
        _variants: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        // impl<'de, 'a, R: 'a, O> serde::de::EnumAccess<'de> for &'a mut Deserializer<R, O>
        // where R: BincodeRead<'de>, O: Options {
        //     type Error = Error;
        //     type Variant = Self;

        //     fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant)>
        //         where V: serde::de::DeserializeSeed<'de>,
        //     {
        //         let idx: u32 = try!(serde::de::Deserialize::deserialize(&mut *self));
        //         let val: Result<_> = seed.deserialize(idx.into_deserializer());
        //         Ok((try!(val), self))
        //     }
        // }

        // visitor.visit_enum(self)
        Err(SDError::NotImplemented)
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        self.top_level = false;
        visitor.visit_seq(Access {
            deserializer: self,
            remaining_items: len,
        })
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        self.top_level = false;
        let value = self.next_byte()?;
        match value {
            0 => visitor.visit_none(),
            1 => visitor.visit_some(&mut *self),
            _ => Err(SDError::InvalidValue)
        }
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        let size_bytes = self.next_bytes(USIZE_SIZE)?;
        let size = unsigned_bytes_to_number(size_bytes) as usize;
        self.deserialize_tuple(size, visitor)
    }

    fn deserialize_map<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(SDError::NotImplemented)
    }

    fn deserialize_struct<V>(
        self,
        _name: &str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_tuple(fields.len(), visitor)
    }

    fn deserialize_identifier<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(SDError::UnsupportedOperation)
    }

    fn deserialize_newtype_struct<V>(self, _name: &str, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_tuple(len, visitor)
    }

    fn deserialize_ignored_any<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(SDError::UnsupportedOperation)
    }

    fn is_human_readable(&self) -> bool {
        false
    }
}

// impl<'de, 'a, R, O> serde::de::VariantAccess<'de> for &'a mut Deserializer<R, O>
// where R: BincodeRead<'de>, O: Options{
//     type Error = Error;

//     fn unit_variant(self) -> Result<()> {
//         Ok(())
//     }

//     fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value>
//         where T: serde::de::DeserializeSeed<'de>,
//     {
//         serde::de::DeserializeSeed::deserialize(seed, self)
//     }

//     fn tuple_variant<V>(self,
//                       len: usize,
//                       visitor: V) -> Result<V::Value>
//         where V: serde::de::Visitor<'de>,
//     {
//         serde::de::Deserializer::deserialize_tuple(self, len, visitor)
//     }

//     fn struct_variant<V>(self,
//                        fields: &'static [&'static str],
//                        visitor: V) -> Result<V::Value>
//         where V: serde::de::Visitor<'de>,
//     {
//         serde::de::Deserializer::deserialize_tuple(self, fields.len(), visitor)
//     }
// }

struct Access<'a, 'de: 'a> {
    deserializer: &'a mut ErdDeserializer<'de>,
    remaining_items: usize,
}

impl<'a, 'de> serde::de::SeqAccess<'de> for Access<'a, 'de> {
    type Error = SDError;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        if self.remaining_items > 0 {
            self.remaining_items -= 1;
            let value = seed.deserialize(&mut *self.deserializer)?;
            Ok(Some(value))
        } else {
            Ok(None)
        }
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.remaining_items)
    }
}

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec::Vec;
    use core::fmt::Debug;

    fn deser_ok<V>(element: V, bytes: &[u8])
    where
        V: serde::de::DeserializeOwned + PartialEq + Debug + 'static,
    {
        let deserialized: V = from_bytes(bytes).unwrap();
        assert_eq!(deserialized, element);
    }

    #[test]
    fn test_num() {
        deser_ok(5u8, &[5u8]);
    }

    #[test]
    fn test_struct() {
        #[derive(Deserialize, PartialEq, Debug)]
        struct Test {
            int: u16,
            seq: Vec<u8>,
            another_byte: u8,
        }

        let test = Test {
            int: 1,
            seq: [5, 6].to_vec(),
            another_byte: 7,
        };
        deser_ok(test, &[0, 1, 0, 0, 0, 2, 5, 6, 7]);
    }

}
