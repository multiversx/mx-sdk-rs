use serde::{ser, Serialize};

use super::bytes_err::{SDError, Result};
use alloc::vec::Vec;

pub struct ErdSerializer {
    output: Vec<u8>,
    top_level: bool,
}

// By convention, the public API of a Serde serializer is one or more `to_abc`
// functions such as `to_string`, `to_bytes`, or `to_writer` depending on what
// Rust types the serializer is able to produce as output.
//
// This basic serializer supports only `to_string`.
pub fn to_bytes<T>(value: T) -> Result<Vec<u8>>
where
    T: Serialize,
{
    let mut serializer = ErdSerializer {
        output: Vec::new(),
        top_level: true,
    };
    value.serialize(&mut serializer)?;
    Ok(serializer.output)
}

/// Temporary solution to serialize u64 until wasm compilation issue is fixed.
pub fn u64_to_bytes(v: u64) -> Vec<u8> {
    let mut serializer = ErdSerializer {
        output: Vec::new(),
        top_level: true,
    };
    serializer.push_number(v, 64, false, true);
    serializer.output
}

impl ErdSerializer {
    fn push_byte(&mut self, value: u8) {
        self.output.push(value);
    }

    /// Adds number to output buffer.
    /// No generics here, because we want the executable binary as small as possible.
    fn push_number(&mut self, x: u64, size_in_bits: usize, signed: bool, mut compact: bool) {
        let negative = 
            compact && // only relevant when compact flag
            signed &&  // only possible when signed flag
            x >> (size_in_bits - 1) & 1 == 1; // compute by checking first bit
        
        let irrelevant_byte = if negative { 0xffu8 } else { 0x00u8 };
        let mut bit_offset = size_in_bits as isize - 8;
        while bit_offset >= 0 {
            // going byte by byte from most to least significant
            let byte = (x >> (bit_offset as usize) & 0xffu64) as u8;
            
            if compact {
                // compact means ignoring irrelvant leading bytes
                // that is 000... for positives and fff... for negatives
                if byte != irrelevant_byte {
                    self.output.push(byte);
                    compact = false;
                }
            } else {
                self.output.push(byte);
            }
            
            bit_offset -= 8;
        }
    }

    #[inline]
    fn push_entity_size(&mut self, size: u32) {
        self.push_number(size as u64, 32, false, false);
    }

    #[inline]
    fn push_variant_index(&mut self, variant_index: u32) {
        self.push_number(variant_index as u64, 32, false, false);
    }
}

impl<'a> ser::Serializer for &'a mut ErdSerializer {
    // The output type produced by this `Serializer` during successful
    // serialization. Most serializers that produce text or binary output should
    // set `Ok = ()` and serialize into an `io::Write` or buffer contained
    // within the `Serializer` instance, as happens here. Serializers that build
    // in-memory data structures may be simplified by using `Ok` to propagate
    // the data structure around.
    type Ok = ();

    // The error type when some error occurs during serialization.
    type Error = SDError;

    // Associated types for keeping track of additional state while serializing
    // compound data structures like sequences and maps. In this case no
    // additional state is required beyond what is already stored in the
    // Serializer struct.
    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    // Here we go with the simple methods. The following 12 methods receive one
    // of the primitive types of the data model and map it to JSON by appending
    // into the output string.
    fn serialize_bool(self, v: bool) -> Result<()> {
        let byte = if v { 1u8 } else { 0u8 };
        self.push_number(byte as u64, 8, false, self.top_level);
        Ok(())
    }

    fn serialize_i8(self, v: i8) -> Result<()> {
        self.push_number(v as u64, 8, true, self.top_level);
        Ok(())
    }

    fn serialize_i16(self, v: i16) -> Result<()> {
        self.push_number(v as u64, 16, true, self.top_level);
        Ok(())
    }

    fn serialize_i32(self, v: i32) -> Result<()> {
        self.push_number(v as u64, 32, true, self.top_level);
        Ok(())
    }

