

use serde::Deserialize;
use serde::de::IntoDeserializer;
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
    fn new(input: &'de [u8]) -> Self {
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
    let mut deserializer = ErdDeserializer::new(bytes);
    let t = T::deserialize(&mut deserializer)?;
    if deserializer.input.is_empty() {
        Ok(t)
    } else {
        Err(SDError::InputTooLong)
    }
}

/// Handles both signed and unsigned of any length.
/// No generics here, because we want the executable binary as small as possible.
pub fn bytes_to_number(bytes: &[u8], signed: bool) -> u64 {
    if bytes.is_empty() {
        return 0;
    }
    let negative = signed && bytes[0] >> 7 == 1;
    let mut result = 
        if negative {
            // start with all bits set to 1, 
            // to ensure that if there are fewer bytes than the result type width,
            // the leading bits will be 1 instead of 0
            0xffffffffffffffffu64 
        } else { 
            0u64 
        };
    for byte in bytes.iter() {
        result <<= 8;
        result |= *byte as u64;
    }
    result
}

impl<'de> ErdDeserializer<'de> {
    fn next_byte(&mut self) -> Result<u8> {
        if !self.input.is_empty() {
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
    ($ty:ty, $deser_method:ident, $visitor_method:ident, $num_bytes:expr, $signed:expr) => {
        #[inline]
        fn $deser_method<V>(self, visitor: V) -> Result<V::Value>
            where V: serde::de::Visitor<'de>,
        {
            let bytes = if self.top_level {
                self.flush()
            } else {
                self.next_bytes($num_bytes)?
            };
            
            visitor.$visitor_method(bytes_to_number(bytes, $signed) as $ty)
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
        if self.top_level {
            // top level bool is either [1] or []
            if !self.input.is_empty() {
                match self.next_byte()? {
                    1 => visitor.visit_bool(true),
                    _ => Err(SDError::InvalidValue)
                }
            } else {
                visitor.visit_bool(false)
            }
        } else {
            // regular bool is either [1] or [0]
            let value = self.next_byte()?;
            match value {
                1 => visitor.visit_bool(true),
                0 => visitor.visit_bool(false),
                _ => Err(SDError::InvalidValue)
            }
        }
    }

    impl_nums!(u8 , deserialize_u8 , visit_u8 , 1, false);
    impl_nums!(u16, deserialize_u16, visit_u16, 2, false);
    impl_nums!(u32, deserialize_u32, visit_u32, 4, false);
    impl_nums!(u64, deserialize_u64, visit_u64, 8, false);


    impl_nums!(i8 , deserialize_i8 , visit_i8 , 1, true);
    impl_nums!(i16, deserialize_i16, visit_i16, 2, true);
    impl_nums!(i32, deserialize_i32, visit_i32, 4, true);
    impl_nums!(i64, deserialize_i64, visit_i64, 8, true);

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
            let size = bytes_to_number(size_bytes, false) as usize;
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
        visitor: V,
    ) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {

        visitor.visit_enum(self)
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        self.top_level = false;
        visitor.visit_seq(Access {
            deserializer: self,
            remaining_items_hint: Some(len),
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
        let remaining_items_hint = if self.top_level {
            None // we will know we ran out of items when the input runs out
        } else {
            let size_bytes = self.next_bytes(USIZE_SIZE)?;
            let size = bytes_to_number(size_bytes, false) as usize;
            Some(size)
        };

        self.top_level = false;
        visitor.visit_seq(Access {
            deserializer: self,
            remaining_items_hint,
        })
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

impl<'de, 'a> serde::de::EnumAccess<'de> for &'a mut ErdDeserializer<'de> {
    type Error = SDError;
    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant)>
        where V: serde::de::DeserializeSeed<'de>,
    {
        self.top_level = false;
        let idx: u32 = serde::de::Deserialize::deserialize(&mut *self)?;
        let val: V::Value = seed.deserialize(idx.into_deserializer())?;
        Ok((val, self))
    }
}

impl<'de, 'a> serde::de::VariantAccess<'de> for &'a mut ErdDeserializer<'de> {
    type Error = SDError;

    fn unit_variant(self) -> Result<()> {
        Ok(())
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value>
        where T: serde::de::DeserializeSeed<'de>,
    {
        serde::de::DeserializeSeed::deserialize(seed, self)
    }

    fn tuple_variant<V>(self,
                      len: usize,
                      visitor: V) -> Result<V::Value>
        where V: serde::de::Visitor<'de>,
    {
        serde::de::Deserializer::deserialize_tuple(self, len, visitor)
    }

    fn struct_variant<V>(self,
                       fields: &'static [&'static str],
                       visitor: V) -> Result<V::Value>
        where V: serde::de::Visitor<'de>,
    {
        serde::de::Deserializer::deserialize_tuple(self, fields.len(), visitor)
    }
}

struct Access<'a, 'de: 'a> {
    deserializer: &'a mut ErdDeserializer<'de>,
    remaining_items_hint: Option<usize>,
}

impl<'a, 'de> serde::de::SeqAccess<'de> for Access<'a, 'de> {
    type Error = SDError;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        match self.remaining_items_hint {
            Some(remaining_items) => {
                if remaining_items > 0 {
                    self.remaining_items_hint = Some(remaining_items - 1);
                    let value = seed.deserialize(&mut *self.deserializer)?;
                    Ok(Some(value))
                } else {
                    Ok(None)
                }
            },
            None => {
                match seed.deserialize(&mut *self.deserializer) {
                    Ok(value) => Ok(Some(value)),
                    Err(SDError::InputTooShort) => Ok(None),
                    Err(other_err) => Err(other_err),
                }
            }
        }
        
    }

    fn size_hint(&self) -> Option<usize> {
        self.remaining_items_hint
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
    fn test_top_numbers_decompacted() {
        // unsigned positive
        deser_ok(5u8, &[5]);
        deser_ok(5u16, &[5]);
        deser_ok(5u32, &[5]);
        deser_ok(5u64, &[5]);
        deser_ok(5usize, &[5]);
        // signed positive
        deser_ok(5i8, &[5]);
        deser_ok(5i16, &[5]);
        deser_ok(5i32, &[5]);
        deser_ok(5i64, &[5]);
        deser_ok(5isize, &[5]);
        // signed negative
        deser_ok(-5i8, &[251]);
        deser_ok(-5i16, &[251]);
        deser_ok(-5i32, &[251]);
        deser_ok(-5i64, &[251]);
        deser_ok(-5isize, &[251]);
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

    #[test]
    fn test_enum() {
        #[derive(Deserialize, Hash, Eq, PartialEq, Clone, Debug)]
        enum E {
            Unit,
            Newtype(u32),
            Tuple(u32, u32),
            Struct { a: u32 },
        }

        let u = E::Unit;
        let expected: &[u8] = &[/*variant index*/ 0, 0, 0, 0];
        deser_ok(u, expected);

        let n = E::Newtype(1);
        let expected: &[u8] = &[/*variant index*/ 0, 0, 0, 1, /*data*/ 0, 0, 0, 1];
        deser_ok(n, expected);

        let t = E::Tuple(1, 2);
        let expected: &[u8] = &[/*variant index*/ 0, 0, 0, 2, /*(*/ 0, 0, 0, 1, /*,*/ 0, 0, 0, 2 /*)*/];
        deser_ok(t, expected);

        let s = E::Struct { a: 1 };
        let expected: &[u8] = &[/*variant index*/ 0, 0, 0, 3, /*data*/ 0, 0, 0, 1];
        deser_ok(s, expected);
    }
}
