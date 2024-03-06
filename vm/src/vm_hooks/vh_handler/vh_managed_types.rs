mod vh_big_float;
mod vh_big_int;
mod vh_managed_buffer;
mod vh_managed_map;

use gob::{ser::TypeId, Buf, StreamDeserializer, StreamSerializer};
use num_traits::{float, Signed, ToBytes};
use serde::ser::Serializer;
pub use vh_big_float::VMHooksBigFloat;
pub use vh_big_int::VMHooksBigInt;
pub use vh_managed_buffer::VMHooksManagedBuffer;
pub use vh_managed_map::VMHooksManagedMap;

use std::fmt::Debug;

use crate::types::RawHandle;

use super::VMHooksError;

const FLOAT_GOB_VERSION: u8 = 1;
const GOB_DEFAULT_PREC: u64 = 53; // can also be 24
const _W: u64 = 64;
const _S: u64 = _W / 8;
// Number of bits in a mantissa word
// 64-bit precision for IEEE-754

#[allow(dead_code)]
pub enum Accuracy {
    Below,
    Exact,
    Above,
}

pub enum Form {
    Zero,
    Finite,
    Infinite,
}

#[allow(dead_code)]
pub enum Mode {
    ToNearestEven, // == IEEE 754-2008 roundTiesToEven  // default
    ToNearestAway, // == IEEE 754-2008 roundTiesToAway
    ToZero,        // == IEEE 754-2008 roundTowardZero
    AwayFromZero,  // no IEEE 754-2008 equivalent
    ToNegativeInf, // == IEEE 754-2008 roundTowardNegative
    ToPositiveInf, // == IEEE 754-2008 roundTowardPositive
}

/// Provides VM hook implementations for methods that deal with more than one type of managed type.
///
/// It is also the trait that unifies all managed type functionality.
pub trait VMHooksManagedTypes:
    VMHooksBigInt + VMHooksManagedBuffer + VMHooksManagedMap + VMHooksBigFloat + VMHooksError + Debug
{
    fn mb_to_big_int_unsigned(&self, buffer_handle: RawHandle, bi_handle: RawHandle) {
        let bytes = self.m_types_lock().mb_to_bytes(buffer_handle);
        self.m_types_lock()
            .bi_set_unsigned_bytes(bi_handle, bytes.as_slice());
    }

    fn mb_to_big_int_signed(&self, buffer_handle: RawHandle, bi_handle: RawHandle) {
        let bytes = self.m_types_lock().mb_to_bytes(buffer_handle);
        self.m_types_lock()
            .bi_set_signed_bytes(bi_handle, bytes.as_slice());
    }

    fn mb_from_big_int_unsigned(&self, buffer_handle: RawHandle, bi_handle: RawHandle) {
        let bi_bytes = self.m_types_lock().bi_get_unsigned_bytes(bi_handle);
        self.m_types_lock().mb_set(buffer_handle, bi_bytes);
    }

    fn mb_from_big_int_signed(&self, buffer_handle: RawHandle, bi_handle: RawHandle) {
        let bi_bytes = self.m_types_lock().bi_get_signed_bytes(bi_handle);
        self.m_types_lock().mb_set(buffer_handle, bi_bytes);
    }

    fn bi_to_string(&self, bi_handle: RawHandle, str_handle: RawHandle) {
        let bi = self.m_types_lock().bi_get(bi_handle);
        let s = bi.to_string();
        self.m_types_lock().mb_set(str_handle, s.into_bytes());
    }

    fn mb_set_random(&self, dest_handle: RawHandle, length: usize) {
        let bytes = self.random_next_bytes(length);
        self.mb_set(dest_handle, bytes.as_slice());
    }

    // transforms big float into bytes with respect to IEEE-754 gob encode
    fn mb_from_big_float(&self, m_buffer_handle: RawHandle, big_float_handle: RawHandle) {
        let big_float = self.m_types_lock().bf_get_f64(big_float_handle);
        let bytes = gob_encode(
            big_float,
            Accuracy::Exact,
            Mode::ToNearestEven,
            GOB_DEFAULT_PREC,
        );
        println!("encoded bytes {:#?}", bytes);
        self.m_types_lock().mb_set(m_buffer_handle, bytes.to_vec());
    }

    // fn mb_from_big_float(&self, m_buffer_handle: RawHandle, big_float_handle: RawHandle) {
    //     let big_float = self.m_types_lock().bf_get_f64(big_float_handle);
    //     println!("big float is {:#?}", big_float.to_bits());

    //     let mut stream_ser = StreamSerializer::new_with_buffer();

    //     // serialize the f64 value using the obtained Serializer
    //     let _ = stream_ser.serialize_with_type_id(TypeId::FLOAT, &big_float);

    //     // extract the serialized data
    //     let output_buffer = stream_ser.into_inner();
    //     let bytes = output_buffer.collect();
    //     println!("output buffer is {:#?}", bytes);

    //     self.m_types_lock().mb_set(m_buffer_handle, bytes);
    // }
}
fn gob_encode(f: f64, accuracy: Accuracy, mode: Mode, precision: u64) -> Vec<u8> {
    let bits = f.to_bits();
    let (_sign, exp, mut mant) = extract_float_components(bits);

    let mut _n = 0;
    let mut _form = Form::Zero;
    if f.is_finite() {
        let mant_number_of_bits = num_bits(mant) as u64;
        _form = Form::Finite;
        _n = (GOB_DEFAULT_PREC + (_W - 1)) / _W;
        if mant_number_of_bits < _n {
            _n = mant_number_of_bits;
        }
    } else {
        _form = Form::Infinite;
    }

    let mut buf = vec![];
    buf.push(FLOAT_GOB_VERSION);

    let mode_bits = mode as u8 & 0b111;
    let acc_bits = (accuracy as u8 + 1) & 0b11;
    let form_bits = _form as u8 & 0b11;

    let mut combined_byte = (mode_bits << 5) | (acc_bits << 3) | (form_bits << 1);
    if f.is_sign_negative() {
        combined_byte |= 1;
    }

    buf.push(combined_byte);
    buf.extend_from_slice(&precision.to_be_bytes());

    if f.is_finite() {
        buf.extend_from_slice(&exp.to_be_bytes());
        mant >>= 64 - _n;
        buf.extend_from_slice(&mant.to_be_bytes())
    }

    buf
}

fn extract_float_components(bits: u64) -> (u64, i64, u64) {
    // Define masks for sign, exponent, and mantissa
    let sign_mask: u64 = 0x8000000000000000;
    let exponent_mask: u64 = 0x7FF0000000000000;
    let mantissa_mask: u64 = 0x000FFFFFFFFFFFFF;

    // Extract sign
    let sign = (bits & sign_mask) >> 63;

    // Extract exponent
    let mut exponent = ((bits & exponent_mask) >> 52) as i64;
    // Adjust for bias
    exponent -= 1023;

    // Extract mantissa
    let mut mantissa = bits & mantissa_mask;

    // Add implicit leading bit
    mantissa |= 0x0010000000000000;

    (sign, exponent, mantissa)
}

fn num_bits(mut n: u64) -> usize {
    if n == 0 {
        return 1; // Special case for 0
    }

    let mut bits = 0;
    while n > 0 {
        bits += 1;
        n >>= 1; // Shift right to check the next bit
    }
    bits
}