    fn serialize_i64(self, v: i64) -> Result<()> {
        self.push_number(v as u64, 64, true, self.top_level);
        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<()> {
        self.push_number(v as u64, 8, false, self.top_level);
        Ok(())
    }

    fn serialize_u16(self, v: u16) -> Result<()> {
        self.push_number(v as u64, 16, false, self.top_level);
        Ok(())
    }

    fn serialize_u32(self, v: u32) -> Result<()> {
        self.push_number(v as u64, 32, false, self.top_level);
        Ok(())
    }

    fn serialize_u64(self, v: u64) -> Result<()> {
        self.push_number(v, 64, false, self.top_level);
        Ok(())
    }

    fn serialize_f32(self, _: f32) -> Result<()> {
        Err(SDError::UnsupportedOperation)
    }

    fn serialize_f64(self, _: f64) -> Result<()> {
        Err(SDError::UnsupportedOperation)
    }

    // Serialize a char as a single-character string. Other formats may
    // represent this differently.
    fn serialize_char(self, _v: char) -> Result<()> {
        Err(SDError::NotImplemented)
    }

    // This only works for strings that don't require escape sequences but you
    // get the idea. For example it would emit invalid JSON if the input string
    // contains a '"' character.
    fn serialize_str(self, _v: &str) -> Result<()> {
        Err(SDError::NotImplemented)
    }

    // Serialize a byte array as an array of bytes. Could also use a base64
    // string here. Binary formats will typically represent byte arrays more
    // compactly.
    fn serialize_bytes(self, v: &[u8]) -> Result<()> {
        if !self.top_level {
            // only save bytes length when bytes are embedded in another structure
            // when they are the top level, the number of bytes is "encoded" in the length of the output
            self.push_entity_size(v.len() as u32);
        }
        self.output.extend_from_slice(v);
        Ok(())
    }

    fn serialize_none(self) -> Result<()> {
        self.push_byte(0u8); // one byte of 0 indicates that nothing comes after
        Ok(())
    }

    fn serialize_some<T>(self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.top_level = false;
        self.push_byte(1u8); // one byte of 1 indicates that something comes after
        value.serialize(self)
    }

    // Nothing to save.
    fn serialize_unit(self) -> Result<()> {
        Ok(())
    }

    // Unit struct means a named value containing no data. Again, since there is
    // no data, map this to JSON as `null`. There is no need to serialize the
    // name in most formats.
    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        self.serialize_unit()
    }

    // When serializing a unit variant (or any other kind of variant), formats
    // can choose whether to keep track of it by index or by name. Binary
    // formats typically use the index of the variant and human-readable formats
    // typically use the name.
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
    ) -> Result<()> {
        self.push_variant_index(variant_index);
        Ok(())
    }

    // As is done here, serializers are encouraged to treat newtype structs as
    // insignificant wrappers around the data they contain.
    fn serialize_newtype_struct<T>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    // Note that newtype variant (and all of the other variant serialization
    // methods) refer exclusively to the "externally tagged" enum
    // representation.
    //
    // Serialize this to JSON in externally tagged form as `{ NAME: VALUE }`.
    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        value: &T,
    ) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.top_level = false;
        self.push_variant_index(variant_index);
        value.serialize(&mut *self)?;
        Ok(())
    }

    // Now we get to the serialization of compound types.
    //
    // The start of the sequence, each value, and the end are three separate
    // method calls. This one is responsible only for serializing the start,
    // which in JSON is `[`.
    //
    // The length of the sequence may or may not be known ahead of time. This
    // doesn't make a difference in JSON because the length is not represented
    // explicitly in the serialized form. Some serializers may only be able to
    // support sequences for which the length is known up front.
    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
        match len {
            Some(l) => {
                if !self.top_level {
                    // again, if it is top level, we can infer the size from the size of the serialized bytes
                    self.push_entity_size(l as u32);
                }
                self.top_level = false;
                Ok(self)
            }
            None => Err(SDError::SequenceLengthRequired)
        }
    }

    // Tuples look just like sequences in JSON. Some formats may be able to
    // represent tuples more efficiently by omitting the length, since tuple
    // means that the corresponding `Deserialize implementation will know the
    // length without needing to look at the serialized data.
    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        self.top_level = false;
        Ok(self)
    }

    // Tuple structs look just like sequences in JSON.
    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        self.serialize_seq(Some(len))
    }

    // Tuple variants are represented in JSON as `{ NAME: [DATA...] }`. Again
    // this method is only responsible for the externally tagged representation.
    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        self.top_level = false;
        self.push_variant_index(variant_index);
        Ok(self)
    }

    // Maps are represented in JSON as `{ K: V, K: V, ... }`.
    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        // self.output += "{";
        // Ok(self)
        Err(SDError::NotImplemented)
    }

    // Structs look just like maps in JSON. In particular, JSON requires that we
    // serialize the field names of the struct. Other formats may be able to
    // omit the field names when serializing structs because the corresponding
    // Deserialize implementation is required to know what the keys are without
    // looking at the serialized data.
    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct> {
        self.top_level = false;
        Ok(self)
    }

    // Struct variants are represented in JSON as `{ NAME: { K: V, ... } }`.
    // This is the externally tagged representation.
    fn serialize_struct_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        self.top_level = false;
        self.push_variant_index(variant_index);
        Ok(self)
    }
}

// The following 7 impls deal with the serialization of compound types like
// sequences and maps. Serialization of such types is begun by a Serializer
// method and followed by zero or more calls to serialize individual elements of
// the compound type and one call to end the compound type.
//
// This impl is SerializeSeq so these methods are called after `serialize_seq`
// is called on the Serializer.
impl<'a> ser::SerializeSeq for &'a mut ErdSerializer {
    // Must match the `Ok` type of the serializer.
    type Ok = ();
    // Must match the `Error` type of the serializer.
    type Error = SDError;

    // Serialize a single element of the sequence.
    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    // Close the sequence.
    fn end(self) -> Result<()> {
        Ok(())
    }
}

// Same thing but for tuples.
impl<'a> ser::SerializeTuple for &'a mut ErdSerializer {
    type Ok = ();
    type Error = SDError;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

// Same thing but for tuple structs.
impl<'a> ser::SerializeTupleStruct for &'a mut ErdSerializer {
    type Ok = ();
    type Error = SDError;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

// Tuple variants are a little different. Refer back to the
// `serialize_tuple_variant` method above:
//
//    self.output += "{";
//    variant.serialize(&mut *self)?;
//    self.output += ":[";
//
// So the `end` method in this impl is responsible for closing both the `]` and
// the `}`.
impl<'a> ser::SerializeTupleVariant for &'a mut ErdSerializer {
    type Ok = ();
    type Error = SDError;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

// Some `Serialize` types are not able to hold a key and value in memory at the
// same time so `SerializeMap` implementations are required to support
// `serialize_key` and `serialize_value` individually.
//
// There is a third optional method on the `SerializeMap` trait. The
// `serialize_entry` method allows serializers to optimize for the case where
// key and value are both available simultaneously. In JSON it doesn't make a
// difference so the default behavior for `serialize_entry` is fine.
impl<'a> ser::SerializeMap for &'a mut ErdSerializer {
    type Ok = ();
    type Error = SDError;

    // The Serde data model allows map keys to be any serializable type. JSON
    // only allows string keys so the implementation below will produce invalid
    // JSON if the key serializes as something other than a string.
    //
    // A real JSON serializer would need to validate that map keys are strings.
    // This can be done by using a different Serializer to serialize the key
    // (instead of `&mut **self`) and having that other serializer only
    // implement `serialize_str` and return an error on any other data type.
    fn serialize_key<T>(&mut self, key: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        key.serialize(&mut **self)
    }

    // It doesn't make a difference whether the colon is printed at the end of
    // `serialize_key` or at the beginning of `serialize_value`. In this case
    // the code is a bit simpler having it here.
    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

// Structs are like maps in which the keys are constrained to be compile-time
// constant strings.
impl<'a> ser::SerializeStruct for &'a mut ErdSerializer {
    type Ok = ();
    type Error = SDError;

    fn serialize_field<T>(&mut self, _key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

// Similar to `SerializeTupleVariant`, here the `end` method is responsible for
// closing both of the curly braces opened by `serialize_struct_variant`.
impl<'a> ser::SerializeStructVariant for &'a mut ErdSerializer {
    type Ok = ();
    type Error = SDError;

    fn serialize_field<T>(&mut self, _key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use core::fmt::Debug;

    fn ser_ok<V>(element: V, bytes: &[u8])
    where
        V: Serialize + PartialEq + Debug + 'static,
    {
        assert_eq!(to_bytes(&element).unwrap().as_slice(), bytes);
    }

    #[test]
    fn test_top_compacted_numbers() {
        // unsigned positive
        ser_ok(5u8, &[5]);
        ser_ok(5u16, &[5]);
        ser_ok(5u32, &[5]);
        ser_ok(5u64, &[5]);
        ser_ok(5usize, &[5]);
        // signed positive
        ser_ok(5i8, &[5]);
        ser_ok(5i16, &[5]);
        ser_ok(5i32, &[5]);
        ser_ok(5i64, &[5]);
        ser_ok(5isize, &[5]);
        // signed negative
        ser_ok(-5i8, &[251]);
        ser_ok(-5i16, &[251]);
        ser_ok(-5i32, &[251]);
        ser_ok(-5i64, &[251]);
        ser_ok(-5isize, &[251]);
    }

    #[test]
    fn test_top_compacted_bool() {
        ser_ok(true,    &[1]);
        ser_ok(false,   &[]);
        ser_ok(&true,   &[1]);
        ser_ok(&false,  &[]);
    }

    #[test]
    fn test_top_compacted_empty_bytes() {
        let empty_byte_slice: &[u8] = &[];
        ser_ok(empty_byte_slice, empty_byte_slice);
    }

    #[test]
    fn test_top_compacted_bytes() {
        ser_ok(&[1u8, 2u8, 3u8], &[1u8, 2u8, 3u8]);
    }

    #[test]
    fn test_top_compacted_vec_u8() {
        let some_vec = [1u8, 2u8, 3u8].to_vec();
        ser_ok(some_vec, &[1u8, 2u8, 3u8]);
    }

    #[test]
    fn test_top_compacted_vec_i32() {
        let some_vec = [1i32, 2i32, 3i32].to_vec();
        let expected: &[u8] = &[0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3];
        ser_ok(some_vec, expected);
    }

    #[test]
    fn test_struct() {
        #[derive(Serialize, PartialEq, Debug)]
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

        ser_ok(test, &[0, 1, 0, 0, 0, 2, 5, 6, 7]);
    }

    #[test]
    fn test_tuple() {
        ser_ok((7u32, -2i16), &[0, 0, 0, 7, 255, 254]);
    }

    #[test]
    fn test_unit() {
        ser_ok((), &[]);
    }

    #[test]
    fn test_enum() {
        #[derive(Serialize, Hash, Eq, PartialEq, Clone, Debug)]
        enum E {
            Unit,
            Newtype(u32),
            Tuple(u32, u32),
            Struct { a: u32 },
        }

        let u = E::Unit;
        let expected: &[u8] = &[/*variant index*/ 0, 0, 0, 0];
        ser_ok(u, expected);

        let n = E::Newtype(1);
        let expected: &[u8] = &[/*variant index*/ 0, 0, 0, 1, /*data*/ 0, 0, 0, 1];
        ser_ok(n, expected);

        let t = E::Tuple(1, 2);
        let expected: &[u8] = &[/*variant index*/ 0, 0, 0, 2, /*(*/ 0, 0, 0, 1, /*,*/ 0, 0, 0, 2 /*)*/];
        ser_ok(t, expected);

        let s = E::Struct { a: 1 };
        let expected: &[u8] = &[/*variant index*/ 0, 0, 0, 3, /*data*/ 0, 0, 0, 1];
        ser_ok(s, expected);
    }
}
